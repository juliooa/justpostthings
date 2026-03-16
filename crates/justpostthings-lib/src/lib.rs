pub mod buffer;
pub mod claude_service;
pub mod gemini_service;
pub mod imgbb;
pub mod openai_service;
pub mod translation;

use serde::{Deserialize, Serialize};
use std::fs;

fn default_true() -> bool {
    true
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct LlmService {
    pub provider: String,
    pub model: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Config {
    pub channels: Vec<Channel>,
    pub default_post_channels: Vec<String>,
    pub llm_service: Option<LlmService>,
    #[serde(default = "default_true")]
    pub save_sent_posts: bool,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct TranslateConfig {
    pub from: String,
    pub to: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Channel {
    pub name: String,
    pub id: String,
    #[serde(default)]
    pub should_translate: bool,
    pub translate: Option<TranslateConfig>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PostRequest {
    pub text: String,
    pub images: Vec<String>,
    pub schedule: Option<String>,
    pub channel_names: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ChannelPostResult {
    pub channel: String,
    pub success: bool,
    pub message: String,
    pub post_id: Option<String>,
}

pub fn load_config(path: &str) -> Result<Config, String> {
    let content = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read config file '{}': {}", path, e))?;
    serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse config file: {}", e))
}
