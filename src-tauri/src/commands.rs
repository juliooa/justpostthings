use justpostthings_lib::{self as lib, buffer, imgbb, translation};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tauri::Manager;

/// Settings sent to/from the frontend. On disk these are split into
/// config.json (same format as CLI) and .env (API keys).
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Settings {
    #[serde(flatten)]
    pub config: lib::Config,
    pub buffer_api_key: String,
    pub openai_api_key: String,
    pub gemini_api_key: String,
    pub imgbb_api_key: String,
}

#[derive(Serialize, Deserialize)]
pub struct TranslationResult {
    pub channel: String,
    pub translated_text: String,
}

fn app_data_dir(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    app.path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data dir: {}", e))
}

fn load_config(app: &tauri::AppHandle) -> Result<lib::Config, String> {
    let path = app_data_dir(app)?.join("config.json");
    if !path.exists() {
        return Err("No config.json found. Please open Settings to configure the app.".to_string());
    }
    let content = std::fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read config.json: {}", e))?;
    serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse config.json: {}", e))
}

fn load_dotenv(app: &tauri::AppHandle) -> Result<(), String> {
    let env_path = app_data_dir(app)?.join(".env");
    if env_path.exists() {
        dotenv::from_path(&env_path).ok();
    }
    Ok(())
}

fn read_env_key(name: &str) -> String {
    std::env::var(name).unwrap_or_default()
}

fn load_settings(app: &tauri::AppHandle) -> Result<Settings, String> {
    load_dotenv(app)?;
    let config = load_config(app)?;
    Ok(Settings {
        config,
        buffer_api_key: read_env_key("BUFFER_API_KEY"),
        openai_api_key: read_env_key("OPENAI_API_KEY"),
        gemini_api_key: read_env_key("GEMINI_API_KEY"),
        imgbb_api_key: read_env_key("IMGBB_API_KEY"),
    })
}

fn apply_env_vars(settings: &Settings) {
    unsafe {
        std::env::set_var("BUFFER_API_KEY", &settings.buffer_api_key);
        std::env::set_var("OPENAI_API_KEY", &settings.openai_api_key);
        std::env::set_var("GEMINI_API_KEY", &settings.gemini_api_key);
        std::env::set_var("IMGBB_API_KEY", &settings.imgbb_api_key);
    }
}

fn save_env_file(app: &tauri::AppHandle, settings: &Settings) -> Result<(), String> {
    let dir = app_data_dir(app)?;
    std::fs::create_dir_all(&dir)
        .map_err(|e| format!("Failed to create app data dir: {}", e))?;

    let content = format!(
        "BUFFER_API_KEY={}\nOPENAI_API_KEY={}\nGEMINI_API_KEY={}\nIMGBB_API_KEY={}\n",
        settings.buffer_api_key,
        settings.openai_api_key,
        settings.gemini_api_key,
        settings.imgbb_api_key,
    );
    std::fs::write(dir.join(".env"), content)
        .map_err(|e| format!("Failed to write .env: {}", e))
}

fn save_config_file(app: &tauri::AppHandle, config: &lib::Config) -> Result<(), String> {
    let dir = app_data_dir(app)?;
    std::fs::create_dir_all(&dir)
        .map_err(|e| format!("Failed to create app data dir: {}", e))?;

    let json = serde_json::to_string_pretty(config)
        .map_err(|e| format!("Failed to serialize config: {}", e))?;
    std::fs::write(dir.join("config.json"), json)
        .map_err(|e| format!("Failed to write config.json: {}", e))
}

#[tauri::command]
pub async fn get_settings(app: tauri::AppHandle) -> Result<Settings, String> {
    load_settings(&app)
}

#[tauri::command]
pub async fn save_settings(app: tauri::AppHandle, settings: Settings) -> Result<(), String> {
    save_config_file(&app, &settings.config)?;
    save_env_file(&app, &settings)?;
    apply_env_vars(&settings);
    Ok(())
}

#[tauri::command]
pub async fn get_config(app: tauri::AppHandle) -> Result<lib::Config, String> {
    load_dotenv(&app)?;
    load_config(&app)
}

#[tauri::command]
pub async fn translate_preview(
    app: tauri::AppHandle,
    text: String,
    channel_names: Vec<String>,
) -> Result<Vec<TranslationResult>, String> {
    let settings = load_settings(&app)?;
    apply_env_vars(&settings);
    let config = settings.config;

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
pub async fn upload_image(app: tauri::AppHandle, file_path: String) -> Result<String, String> {
    load_dotenv(&app)?;
    let client = reqwest::Client::new();
    imgbb::upload_image(&client, &file_path, None).await
}

#[tauri::command]
pub async fn submit_post(
    app: tauri::AppHandle,
    text: String,
    images: Vec<String>,
    schedule: Option<String>,
    channel_names: Vec<String>,
    text_overrides: Option<std::collections::HashMap<String, String>>,
    schedule_overrides: Option<std::collections::HashMap<String, String>>,
) -> Result<Vec<lib::ChannelPostResult>, String> {
    let settings = load_settings(&app)?;
    apply_env_vars(&settings);
    let config = settings.config;

    let api_key = settings.buffer_api_key;
    if api_key.is_empty() {
        return Err("BUFFER_API_KEY not set. Please configure it in Settings.".to_string());
    }

    let client = reqwest::Client::new();

    let resolved_images = imgbb::resolve_images(&client, &images, schedule.as_deref()).await?;

    let selected_channels: Vec<lib::Channel> = channel_names
        .iter()
        .filter_map(|name| config.channels.iter().find(|c| c.name == *name).cloned())
        .collect();

    if selected_channels.is_empty() {
        return Err("No valid channels selected".to_string());
    }

    let overrides = text_overrides.unwrap_or_default();

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

    let sched_overrides = schedule_overrides.unwrap_or_default();

    let results = buffer::post_to_channels(
        &client,
        &api_key,
        &text,
        &selected_channels,
        &resolved_images,
        &schedule,
        &sched_overrides,
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
    app: tauri::AppHandle,
    text: String,
    max_chars: Option<usize>,
) -> Result<String, String> {
    let settings = load_settings(&app)?;
    apply_env_vars(&settings);
    let config = settings.config;

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

// --- Posts ---

fn posts_dir(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    let base = app_data_dir(app)?;
    Ok(base.join("posts"))
}

#[tauri::command]
pub async fn save_sent_post(
    app: tauri::AppHandle,
    channel_texts: Vec<(String, String)>,
) -> Result<(), String> {
    let dir = posts_dir(&app)?;
    std::fs::create_dir_all(&dir)
        .map_err(|e| format!("Failed to create posts dir: {}", e))?;

    let mut content = String::new();
    for (i, (channel, text)) in channel_texts.iter().enumerate() {
        if i > 0 {
            content.push_str("\n-------\n");
        }
        content.push_str(&format!("channel: {}\n{}", channel, text));
    }

    let id = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis()
        .to_string();

    let path = dir.join(format!("{}.txt", id));
    std::fs::write(&path, &content)
        .map_err(|e| format!("Failed to write post: {}", e))?;

    Ok(())
}

#[tauri::command]
pub async fn open_posts_folder(app: tauri::AppHandle) -> Result<(), String> {
    let dir = posts_dir(&app)?;
    std::fs::create_dir_all(&dir)
        .map_err(|e| format!("Failed to create posts dir: {}", e))?;
    open::that(&dir).map_err(|e| format!("Failed to open folder: {}", e))
}

// --- Ideas ---

#[derive(Serialize, Deserialize, Clone)]
pub struct Idea {
    pub id: String,
    pub content: String,
}

fn ideas_dir(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    let base = app_data_dir(app)?;
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
