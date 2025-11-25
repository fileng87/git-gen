/// Maximum length of diff to send to LLM
pub const MAX_DIFF_LENGTH: usize = 114514;

/// Temperature for LLM generation
pub const TEMPERATURE: f32 = 0.7;

/// Truncate diff if it's too long for LLM token limits
pub fn truncate_diff(diff: String) -> String {
    if diff.len() > MAX_DIFF_LENGTH {
        diff.chars().take(MAX_DIFF_LENGTH).collect::<String>()
    } else {
        diff
    }
}

/// Validate and extract commit message from LLM response
pub fn validate_commit_message(message: String, provider_name: &str) -> anyhow::Result<String> {
    let message = message.trim().to_string();

    if message.is_empty() {
        return Err(anyhow::anyhow!(
            "Empty commit message from {} API",
            provider_name
        ));
    }

    Ok(message)
}

/// Handle API error response
pub async fn handle_api_error(
    response: reqwest::Response,
    provider_name: &str,
) -> anyhow::Result<String> {
    let status = response.status();
    let error_text = response
        .text()
        .await
        .unwrap_or_else(|_| "Unknown error".to_string());

    Err(anyhow::anyhow!(
        "{} API returned error status {}: {}",
        provider_name,
        status,
        error_text
    ))
}
