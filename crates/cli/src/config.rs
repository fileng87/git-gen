use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

/// Application configuration
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AppConfig {
    /// Default LLM provider to use
    #[serde(default)]
    pub default_provider: Option<String>,

    /// OpenAI configuration
    #[serde(default)]
    pub openai: Option<OpenAIConfig>,

    /// Gemini configuration
    #[serde(default)]
    pub gemini: Option<GeminiConfig>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct OpenAIConfig {
    pub api_key: String,
    pub model: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GeminiConfig {
    pub api_key: String,
    pub model: String,
}

impl AppConfig {
    /// Load configuration from file or environment variables
    /// Automatically creates config file if it doesn't exist
    pub fn load() -> Result<Self> {
        // Try to load from config file first
        if let Some(config_path) = Self::get_config_path() {
            if config_path.exists() {
                match Self::load_from_file(&config_path) {
                    Ok(config) => return Ok(config),
                    Err(e) => {
                        eprintln!("Warning: Failed to load config from {}: {}", config_path.display(), e);
                        // Fall through to create new config
                    }
                }
            } else {
                // Config file doesn't exist, try to create it
                match Self::create_config_file(&config_path) {
                    Ok(config) => {
                        Self::print_config_created_message(&config_path, &config);
                        return Ok(config);
                    }
                    Err(e) => {
                        eprintln!("Warning: Failed to create config file: {}", e);
                        // Fall through to environment variables
                    }
                }
            }
        }

        // Fallback to environment variables
        Self::load_from_env()
    }

    /// Get the config file path
    /// Always use home directory: ~/.git-gen/config.toml
    pub fn get_config_path() -> Option<PathBuf> {
        dirs::home_dir().map(|home_dir| home_dir.join(".git-gen").join("config.toml"))
    }

    /// Create config file from environment variables or with example values
    fn create_config_file(config_path: &Path) -> Result<Self> {
        // Create parent directory if it doesn't exist
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create config directory: {}", parent.display()))?;
        }

        // Try to create config from environment variables first
        let config = match Self::load_from_env() {
            Ok(config) => config,
            Err(_) => {
                // If no env vars, create example config
                AppConfig {
                    default_provider: Some("openai".to_string()),
                    openai: Some(OpenAIConfig {
                        api_key: "your-api-key-here".to_string(),
                        model: "gpt-4o-mini".to_string(),
                    }),
                    gemini: Some(GeminiConfig {
                        api_key: "your-api-key-here".to_string(),
                        model: "gemini-pro".to_string(),
                    }),
                }
            }
        };

        // Write config to file
        let toml_content = toml::to_string_pretty(&config)
            .context("Failed to serialize config to TOML")?;

        fs::write(config_path, toml_content)
            .with_context(|| format!("Failed to write config file: {}", config_path.display()))?;

        Ok(config)
    }

    /// Print helpful message when config file is created
    fn print_config_created_message(config_path: &Path, config: &AppConfig) {
        println!("Created config file at: {}", config_path.display());

        // Check if config has placeholder values
        let has_placeholder = config
            .openai
            .as_ref()
            .map(|c| c.api_key == "your-api-key-here")
            .unwrap_or(false)
            || config
                .gemini
                .as_ref()
                .map(|c| c.api_key == "your-api-key-here")
                .unwrap_or(false);

        if has_placeholder {
            println!();
            println!("⚠️  Configuration file created with example values.");
            println!("   Please edit the config file and replace 'your-api-key-here' with your actual API keys:");
            println!("   {}", config_path.display());
            println!();
            println!("   Alternatively, you can set environment variables:");
            println!("   - OPENAI_API_KEY and OPENAI_MODEL");
            println!("   - GEMINI_API_KEY and GEMINI_MODEL");
        } else {
            println!("   Configuration loaded from environment variables.");
        }
    }

    /// Load configuration from file
    fn load_from_file(path: &Path) -> Result<Self> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read config file: {}", path.display()))?;

        let config: AppConfig = toml::from_str(&content)
            .context("Failed to parse config file as TOML")?;

        Ok(config)
    }

    /// Load configuration from environment variables
    fn load_from_env() -> Result<Self> {
        let mut config = AppConfig {
            default_provider: None,
            openai: None,
            gemini: None,
        };

        // Try OpenAI
        if let (Ok(api_key), Ok(model)) = (env::var("OPENAI_API_KEY"), env::var("OPENAI_MODEL")) {
            config.openai = Some(OpenAIConfig { api_key, model });
            if config.default_provider.is_none() {
                config.default_provider = Some("openai".to_string());
            }
        }

        // Try Gemini
        if let (Ok(api_key), Ok(model)) = (env::var("GEMINI_API_KEY"), env::var("GEMINI_MODEL")) {
            config.gemini = Some(GeminiConfig { api_key, model });
            if config.default_provider.is_none() {
                config.default_provider = Some("gemini".to_string());
            }
        }

        // Check if at least one provider is configured
        if config.openai.is_none() && config.gemini.is_none() {
            return Err(anyhow!(
                "No LLM provider configured. Please either:\n  - Create a config file at {}\n  - Set environment variables: OPENAI_API_KEY/OPENAI_MODEL or GEMINI_API_KEY/GEMINI_MODEL\n  - Or use --llm flag to specify provider",
                Self::get_config_path()
                    .map(|p| p.display().to_string())
                    .unwrap_or_else(|| "~/.git-gen/config.toml".to_string())
            ));
        }

        Ok(config)
    }

    /// Get the default provider name
    pub fn get_default_provider(&self) -> Option<&str> {
        self.default_provider.as_deref()
    }

    /// Check if a specific provider is configured
    pub fn has_provider(&self, provider: &str) -> bool {
        match provider {
            "openai" => self.openai.is_some(),
            "gemini" => self.gemini.is_some(),
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_config_path() {
        let path = AppConfig::get_config_path();
        assert!(path.is_some());
        let path = path.unwrap();
        assert!(path.to_string_lossy().contains("git-gen"));
        assert!(path.to_string_lossy().ends_with("config.toml"));
    }

    #[test]
    fn test_has_provider() {
        let config = AppConfig {
            default_provider: Some("openai".to_string()),
            openai: Some(OpenAIConfig {
                api_key: "key".to_string(),
                model: "model".to_string(),
            }),
            gemini: None,
        };

        assert!(config.has_provider("openai"));
        assert!(!config.has_provider("gemini"));
    }

    #[test]
    fn test_get_default_provider() {
        let config = AppConfig {
            default_provider: Some("openai".to_string()),
            openai: None,
            gemini: None,
        };

        assert_eq!(config.get_default_provider(), Some("openai"));
    }
}
