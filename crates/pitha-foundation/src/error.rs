use pitha_core::CoreError;

/// Foundation-specific error type.
#[derive(Debug, thiserror::Error)]
pub enum FoundationError {
    #[error("configuration error: {0}")]
    Configuration(String),

    #[error("initialization error: {0}")]
    Initialization(String),

    #[error("lifecycle error: {0}")]
    Lifecycle(String),

    #[error("state error: {0}")]
    State(String),

    #[error("logging error: {0}")]
    Logging(String),
}

impl From<FoundationError> for CoreError {
    fn from(e: FoundationError) -> Self {
        match e {
            FoundationError::Configuration(msg) => CoreError::Configuration(msg),
            FoundationError::Initialization(msg) => CoreError::Initialization(msg),
            FoundationError::Lifecycle(msg) => CoreError::Lifecycle(msg),
            FoundationError::State(msg) => CoreError::Internal(msg),
            FoundationError::Logging(msg) => CoreError::Internal(msg),
        }
    }
}

impl FoundationError {
    pub fn configuration(msg: impl Into<String>) -> Self {
        Self::Configuration(msg.into())
    }

    pub fn initialization(msg: impl Into<String>) -> Self {
        Self::Initialization(msg.into())
    }

    pub fn lifecycle(msg: impl Into<String>) -> Self {
        Self::Lifecycle(msg.into())
    }

    pub fn state(msg: impl Into<String>) -> Self {
        Self::State(msg.into())
    }

    pub fn logging(msg: impl Into<String>) -> Self {
        Self::Logging(msg.into())
    }
}
