/// Configuration for Gemini LLM
#[derive(Clone, Debug)]
pub struct GeminiConfig {
    /// Gemini API key
    pub api_key: String,
    /// Model to use (e.g., "gemini-pro", "gemini-1.5-pro")
    pub model: String,
}

impl GeminiConfig {
    /// Create a new config with API key and model
    pub fn new(api_key: String, model: String) -> Self {
        Self { api_key, model }
    }
}
