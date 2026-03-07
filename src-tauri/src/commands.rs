use justpostthings_lib::{self as lib, buffer, imgbb, translation};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct TranslationResult {
    pub channel: String,
    pub translated_text: String,
}

/// Find the project root by looking for config.json in cwd, then parent dirs.
fn find_project_root() -> Option<PathBuf> {
    let mut dir = std::env::current_dir().ok()?;
    for _ in 0..3 {
        if dir.join("config.json").exists() {
            return Some(dir);
        }
        dir = dir.parent()?.to_path_buf();
    }
    None
}

fn resolve_config_path(config_path: Option<String>) -> String {
    if let Some(p) = config_path {
        return p;
    }
    if let Some(root) = find_project_root() {
        return root.join("config.json").to_string_lossy().to_string();
    }
    "./config.json".to_string()
}

fn load_dotenv() {
    if let Some(root) = find_project_root() {
        let env_path = root.join(".env");
        if env_path.exists() {
            dotenv::from_path(&env_path).ok();
            return;
        }
    }
    dotenv::dotenv().ok();
}

#[tauri::command]
pub async fn get_config(config_path: Option<String>) -> Result<lib::Config, String> {
    load_dotenv();
    let path = resolve_config_path(config_path);
    lib::load_config(&path)
}

#[tauri::command]
pub async fn translate_preview(
    text: String,
    channel_names: Vec<String>,
    config_path: Option<String>,
) -> Result<Vec<TranslationResult>, String> {
    load_dotenv();
    let path = resolve_config_path(config_path);
    let config = lib::load_config(&path)?;

    let client = reqwest::Client::new();
    let mut results = Vec::new();

    let service_name = config.translation_service.as_deref();

    for name in &channel_names {
        let channel = config
            .channels
            .iter()
            .find(|c| &c.name == name)
            .ok_or_else(|| format!("Channel '{}' not found", name))?;

        if channel.should_translate {
            if let Some(ref tc) = channel.translate {
                let svc_name = service_name.ok_or_else(|| {
                    "translation_service not set in config".to_string()
                })?;
                let service = translation::create_service(svc_name, client.clone())?;
                let translated = service.translate(&text, &tc.from, &tc.to).await?;
                results.push(TranslationResult {
                    channel: name.clone(),
                    translated_text: translated,
                });
            }
        }
    }

    Ok(results)
}

#[tauri::command]
pub async fn upload_image(file_path: String) -> Result<String, String> {
    load_dotenv();
    let client = reqwest::Client::new();
    imgbb::upload_image(&client, &file_path).await
}

#[tauri::command]
pub async fn submit_post(
    text: String,
    images: Vec<String>,
    schedule: Option<String>,
    channel_names: Vec<String>,
    text_overrides: Option<std::collections::HashMap<String, String>>,
    config_path: Option<String>,
) -> Result<Vec<lib::ChannelPostResult>, String> {
    load_dotenv();
    let path = resolve_config_path(config_path);
    let config = lib::load_config(&path)?;

    let api_key = std::env::var("BUFFER_API_KEY")
        .map_err(|_| "BUFFER_API_KEY not set".to_string())?;

    let client = reqwest::Client::new();

    // Resolve images
    let resolved_images = imgbb::resolve_images(&client, &images).await?;

    // Find selected channels
    let selected_channels: Vec<lib::Channel> = channel_names
        .iter()
        .filter_map(|name| config.channels.iter().find(|c| c.name == *name).cloned())
        .collect();

    if selected_channels.is_empty() {
        return Err("No valid channels selected".to_string());
    }

    let overrides = text_overrides.unwrap_or_default();

    // If we have overrides for all translating channels, skip creating the service
    let needs_translation = selected_channels.iter().any(|c| {
        c.should_translate && c.translate.is_some() && !overrides.contains_key(&c.name)
    });

    let translation_service: Option<Box<dyn translation::TranslationService + Send + Sync>> =
        if needs_translation {
            let svc_name = config
                .translation_service
                .as_deref()
                .ok_or_else(|| "translation_service not set in config".to_string())?;
            Some(translation::create_service(svc_name, client.clone())?)
        } else {
            None
        };

    let results = buffer::post_to_channels(
        &client,
        &api_key,
        &text,
        &selected_channels,
        &resolved_images,
        &schedule,
        translation_service
            .as_ref()
            .map(|s| s.as_ref() as &(dyn translation::TranslationService + Send + Sync)),
        &overrides,
    )
    .await;

    Ok(results)
}

#[tauri::command]
pub async fn shrink_text(
    text: String,
    max_chars: Option<usize>,
    config_path: Option<String>,
) -> Result<String, String> {
    load_dotenv();
    let path = resolve_config_path(config_path);
    let config = lib::load_config(&path)?;

    let svc_name = config
        .translation_service
        .as_deref()
        .ok_or_else(|| "translation_service not set in config".to_string())?;

    let client = reqwest::Client::new();
    let service = translation::create_service(svc_name, client)?;

    let limit = max_chars.unwrap_or(280);
    let prompt = translation::build_shrink_prompt(&text, limit);
    service.prompt(&prompt).await
}

#[tauri::command]
pub async fn read_image_base64(file_path: String) -> Result<String, String> {
    use base64::Engine;
    let bytes = std::fs::read(&file_path)
        .map_err(|e| format!("Failed to read file '{}': {}", file_path, e))?;

    let mime = if file_path.ends_with(".png") {
        "image/png"
    } else if file_path.ends_with(".gif") {
        "image/gif"
    } else if file_path.ends_with(".webp") {
        "image/webp"
    } else {
        "image/jpeg"
    };

    let encoded = base64::engine::general_purpose::STANDARD.encode(&bytes);
    Ok(format!("data:{};base64,{}", mime, encoded))
}
