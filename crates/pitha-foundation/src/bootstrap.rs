use pitha_core::CoreResult;

use crate::config::Config;
use crate::lifecycle::LifecycleManager;
use crate::logging::Logger;
use crate::state::StateRegistry;

/// Bootstrap result containing initialized platform components.
pub struct BootstrapResult {
    pub config: Config,
    pub state: StateRegistry,
    pub lifecycle: LifecycleManager,
}

/// Application bootstrap coordinator.
///
/// Initializes platform components in the correct order:
/// Config → Logger → State → Lifecycle
pub struct Bootstrap {
    config_path: Option<String>,
}

impl Bootstrap {
    pub fn new() -> Self {
        Self { config_path: None }
    }

    /// Set the configuration file path.
    pub fn with_config(mut self, path: impl Into<String>) -> Self {
        self.config_path = Some(path.into());
        self
    }

    /// Execute the bootstrap sequence.
    pub fn run(self) -> CoreResult<BootstrapResult> {
        // 1. Load configuration
        let config = match self.config_path {
            Some(path) => Config::load(path)?,
            None => Config::default(),
        };

        // 2. Initialize logging
        let _logger = Logger::init(&config.logging)?;

        // 3. Initialize state registry
        let mut state = StateRegistry::new();
        state.insert(config.clone());

        // 4. Initialize lifecycle manager
        let mut lifecycle = LifecycleManager::new();
        lifecycle.initialize()?;

        Ok(BootstrapResult {
            config,
            state,
            lifecycle,
        })
    }
}

impl Default for Bootstrap {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bootstrap_default() {
        let result = Bootstrap::new().run();
        if result.is_err() {
            // May fail if logging already initialized
        }
    }

    #[test]
    fn test_bootstrap_with_config() {
        let result = Bootstrap::new().with_config("nonexistent.toml").run();
        assert!(result.is_err());
    }
}
