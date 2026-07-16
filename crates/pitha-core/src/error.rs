use std::fmt;

/// Core error type for the Pīṭha platform.
///
/// All platform errors derive from this type to ensure consistent error handling.
#[derive(Debug, thiserror::Error)]
pub enum CoreError {
    #[error("configuration error: {0}")]
    Configuration(String),

    #[error("initialization error: {0}")]
    Initialization(String),

    #[error("validation error: {0}")]
    Validation(String),

    #[error("lifecycle error: {0}")]
    Lifecycle(String),

    #[error("storage error: {0}")]
    Storage(String),

    #[error("runtime error: {0}")]
    Runtime(String),

    #[error("internal error: {0}")]
    Internal(String),
}

impl CoreError {
    pub fn configuration(msg: impl Into<String>) -> Self {
        Self::Configuration(msg.into())
    }

    pub fn initialization(msg: impl Into<String>) -> Self {
        Self::Initialization(msg.into())
    }

    pub fn validation(msg: impl Into<String>) -> Self {
        Self::Validation(msg.into())
    }

    pub fn lifecycle(msg: impl Into<String>) -> Self {
        Self::Lifecycle(msg.into())
    }

    pub fn storage(msg: impl Into<String>) -> Self {
        Self::Storage(msg.into())
    }

    pub fn runtime(msg: impl Into<String>) -> Self {
        Self::Runtime(msg.into())
    }

    pub fn internal(msg: impl Into<String>) -> Self {
        Self::Internal(msg.into())
    }
}

/// Trait for converting external errors into [`CoreError`].
pub trait IntoCoreError {
    fn into_core_error(self) -> CoreError;
}

impl<T: fmt::Display> IntoCoreError for T {
    fn into_core_error(self) -> CoreError {
        CoreError::Internal(self.to_string())
    }
}
