/// Configuration for OpenAI LLM
#[derive(Clone, Debug)]
pub struct OpenAIConfig {
    /// OpenAI API key
    pub api_key: String,
    /// Model to use (e.g., "gpt-4o-mini", "gpt-4")
    pub model: String,
}

impl OpenAIConfig {
    /// Create a new config with API key and model
    pub fn new(api_key: String, model: String) -> Self {
        Self { api_key, model }
    }
}
