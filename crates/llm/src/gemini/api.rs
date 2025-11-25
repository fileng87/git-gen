use super::config::GeminiConfig;
use super::types::{
    Content, GenerateContentRequest, GenerateContentResponse, GenerationConfig, Part,
    SystemInstruction,
};
use crate::common::{TEMPERATURE, handle_api_error, truncate_diff, validate_commit_message};
use anyhow::anyhow;

const GEMINI_API_BASE_URL: &str = "https://generativelanguage.googleapis.com/v1beta";

pub async fn generate_commit_message(
    config: &GeminiConfig,
    diff: String,
    client: &reqwest::Client,
) -> anyhow::Result<String> {
    // Truncate diff if too long
    let diff = truncate_diff(diff);

    let request = GenerateContentRequest {
        contents: vec![Content {
            parts: vec![Part { text: diff }],
            role: Some("user".to_string()),
        }],
        system_instruction: Some(SystemInstruction {
            parts: vec![Part {
                text: crate::prompt::SYSTEM_PROMPT.to_string(),
            }],
        }),
        generation_config: Some(GenerationConfig {
            temperature: Some(TEMPERATURE),
        }),
    };

    let url = format!(
        "{}/models/{}:generateContent?key={}",
        GEMINI_API_BASE_URL, config.model, config.api_key
    );

    let response = client
        .post(&url)
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await
        .map_err(|e| anyhow!("Failed to send request to Gemini API: {}", e))?;

    if !response.status().is_success() {
        return handle_api_error(response, "Gemini").await;
    }

    let gemini_response: GenerateContentResponse = response
        .json()
        .await
        .map_err(|e| anyhow!("Failed to parse Gemini API response: {}", e))?;

    let message = gemini_response
        .candidates
        .first()
        .and_then(|candidate| candidate.content.parts.first())
        .map(|part| part.text.clone())
        .ok_or_else(|| anyhow!("No message in Gemini API response"))?;

    validate_commit_message(message, "Gemini")
}
