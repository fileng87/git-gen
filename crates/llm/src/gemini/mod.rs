use async_trait::async_trait;
use git_gen_core::LlmGenerater;
use std::env;

mod api;
mod config;
mod types;

pub use config::GeminiConfig;

/// Gemini implementation of LlmGenerater
pub struct GeminiLlmGenerater {
    config: GeminiConfig,
    client: reqwest::Client,
}

impl GeminiLlmGenerater {
    /// Create a new GeminiLlmGenerater with api_key and model
    pub fn with_config(api_key: String, model: String) -> Self {
        Self {
            config: GeminiConfig::new(api_key, model),
            client: reqwest::Client::new(),
        }
    }

    /// Create a new GeminiLlmGenerater with API key and model from environment
    pub fn from_env() -> anyhow::Result<Self> {
        let api_key = env::var("GEMINI_API_KEY")
            .map_err(|_| anyhow::anyhow!("GEMINI_API_KEY environment variable is not set"))?;
        let model = env::var("GEMINI_MODEL")
            .map_err(|_| anyhow::anyhow!("GEMINI_MODEL environment variable is not set"))?;

        Ok(Self::with_config(api_key, model))
    }
}

#[async_trait]
impl LlmGenerater for GeminiLlmGenerater {
    async fn generate_commit_message(&self, diff: String) -> anyhow::Result<String> {
        api::generate_commit_message(&self.config, diff, &self.client).await
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_system_prompt_exists() {
        let prompt = crate::prompt::SYSTEM_PROMPT;
        assert!(!prompt.is_empty());
        assert!(prompt.contains("commit message"));
    }
}
