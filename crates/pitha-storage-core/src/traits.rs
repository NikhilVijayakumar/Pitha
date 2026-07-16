use async_trait::async_trait;
use bytes::Bytes;

use crate::error::StorageError;

/// Key-value store trait for structured data.
#[async_trait]
pub trait KeyValueStore: Send + Sync {
    /// Get a value by key.
    async fn get(&self, key: &str) -> Result<Option<Bytes>, StorageError>;

    /// Set a value by key.
    async fn set(&self, key: &str, value: Bytes) -> Result<(), StorageError>;

    /// Delete a value by key.
    async fn delete(&self, key: &str) -> Result<(), StorageError>;

    /// Check if a key exists.
    async fn exists(&self, key: &str) -> Result<bool, StorageError>;

    /// List all keys with an optional prefix.
    async fn list(&self, prefix: Option<&str>) -> Result<Vec<String>, StorageError>;
}

/// Blob store trait for binary data.
#[async_trait]
pub trait BlobStore: Send + Sync {
    /// Read a blob by path.
    async fn read(&self, path: &str) -> Result<Option<Bytes>, StorageError>;

    /// Write a blob to a path.
    async fn write(&self, path: &str, data: Bytes) -> Result<(), StorageError>;

    /// Delete a blob by path.
    async fn delete(&self, path: &str) -> Result<(), StorageError>;

    /// Check if a blob exists.
    async fn exists(&self, path: &str) -> Result<bool, StorageError>;

    /// List blobs with an optional prefix.
    async fn list(&self, prefix: Option<&str>) -> Result<Vec<String>, StorageError>;
}
