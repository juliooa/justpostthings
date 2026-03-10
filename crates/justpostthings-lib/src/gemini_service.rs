use async_trait::async_trait;
use serde::Deserialize;

use crate::translation::{build_prompt, TranslationService};

pub struct GeminiService {
    api_key: String,
    client: reqwest::Client,
    model: String,
}

#[derive(Deserialize)]
struct GeminiResponse {
    candidates: Vec<Candidate>,
}

#[derive(Deserialize)]
struct Candidate {
    content: Content,
}

#[derive(Deserialize)]
struct Content {
    parts: Vec<Part>,
}

#[derive(Deserialize)]
struct Part {
    text: String,
}

impl GeminiService {
    pub fn new(client: reqwest::Client, model: Option<&str>) -> Result<Self, String> {
        let api_key = std::env::var("GEMINI_API_KEY")
            .map_err(|_| "GEMINI_API_KEY not set. Add it to .env or export it.".to_string())?;
        let model = model.unwrap_or("gemini-2.0-flash").to_string();
        Ok(Self { api_key, client, model })
    }
}

#[async_trait]
impl TranslationService for GeminiService {
    async fn translate(&self, text: &str, from: &str, to: &str) -> Result<String, String> {
        self.prompt(&build_prompt(text, from, to)).await
    }

    async fn prompt(&self, prompt: &str) -> Result<String, String> {
        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
            self.model, self.api_key
        );

        let body = serde_json::json!({
            "contents": [{
                "parts": [{ "text": prompt }]
            }]
        });

        let res = self
            .client
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("Gemini request failed: {}", e))?;

        let status = res.status();
        let response_text = res.text().await.map_err(|e| format!("Failed to read Gemini response: {}", e))?;

        if !status.is_success() {
            return Err(format!("Gemini API error (HTTP {}): {}", status, response_text));
        }

        let parsed: GeminiResponse = serde_json::from_str(&response_text)
            .map_err(|e| format!("Failed to parse Gemini response: {} — {}", e, response_text))?;

        parsed
            .candidates
            .first()
            .and_then(|c| c.content.parts.first())
            .map(|p| p.text.trim().to_string())
            .ok_or_else(|| "Gemini returned no content".to_string())
    }
}
