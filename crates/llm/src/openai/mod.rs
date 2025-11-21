use async_trait::async_trait;
use git_gen_core::LlmGenerater;
use std::env;

mod api;
mod config;
mod types;

pub use config::OpenAIConfig;

/// OpenAI implementation of LlmGenerater
pub struct OpenAILlmGenerater {
    config: OpenAIConfig,
    client: reqwest::Client,
}

impl OpenAILlmGenerater {
    /// Create a new OpenAILlmGenerater with api_key and model
    pub fn with_config(api_key: String, model: String) -> Self {
        Self {
            config: OpenAIConfig::new(api_key, model),
            client: reqwest::Client::new(),
        }
    }

    /// Create a new OpenAILlmGenerater with API key and model from environment
    pub fn from_env() -> anyhow::Result<Self> {
        let api_key = env::var("OPENAI_API_KEY")
            .map_err(|_| anyhow::anyhow!("OPENAI_API_KEY environment variable is not set"))?;
        let model = env::var("OPENAI_MODEL")
            .map_err(|_| anyhow::anyhow!("OPENAI_MODEL environment variable is not set"))?;

        Ok(Self::with_config(api_key, model))
    }
}

#[async_trait]
impl LlmGenerater for OpenAILlmGenerater {
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

