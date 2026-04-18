use async_trait::async_trait;
use serde::Deserialize;

use crate::translation::{build_prompt, TranslationService};

/// Claude via Anthropic API
pub struct ClaudeApiService {
    api_key: String,
    client: reqwest::Client,
    model: String,
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
    pub fn new(client: reqwest::Client, model: Option<&str>) -> Result<Self, String> {
        let api_key = std::env::var("ANTHROPIC_API_KEY")
            .map_err(|_| "ANTHROPIC_API_KEY not set. Add it to .env or export it.".to_string())?;
        let model = model.unwrap_or("claude-sonnet-4-20250514").to_string();
        Ok(Self { api_key, client, model })
    }
}

#[async_trait]
impl TranslationService for ClaudeApiService {
    async fn translate(&self, text: &str, from: &str, to: &str, custom_prompt: Option<&str>) -> Result<String, String> {
        self.prompt(&build_prompt(text, from, to, custom_prompt)).await
    }

    async fn prompt(&self, prompt: &str) -> Result<String, String> {
        let body = serde_json::json!({
            "model": &self.model,
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
pub struct ClaudeCliService {
    claude_bin: String,
    model: Option<String>,
}

impl ClaudeCliService {
    pub fn new(model: Option<&str>) -> Result<Self, String> {
        let claude_bin = Self::find_claude_binary()
            .ok_or_else(|| "claude CLI not found. Install it first.".to_string())?;
        // Verify the CLI is available
        std::process::Command::new(&claude_bin)
            .arg("--version")
            .output()
            .map_err(|_| format!("claude CLI found at {} but failed to run.", claude_bin))?;
        Ok(Self {
            claude_bin,
            model: model.map(|m| m.to_string()),
        })
    }

    /// Resolve the claude binary path.
    /// GUI apps on macOS don't inherit the shell PATH, so we check common locations.
    fn find_claude_binary() -> Option<String> {
        // Try bare command first (works in terminals)
        if std::process::Command::new("claude")
            .arg("--version")
            .output()
            .is_ok()
        {
            return Some("claude".to_string());
        }

        // Common install locations on macOS/Linux
        let home = std::env::var("HOME").unwrap_or_default();
        let candidates = [
            format!("{}/.local/bin/claude", home),
            format!("{}/.claude/local/claude", home),
            "/usr/local/bin/claude".to_string(),
            "/opt/homebrew/bin/claude".to_string(),
        ];

        for path in &candidates {
            if std::path::Path::new(path).is_file() {
                return Some(path.clone());
            }
        }

        None
    }
}

#[async_trait]
impl TranslationService for ClaudeCliService {
    async fn translate(&self, text: &str, from: &str, to: &str, custom_prompt: Option<&str>) -> Result<String, String> {
        self.prompt(&build_prompt(text, from, to, custom_prompt)).await
    }

    async fn prompt(&self, prompt: &str) -> Result<String, String> {
        let prompt = prompt.to_string();
        let model = self.model.clone();
        let claude_bin = self.claude_bin.clone();
        tokio::task::spawn_blocking(move || {
            let mut cmd = std::process::Command::new(&claude_bin);
            cmd.arg("-p").arg(&prompt);
            if let Some(ref model) = model {
                cmd.arg("--model").arg(model);
            }
            let output = cmd
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
