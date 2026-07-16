use pitha_core::{CoreResult, Lifecycle};

use crate::error::FoundationError;

/// Lifecycle states for managed components.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LifecycleState {
    Created,
    Initialized,
    Starting,
    Running,
    Stopping,
    Stopped,
    Failed,
}

/// Manages the lifecycle of platform components.
pub struct LifecycleManager {
    state: LifecycleState,
    components: Vec<Box<dyn Lifecycle>>,
}

impl LifecycleManager {
    pub fn new() -> Self {
        Self {
            state: LifecycleState::Created,
            components: Vec::new(),
        }
    }

    /// Register a component for lifecycle management.
    pub fn register(&mut self, component: impl Lifecycle + 'static) {
        self.components.push(Box::new(component));
    }

    /// Initialize all registered components.
    pub fn initialize(&mut self) -> CoreResult<()> {
        for component in &mut self.components {
            component.initialize()?;
        }
        self.state = LifecycleState::Initialized;
        Ok(())
    }

    /// Start all registered components.
    pub fn start(&mut self) -> CoreResult<()> {
        if self.state != LifecycleState::Initialized {
            return Err(FoundationError::lifecycle("cannot start: not initialized").into());
        }

        self.state = LifecycleState::Starting;
        for component in &mut self.components {
            component.start()?;
        }
        self.state = LifecycleState::Running;
        Ok(())
    }

    /// Stop all registered components in reverse order.
    pub fn stop(&mut self) -> CoreResult<()> {
        if self.state != LifecycleState::Running {
            return Err(FoundationError::lifecycle("cannot stop: not running").into());
        }

        self.state = LifecycleState::Stopping;
        for component in self.components.iter_mut().rev() {
            component.stop()?;
        }
        self.state = LifecycleState::Stopped;
        Ok(())
    }

    /// Shut down all registered components in reverse order.
    pub fn shutdown(&mut self) -> CoreResult<()> {
        for component in self.components.iter_mut().rev() {
            component.shutdown()?;
        }
        self.state = LifecycleState::Stopped;
        Ok(())
    }

    /// Get the current lifecycle state.
    pub fn state(&self) -> LifecycleState {
        self.state
    }

    /// Check if the manager is running.
    pub fn is_running(&self) -> bool {
        self.state == LifecycleState::Running
    }
}

impl Default for LifecycleManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestComponent {
        initialized: bool,
        running: bool,
    }

    impl TestComponent {
        fn new() -> Self {
            Self {
                initialized: false,
                running: false,
            }
        }
    }

    impl Lifecycle for TestComponent {
        fn initialize(&mut self) -> CoreResult<()> {
            self.initialized = true;
            Ok(())
        }

        fn start(&mut self) -> CoreResult<()> {
            self.running = true;
            Ok(())
        }

        fn stop(&mut self) -> CoreResult<()> {
            self.running = false;
            Ok(())
        }

        fn is_running(&self) -> bool {
            self.running
        }
    }

    #[test]
    fn test_lifecycle_manager() {
        let mut manager = LifecycleManager::new();
        manager.register(TestComponent::new());

        assert_eq!(manager.state(), LifecycleState::Created);

        manager.initialize().unwrap();
        assert_eq!(manager.state(), LifecycleState::Initialized);

        manager.start().unwrap();
        assert_eq!(manager.state(), LifecycleState::Running);
        assert!(manager.is_running());

        manager.stop().unwrap();
        assert_eq!(manager.state(), LifecycleState::Stopped);
        assert!(!manager.is_running());
    }
}
