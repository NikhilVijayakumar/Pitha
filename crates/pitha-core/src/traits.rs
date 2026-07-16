use crate::result::CoreResult;

/// Trait for types that have a unique identifier.
pub trait Identifiable {
    type Id: Eq + std::hash::Hash + Clone + std::fmt::Debug;

    fn id(&self) -> &Self::Id;
}

/// Trait for types that have a human-readable name.
pub trait Nameable {
    fn name(&self) -> &str;
}

/// Trait for types that can validate themselves.
pub trait Validate {
    fn validate(&self) -> CoreResult<()>;
}

/// Trait for types with a managed lifecycle.
pub trait Lifecycle {
    /// Initialize the component.
    fn initialize(&mut self) -> CoreResult<()>;

    /// Start the component.
    fn start(&mut self) -> CoreResult<()> {
        Ok(())
    }

    /// Stop the component.
    fn stop(&mut self) -> CoreResult<()> {
        Ok(())
    }

    /// Shut down the component.
    fn shutdown(&mut self) -> CoreResult<()> {
        Ok(())
    }

    /// Check if the component is running.
    fn is_running(&self) -> bool {
        false
    }
}
