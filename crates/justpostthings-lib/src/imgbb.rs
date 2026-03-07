use base64::Engine;
use serde::Deserialize;
use std::path::Path;

#[derive(Deserialize)]
struct ImgbbResponse {
    data: ImgbbData,
}

#[derive(Deserialize)]
struct ImgbbData {
    url: String,
}

pub async fn upload_image(client: &reqwest::Client, file_path: &str) -> Result<String, String> {
    let api_key = std::env::var("IMGBB_API_KEY")
        .map_err(|_| "IMGBB_API_KEY not set. Add it to .env or export it.".to_string())?;

    let path = Path::new(file_path);
    if !path.exists() {
        return Err(format!("File not found: {}", file_path));
    }

    let file_bytes = std::fs::read(path)
        .map_err(|e| format!("Failed to read file '{}': {}", file_path, e))?;

    let encoded = base64::engine::general_purpose::STANDARD.encode(&file_bytes);

    let form = reqwest::multipart::Form::new()
        .text("key", api_key)
        .text("image", encoded)
        .text("expiration", "600");

    let resp = client
        .post("https://api.imgbb.com/1/upload")
        .multipart(form)
        .send()
        .await
        .map_err(|e| format!("imgbb upload request failed: {}", e))?;

    let status = resp.status();
    let body = resp.text().await.unwrap_or_default();

    if !status.is_success() {
        return Err(format!("imgbb returned HTTP {}: {}", status, body));
    }

    let parsed: ImgbbResponse = serde_json::from_str(&body)
        .map_err(|e| format!("Failed to parse imgbb response: {} — {}", e, body))?;

    Ok(parsed.data.url)
}

pub async fn resolve_images(client: &reqwest::Client, images: &[String]) -> Result<Vec<String>, String> {
    let mut urls = Vec::new();
    for img in images {
        if img.starts_with("http://") || img.starts_with("https://") {
            urls.push(img.clone());
        } else {
            let url = upload_image(client, img).await?;
            urls.push(url);
        }
    }
    Ok(urls)
}
