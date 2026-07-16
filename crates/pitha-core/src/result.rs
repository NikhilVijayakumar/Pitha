use crate::error::CoreError;

/// Platform result type alias.
pub type CoreResult<T> = Result<T, CoreError>;

/// Execute a fallible operation, converting errors to [`CoreError`].
pub fn fallible<T>(result: Result<T, impl std::fmt::Display>) -> CoreResult<T> {
    result.map_err(|e| CoreError::Internal(e.to_string()))
}
