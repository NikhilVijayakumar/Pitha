use pitha_core::CoreResult;
use pitha_foundation::{Bootstrap, BootstrapResult, Config};

/// CLI runtime for Pīṭha applications.
pub struct CliRuntime {
    bootstrap_result: Option<BootstrapResult>,
}

impl CliRuntime {
    pub fn new() -> Self {
        Self {
            bootstrap_result: None,
        }
    }

    /// Initialize the runtime with configuration.
    pub fn init(&mut self, config_path: Option<&str>) -> CoreResult<()> {
        let bootstrap = match config_path {
            Some(path) => Bootstrap::new().with_config(path),
            None => Bootstrap::new(),
        };

        let result = bootstrap.run()?;
        self.bootstrap_result = Some(result);
        Ok(())
    }

    /// Get a reference to the configuration.
    pub fn config(&self) -> CoreResult<&Config> {
        self.bootstrap_result
            .as_ref()
            .map(|r| &r.config)
            .ok_or_else(|| pitha_core::CoreError::initialization("runtime not initialized"))
    }

    /// Run the application with the provided handler.
    pub async fn run<F, Fut>(&mut self, handler: F) -> CoreResult<()>
    where
        F: FnOnce(BootstrapResult) -> Fut,
        Fut: std::future::Future<Output = CoreResult<()>>,
    {
        let result = self
            .bootstrap_result
            .take()
            .ok_or_else(|| pitha_core::CoreError::initialization("runtime not initialized"))?;

        handler(result).await
    }
}

impl Default for CliRuntime {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_runtime_new() {
        let runtime = CliRuntime::new();
        assert!(runtime.bootstrap_result.is_none());
    }
}
