mod common;
mod prompt;

pub mod openai;
pub mod gemini;

// Re-export commonly used types
pub use openai::{OpenAILlmGenerater, OpenAIConfig};
pub use gemini::{GeminiLlmGenerater, GeminiConfig};

