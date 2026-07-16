mod error;
mod metadata;
mod result;
mod traits;
mod version;

pub use error::{CoreError, IntoCoreError};
pub use metadata::Metadata;
pub use result::{fallible, CoreResult};
pub use traits::{Identifiable, Lifecycle, Nameable, Validate};
pub use version::Version;
