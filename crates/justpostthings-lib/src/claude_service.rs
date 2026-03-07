use async_trait::async_trait;
use serde::Deserialize;

use crate::translation::{build_prompt, TranslationService};

/// Claude via Anthropic API
pub struct ClaudeApiService {
    api_key: String,
    client: reqwest::Client,
}

#[derive(Deserialize)]
struct MessagesResponse {
    content: Vec<ContentBlock>,
}

#[derive(Deserialize)]
struct ContentBlock {
    text: Option<String>,
}

impl ClaudeApiService {
    pub fn new(client: reqwest::Client) -> Result<Self, String> {
        let api_key = std::env::var("ANTHROPIC_API_KEY")
            .map_err(|_| "ANTHROPIC_API_KEY not set. Add it to .env or export it.".to_string())?;
        Ok(Self { api_key, client })
    }
}

#[async_trait]
impl TranslationService for ClaudeApiService {
    async fn translate(&self, text: &str, from: &str, to: &str) -> Result<String, String> {
        self.prompt(&build_prompt(text, from, to)).await
    }

    async fn prompt(&self, prompt: &str) -> Result<String, String> {
        let body = serde_json::json!({
            "model": "claude-sonnet-4-20250514",
            "max_tokens": 1024,
            "messages": [
                { "role": "user", "content": prompt }
            ]
        });

        let res = self
            .client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("Claude API request failed: {}", e))?;

        let status = res.status();
        let response_text = res.text().await.map_err(|e| format!("Failed to read Claude response: {}", e))?;

        if !status.is_success() {
            return Err(format!("Claude API error (HTTP {}): {}", status, response_text));
        }

        let parsed: MessagesResponse = serde_json::from_str(&response_text)
            .map_err(|e| format!("Failed to parse Claude response: {} — {}", e, response_text))?;

        parsed
            .content
            .first()
            .and_then(|b| b.text.as_ref())
            .map(|t| t.trim().to_string())
            .ok_or_else(|| "Claude returned no content".to_string())
    }
}

/// Claude via CLI (`claude -p 'prompt'`)
pub struct ClaudeCliService;

impl ClaudeCliService {
    pub fn new() -> Result<Self, String> {
        // Verify the CLI is available
        std::process::Command::new("claude")
            .arg("--version")
            .output()
            .map_err(|_| "claude CLI not found. Install it first.".to_string())?;
        Ok(Self)
    }
}

#[async_trait]
impl TranslationService for ClaudeCliService {
    async fn translate(&self, text: &str, from: &str, to: &str) -> Result<String, String> {
        self.prompt(&build_prompt(text, from, to)).await
    }

    async fn prompt(&self, prompt: &str) -> Result<String, String> {
        let prompt = prompt.to_string();
        tokio::task::spawn_blocking(move || {
            let output = std::process::Command::new("claude")
                .arg("-p")
                .arg(&prompt)
                .output()
                .map_err(|e| format!("Failed to run claude CLI: {}", e))?;

            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                return Err(format!("claude CLI error: {}", stderr));
            }

            let stdout = String::from_utf8_lossy(&output.stdout);
            Ok(stdout.trim().to_string())
        })
        .await
        .map_err(|e| format!("Task join error: {}", e))?
    }
}
