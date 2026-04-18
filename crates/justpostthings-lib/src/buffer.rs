use serde::{Deserialize, Serialize};

use crate::{Channel, ChannelPostResult, translation};

#[derive(Serialize)]
struct GraphQLRequest {
    query: String,
    variables: serde_json::Value,
}

#[derive(Deserialize, Debug)]
struct GraphQLResponse {
    data: Option<serde_json::Value>,
    errors: Option<Vec<serde_json::Value>>,
}

const MUTATION: &str = r#"
mutation CreatePost($input: CreatePostInput!) {
  createPost(input: $input) {
    ... on PostActionSuccess {
      post { id text assets { id mimeType } }
    }
    ... on MutationError {
      message
    }
  }
}
"#;

pub fn build_variables(
    text: &str,
    channel_id: &str,
    images: &[String],
    schedule: &Option<String>,
) -> serde_json::Value {
    let mut input = if let Some(due_at) = schedule {
        serde_json::json!({
            "text": text,
            "channelId": channel_id,
            "schedulingType": "automatic",
            "mode": "customScheduled",
            "dueAt": due_at,
        })
    } else {
        serde_json::json!({
            "text": text,
            "channelId": channel_id,
            "schedulingType": "automatic",
            "mode": "shareNow",
        })
    };

    if !images.is_empty() {
        let image_assets: Vec<serde_json::Value> = images
            .iter()
            .map(|url| serde_json::json!({ "url": url }))
            .collect();
        input["assets"] = serde_json::json!({ "images": image_assets });
    }

    serde_json::json!({ "input": input })
}

pub async fn post_to_channels(
    client: &reqwest::Client,
    api_key: &str,
    text: &str,
    channels: &[Channel],
    images: &[String],
    schedule: &Option<String>,
    schedule_overrides: &std::collections::HashMap<String, String>,
    translation_service: Option<&(dyn translation::TranslationService + Send + Sync)>,
    text_overrides: &std::collections::HashMap<String, String>,
    custom_translation_prompt: Option<&str>,
) -> Vec<ChannelPostResult> {
    let mut results = Vec::new();

    for channel in channels {
        eprintln!("[buffer] Posting to channel '{}'...", channel.name);
        let post_text = if let Some(override_text) = text_overrides.get(&channel.name) {
            override_text.clone()
        } else if channel.should_translate {
            if let Some(ref translate_config) = channel.translate {
                if let Some(service) = translation_service {
                    match service
                        .translate(text, &translate_config.from, &translate_config.to, custom_translation_prompt)
                        .await
                    {
                        Ok(translated) => translated,
                        Err(e) => {
                            results.push(ChannelPostResult {
                                channel: channel.name.clone(),
                                success: false,
                                message: format!("Translation failed: {}", e),
                                post_id: None,
                            });
                            continue;
                        }
                    }
                } else {
                    text.to_string()
                }
            } else {
                text.to_string()
            }
        } else {
            text.to_string()
        };

        eprintln!("[buffer] Text for '{}': {:?}", channel.name, post_text);
        let effective_schedule = schedule_overrides
            .get(&channel.name)
            .cloned()
            .or_else(|| schedule.clone());
        let variables = build_variables(&post_text, &channel.id, images, &effective_schedule);
        eprintln!("[buffer] Variables: {}", serde_json::to_string_pretty(&variables).unwrap_or_default());

        let body = GraphQLRequest {
            query: MUTATION.to_string(),
            variables,
        };

        let res = client
            .post("https://api.buffer.com")
            .header("Authorization", format!("Bearer {}", api_key))
            .json(&body)
            .send()
            .await;

        match res {
            Ok(response) => {
                let status = response.status();
                let resp_text = response.text().await.unwrap_or_default();

                if !status.is_success() {
                    results.push(ChannelPostResult {
                        channel: channel.name.clone(),
                        success: false,
                        message: format!("HTTP {}: {}", status, resp_text),
                        post_id: None,
                    });
                    continue;
                }

                match serde_json::from_str::<GraphQLResponse>(&resp_text) {
                    Ok(gql) => {
                        if let Some(errors) = gql.errors {
                            results.push(ChannelPostResult {
                                channel: channel.name.clone(),
                                success: false,
                                message: format!("GraphQL errors: {:?}", errors),
                                post_id: None,
                            });
                        } else if let Some(data) = gql.data {
                            let create_post = &data["createPost"];
                            if let Some(message) = create_post.get("message") {
                                results.push(ChannelPostResult {
                                    channel: channel.name.clone(),
                                    success: false,
                                    message: format!("Mutation error: {}", message),
                                    post_id: None,
                                });
                            } else if let Some(post) = create_post.get("post") {
                                let post_id =
                                    post["id"].as_str().unwrap_or("unknown").to_string();
                                results.push(ChannelPostResult {
                                    channel: channel.name.clone(),
                                    success: true,
                                    message: "Posted successfully".to_string(),
                                    post_id: Some(post_id),
                                });
                            } else {
                                results.push(ChannelPostResult {
                                    channel: channel.name.clone(),
                                    success: true,
                                    message: format!("Response: {}", create_post),
                                    post_id: None,
                                });
                            }
                        }
                    }
                    Err(e) => {
                        results.push(ChannelPostResult {
                            channel: channel.name.clone(),
                            success: false,
                            message: format!("Failed to parse response: {} — {}", e, resp_text),
                            post_id: None,
                        });
                    }
                }
            }
            Err(e) => {
                results.push(ChannelPostResult {
                    channel: channel.name.clone(),
                    success: false,
                    message: format!("Request failed: {}", e),
                    post_id: None,
                });
            }
        }
    }

    results
}
