use async_trait::async_trait;
use serde::Deserialize;

use crate::translation::{build_prompt, TranslationService};

pub struct OpenAIService {
    api_key: String,
    client: reqwest::Client,
    model: String,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: Message,
}

#[derive(Deserialize)]
struct Message {
    content: String,
}

impl OpenAIService {
    pub fn new(client: reqwest::Client, model: Option<&str>) -> Result<Self, String> {
        let api_key = std::env::var("OPENAI_API_KEY")
            .map_err(|_| "OPENAI_API_KEY not set. Add it to .env or export it.".to_string())?;
        let model = model.unwrap_or("gpt-4o-mini").to_string();
        Ok(Self { api_key, client, model })
    }
}

#[async_trait]
impl TranslationService for OpenAIService {
    async fn translate(&self, text: &str, from: &str, to: &str, custom_prompt: Option<&str>) -> Result<String, String> {
        self.prompt(&build_prompt(text, from, to, custom_prompt)).await
    }

    async fn prompt(&self, prompt: &str) -> Result<String, String> {
        let body = serde_json::json!({
            "model": &self.model,
            "messages": [
                { "role": "user", "content": prompt }
            ]
        });

        let res = self
            .client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("OpenAI request failed: {}", e))?;

        let status = res.status();
        let response_text = res.text().await.map_err(|e| format!("Failed to read OpenAI response: {}", e))?;

        if !status.is_success() {
            return Err(format!("OpenAI API error (HTTP {}): {}", status, response_text));
        }

        let parsed: ChatResponse = serde_json::from_str(&response_text)
            .map_err(|e| format!("Failed to parse OpenAI response: {} — {}", e, response_text))?;

        parsed
            .choices
            .first()
            .map(|c| c.message.content.trim().to_string())
            .ok_or_else(|| "OpenAI returned no choices".to_string())
    }
}
