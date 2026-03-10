use justpostthings_lib::{self as lib, buffer, imgbb, translation};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tauri::Manager;

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

    let llm = config.llm_service.as_ref();

    for name in &channel_names {
        let channel = config
            .channels
            .iter()
            .find(|c| &c.name == name)
            .ok_or_else(|| format!("Channel '{}' not found", name))?;

        if channel.should_translate {
            if let Some(ref tc) = channel.translate {
                let llm = llm.ok_or_else(|| {
                    "llm_service not set in config".to_string()
                })?;
                let service = translation::create_service(&llm.provider, llm.model.as_deref(), client.clone())?;
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
            let llm = config
                .llm_service
                .as_ref()
                .ok_or_else(|| "llm_service not set in config".to_string())?;
            Some(translation::create_service(&llm.provider, llm.model.as_deref(), client.clone())?)
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

    let llm = config
        .llm_service
        .as_ref()
        .ok_or_else(|| "llm_service not set in config".to_string())?;

    let client = reqwest::Client::new();
    let service = translation::create_service(&llm.provider, llm.model.as_deref(), client)?;

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

// --- Ideas ---

#[derive(Serialize, Deserialize, Clone)]
pub struct Idea {
    pub id: String,
    pub content: String,
}

fn ideas_dir(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    let base = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data dir: {}", e))?;
    Ok(base.join("ideas"))
}

#[tauri::command]
pub async fn list_ideas(app: tauri::AppHandle) -> Result<Vec<Idea>, String> {
    let dir = ideas_dir(&app)?;
    if !dir.exists() {
        return Ok(Vec::new());
    }

    let mut ideas: Vec<Idea> = std::fs::read_dir(&dir)
        .map_err(|e| format!("Failed to read ideas dir: {}", e))?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.extension()?.to_str()? != "txt" {
                return None;
            }
            let id = path.file_stem()?.to_str()?.to_string();
            let content = std::fs::read_to_string(&path).ok()?;
            Some(Idea { id, content })
        })
        .collect();

    // Sort newest first
    ideas.sort_by(|a, b| b.id.cmp(&a.id));
    Ok(ideas)
}

#[tauri::command]
pub async fn create_idea(app: tauri::AppHandle, content: String) -> Result<Idea, String> {
    let dir = ideas_dir(&app)?;
    std::fs::create_dir_all(&dir)
        .map_err(|e| format!("Failed to create ideas dir: {}", e))?;

    let id = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis()
        .to_string();

    let path = dir.join(format!("{}.txt", id));
    std::fs::write(&path, &content)
        .map_err(|e| format!("Failed to write idea: {}", e))?;

    Ok(Idea { id, content })
}

#[tauri::command]
pub async fn update_idea(app: tauri::AppHandle, id: String, content: String) -> Result<(), String> {
    let dir = ideas_dir(&app)?;
    let path = dir.join(format!("{}.txt", id));
    if !path.exists() {
        return Err(format!("Idea '{}' not found", id));
    }
    std::fs::write(&path, &content)
        .map_err(|e| format!("Failed to update idea: {}", e))?;
    Ok(())
}

#[tauri::command]
pub async fn delete_idea(app: tauri::AppHandle, id: String) -> Result<(), String> {
    let dir = ideas_dir(&app)?;
    let path = dir.join(format!("{}.txt", id));
    if path.exists() {
        std::fs::remove_file(&path)
            .map_err(|e| format!("Failed to delete idea: {}", e))?;
    }
    Ok(())
}
