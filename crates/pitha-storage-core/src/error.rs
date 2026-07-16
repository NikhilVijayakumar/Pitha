use pitha_core::CoreError;

/// Storage-specific error type.
#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("not found: {0}")]
    NotFound(String),

    #[error("already exists: {0}")]
    AlreadyExists(String),

    #[error("permission denied: {0}")]
    PermissionDenied(String),

    #[error("io error: {0}")]
    Io(String),

    #[error("serialization error: {0}")]
    Serialization(String),

    #[error("storage error: {0}")]
    Other(String),
}

impl From<StorageError> for CoreError {
    fn from(e: StorageError) -> Self {
        CoreError::storage(e.to_string())
    }
}

impl From<std::io::Error> for StorageError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e.to_string())
    }
}
