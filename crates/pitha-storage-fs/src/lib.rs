use std::path::PathBuf;

use async_trait::async_trait;
use bytes::Bytes;
use pitha_storage_core::error::StorageError;
use pitha_storage_core::traits::BlobStore;

/// Filesystem-based blob store.
pub struct FsBlobStore {
    root: PathBuf,
}

impl FsBlobStore {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    fn resolve_path(&self, path: &str) -> PathBuf {
        self.root.join(path)
    }
}

#[async_trait]
impl BlobStore for FsBlobStore {
    async fn read(&self, path: &str) -> Result<Option<Bytes>, StorageError> {
        let full_path = self.resolve_path(path);
        if !full_path.exists() {
            return Ok(None);
        }
        let data = tokio::fs::read(&full_path).await?;
        Ok(Some(Bytes::from(data)))
    }

    async fn write(&self, path: &str, data: Bytes) -> Result<(), StorageError> {
        let full_path = self.resolve_path(path);
        if let Some(parent) = full_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }
        tokio::fs::write(&full_path, data.as_ref()).await?;
        Ok(())
    }

    async fn delete(&self, path: &str) -> Result<(), StorageError> {
        let full_path = self.resolve_path(path);
        if full_path.exists() {
            tokio::fs::remove_file(&full_path).await?;
        }
        Ok(())
    }

    async fn exists(&self, path: &str) -> Result<bool, StorageError> {
        let full_path = self.resolve_path(path);
        Ok(full_path.exists())
    }

    async fn list(&self, prefix: Option<&str>) -> Result<Vec<String>, StorageError> {
        let dir = match prefix {
            Some(p) => self.root.join(p),
            None => self.root.clone(),
        };

        if !dir.exists() {
            return Ok(Vec::new());
        }

        let mut entries = Vec::new();
        let mut read_dir = tokio::fs::read_dir(&dir).await?;

        while let Some(entry) = read_dir.next_entry().await? {
            let name = entry.file_name().to_string_lossy().to_string();
            entries.push(name);
        }

        Ok(entries)
    }
}
