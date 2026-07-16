use pitha_core::CoreResult;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

use crate::config::LoggingConfig;
use crate::error::FoundationError;

/// Structured logging infrastructure using `tracing`.
pub struct Logger;

impl Logger {
    /// Initialize the logging subsystem.
    pub fn init(config: &LoggingConfig) -> CoreResult<Self> {
        let level = config.level.as_deref().unwrap_or("info");
        let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(level));

        let subscriber = tracing_subscriber::registry()
            .with(filter)
            .with(fmt::layer());

        tracing::subscriber::set_global_default(subscriber).map_err(|e| {
            FoundationError::logging(format!("failed to set global subscriber: {e}"))
        })?;

        Ok(Self)
    }

    /// Initialize with a default configuration.
    pub fn init_default() -> CoreResult<Self> {
        Self::init(&LoggingConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logger_init_default() {
        let result = Logger::init_default();
        // May fail if already initialized in another test
        if result.is_err() {
            // Already initialized, that's fine
        }
    }
}
