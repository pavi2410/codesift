//! Foundation types, workspace, and virtual file system for codesift.

mod error;
mod types;
mod vfs;
mod workspace;

pub use error::{Error, Result};
pub use types::{Language, Location, Span, SymbolKind, Visibility};
pub use vfs::{FileEntry, discover_files, hash_file};
pub use workspace::Workspace;
