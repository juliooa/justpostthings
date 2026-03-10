use clap::Parser;
use std::process;

use justpostthings_lib::{self as lib, buffer, imgbb, translation};

#[derive(Parser)]
#[command(name = "justpostthings", about = "Post to X and LinkedIn via Buffer")]
struct Cli {
    /// The text content of the post
    text: String,

    /// Image URL(s) to attach (can be repeated)
    #[arg(long = "image")]
    images: Vec<String>,

    /// Schedule post for a specific time (ISO 8601 datetime)
    #[arg(long)]
    schedule: Option<String>,

    /// Override default channels (comma-separated names)
    #[arg(long, value_delimiter = ',')]
    channels: Vec<String>,

    /// Path to config file
    #[arg(long, default_value = "./config.json")]
    config: String,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let cli = Cli::parse();

    let api_key = std::env::var("BUFFER_API_KEY").unwrap_or_else(|_| {
        eprintln!("BUFFER_API_KEY not set. Add it to .env or export it.");
        process::exit(1);
    });

    let config = lib::load_config(&cli.config).unwrap_or_else(|e| {
        eprintln!("{}", e);
        process::exit(1);
    });

    let selected_names: Vec<String> = if cli.channels.is_empty() {
        config.default_post_channels.clone()
    } else {
        cli.channels.clone()
    };

    let selected_channels: Vec<lib::Channel> = selected_names
        .iter()
        .filter_map(|name| {
            config
                .channels
                .iter()
                .find(|c| c.name == *name)
                .cloned()
                .or_else(|| {
                    eprintln!("Warning: channel '{}' not found in config, skipping", name);
                    None
                })
        })
        .collect();

    if selected_channels.is_empty() {
        eprintln!("No valid channels selected.");
        process::exit(1);
    }

    let client = reqwest::Client::new();

    // Resolve images
    let resolved_images = match imgbb::resolve_images(&client, &cli.images).await {
        Ok(urls) => urls,
        Err(e) => {
            eprintln!("Image resolution failed: {}", e);
            process::exit(1);
        }
    };

    // Create LLM service lazily
    let translation_service: Option<Box<dyn translation::TranslationService + Send + Sync>> =
        if selected_channels
            .iter()
            .any(|c| c.should_translate && c.translate.is_some())
        {
            let llm = config.llm_service.as_ref().unwrap_or_else(|| {
                eprintln!(
                    "A channel has translation enabled but 'llm_service' is not set in config."
                );
                process::exit(1);
            });
            match translation::create_service(&llm.provider, llm.model.as_deref(), client.clone()) {
                Ok(s) => Some(s),
                Err(e) => {
                    eprintln!("{}", e);
                    process::exit(1);
                }
            }
        } else {
            None
        };

    let results = buffer::post_to_channels(
        &client,
        &api_key,
        &cli.text,
        &selected_channels,
        &resolved_images,
        &cli.schedule,
        translation_service
            .as_ref()
            .map(|s| s.as_ref() as &(dyn translation::TranslationService + Send + Sync)),
        &std::collections::HashMap::new(),
    )
    .await;

    for result in &results {
        if result.success {
            if let Some(ref id) = result.post_id {
                println!("[{}] {} (id: {})", result.channel, result.message, id);
            } else {
                println!("[{}] {}", result.channel, result.message);
            }
        } else {
            eprintln!("[{}] {}", result.channel, result.message);
        }
    }

    if results.iter().any(|r| !r.success) {
        process::exit(1);
    }
}
