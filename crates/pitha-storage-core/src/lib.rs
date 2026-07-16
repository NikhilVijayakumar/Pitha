pub mod error;
pub mod traits;

pub use error::StorageError;
pub use traits::{BlobStore, KeyValueStore};
