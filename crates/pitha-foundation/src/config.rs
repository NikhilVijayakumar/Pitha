use std::path::Path;

use pitha_core::CoreResult;
use serde::Deserialize;

use crate::error::FoundationError;

/// Application configuration loaded from TOML files and environment variables.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub application: ApplicationConfig,

    #[serde(default)]
    pub logging: LoggingConfig,

    #[serde(default)]
    pub storage: StorageConfig,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct ApplicationConfig {
    pub name: String,
    pub version: Option<String>,
    pub environment: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LoggingConfig {
    pub level: Option<String>,
    pub format: Option<String>,
    pub file: Option<String>,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: Some("info".to_string()),
            format: Some("text".to_string()),
            file: None,
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct StorageConfig {
    pub backend: Option<String>,
    pub path: Option<String>,
}

impl Config {
    /// Load configuration from a TOML file.
    pub fn from_file(path: impl AsRef<Path>) -> CoreResult<Self> {
        let content = std::fs::read_to_string(path.as_ref()).map_err(|e| {
            FoundationError::configuration(format!(
                "failed to read config file {}: {e}",
                path.as_ref().display()
            ))
        })?;

        Self::parse(&content)
    }

    /// Parse configuration from a TOML string.
    pub fn parse(content: &str) -> CoreResult<Self> {
        toml::from_str(content).map_err(|e| {
            FoundationError::configuration(format!("failed to parse config: {e}")).into()
        })
    }

    /// Load configuration with environment variable overrides.
    pub fn load(path: impl AsRef<Path>) -> CoreResult<Self> {
        let mut config = Self::from_file(path)?;

        // Apply environment variable overrides
        if let Ok(env) = std::env::var("PITHA_APP_NAME") {
            config.application.name = env;
        }
        if let Ok(env) = std::env::var("PITHA_LOG_LEVEL") {
            config.logging.level = Some(env);
        }
        if let Ok(env) = std::env::var("PITHA_STORAGE_BACKEND") {
            config.storage.backend = Some(env);
        }

        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_from_str() {
        let toml = r#"
[application]
name = "test-app"
version = "0.1.0"

[logging]
level = "debug"
"#;

        let config = Config::parse(toml).unwrap();
        assert_eq!(config.application.name, "test-app");
        assert_eq!(config.logging.level.as_deref(), Some("debug"));
    }

    #[test]
    fn test_config_defaults() {
        let toml = r#"
[application]
name = "test-app"
"#;

        let config = Config::parse(toml).unwrap();
        assert_eq!(config.logging.level.as_deref(), Some("info"));
        assert_eq!(config.logging.format.as_deref(), Some("text"));
    }
}
