use super::config::OpenAIConfig;
use super::types::{ChatMessage, ChatRequest, ChatResponse};
use crate::common::{TEMPERATURE, handle_api_error, truncate_diff, validate_commit_message};
use anyhow::anyhow;

const OPENAI_API_URL: &str = "https://api.openai.com/v1/chat/completions";

pub async fn generate_commit_message(
    config: &OpenAIConfig,
    diff: String,
    client: &reqwest::Client,
) -> anyhow::Result<String> {
    // Truncate diff if too long
    let diff = truncate_diff(diff);

    let request = ChatRequest {
        model: config.model.clone(),
        messages: vec![
            ChatMessage {
                role: "system".to_string(),
                content: crate::prompt::SYSTEM_PROMPT.to_string(),
            },
            ChatMessage {
                role: "user".to_string(),
                content: diff,
            },
        ],
        temperature: TEMPERATURE,
    };

    let response = client
        .post(OPENAI_API_URL)
        .header("Authorization", format!("Bearer {}", config.api_key))
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await
        .map_err(|e| anyhow!("Failed to send request to OpenAI API: {}", e))?;

    if !response.status().is_success() {
        return handle_api_error(response, "OpenAI").await;
    }

    let chat_response: ChatResponse = response
        .json()
        .await
        .map_err(|e| anyhow!("Failed to parse OpenAI API response: {}", e))?;

    let message = chat_response
        .choices
        .first()
        .map(|choice| choice.message.content.clone())
        .ok_or_else(|| anyhow!("No message in OpenAI API response"))?;

    validate_commit_message(message, "OpenAI")
}
