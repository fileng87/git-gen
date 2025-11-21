use anyhow::anyhow;
use git_gen_core::LlmGenerater;
use git_gen_llm::{GeminiLlmGenerater, OpenAILlmGenerater};

use crate::config::AppConfig;

/// LLM provider type
#[derive(Clone, Debug, clap::ValueEnum)]
pub enum LlmProvider {
    OpenAI,
    Gemini,
}

impl LlmProvider {
    /// Detect LLM provider from config
    pub fn detect_from_config(config: &AppConfig) -> Option<Self> {
        // Check default provider first
        if let Some(default) = config.get_default_provider() {
            match default {
                "openai" if config.openai.is_some() => return Some(Self::OpenAI),
                "gemini" if config.gemini.is_some() => return Some(Self::Gemini),
                _ => {}
            }
        }

        // Fallback: check which providers are configured
        if config.openai.is_some() {
            return Some(Self::OpenAI);
        }
        if config.gemini.is_some() {
            return Some(Self::Gemini);
        }

        None
    }

    /// Create LLM generater from this provider using config
    pub fn create_llm_generater_from_config(
        &self,
        config: &AppConfig,
    ) -> anyhow::Result<Box<dyn LlmGenerater>> {
        match self {
            Self::OpenAI => {
                let openai_config = config
                    .openai
                    .as_ref()
                    .ok_or_else(|| anyhow!("OpenAI is not configured in config file or environment"))?;

                let llm = OpenAILlmGenerater::with_config(
                    openai_config.api_key.clone(),
                    openai_config.model.clone(),
                );
                Ok(Box::new(llm))
            }
            Self::Gemini => {
                let gemini_config = config
                    .gemini
                    .as_ref()
                    .ok_or_else(|| anyhow!("Gemini is not configured in config file or environment"))?;

                let llm = GeminiLlmGenerater::with_config(
                    gemini_config.api_key.clone(),
                    gemini_config.model.clone(),
                );
                Ok(Box::new(llm))
            }
        }
    }
}
