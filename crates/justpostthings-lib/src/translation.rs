use async_trait::async_trait;

use crate::claude_service::{ClaudeApiService, ClaudeCliService};
use crate::gemini_service::GeminiService;
use crate::openai_service::OpenAIService;

#[async_trait]
pub trait TranslationService {
    async fn translate(&self, text: &str, from: &str, to: &str) -> Result<String, String>;
    async fn prompt(&self, prompt: &str) -> Result<String, String>;
}

pub fn build_prompt(text: &str, from: &str, to: &str) -> String {
    format!(
        "Translate the following text from {from} to {to}. \
         Reply with ONLY the translated text, nothing else. \
         No explanations, no quotes, no extra words. \
         Keep the same tone and meaning. \
         Preserve all line breaks and paragraph spacing exactly as in the original. \
         If something from {from} sounds weird in {to}, find a natural way to say it.\n\n\
         {text}"
    )
}

pub fn build_shrink_prompt(text: &str, max_chars: usize) -> String {
    format!(
        "Shorten the following text to fit within {max_chars} characters. \
         Reply with ONLY the shortened text, nothing else. \
         No explanations, no quotes, no extra words. \
         Keep the most important information and the same tone. \
         Preserve line breaks where possible. \
         The result MUST be {max_chars} characters or fewer.\n\n\
         {text}"
    )
}

pub fn create_service(
    name: &str,
    model: Option<&str>,
    client: reqwest::Client,
) -> Result<Box<dyn TranslationService + Send + Sync>, String> {
    match name {
        "openai" => Ok(Box::new(OpenAIService::new(client, model)?)),
        "gemini" => Ok(Box::new(GeminiService::new(client, model)?)),
        "claude-api" => Ok(Box::new(ClaudeApiService::new(client, model)?)),
        "claude-cli" => Ok(Box::new(ClaudeCliService::new(model)?)),
        _ => Err(format!(
            "Unknown LLM service: '{}'. Use 'openai', 'gemini', 'claude-api', or 'claude-cli'.",
            name
        )),
    }
}
