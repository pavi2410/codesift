use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StackVersions {
    pub fjall: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IndexMeta {
    pub index_format_version: u32,
    pub workspace_rev: u64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub stack: StackVersions,
    pub file_count: u64,
    pub symbol_count: u64,
}

impl IndexMeta {
    pub fn new() -> Self {
        let now = Utc::now();
        Self {
            index_format_version: crate::INDEX_FORMAT_VERSION,
            workspace_rev: 1,
            created_at: now,
            updated_at: now,
            stack: StackVersions {
                fjall: env!("CARGO_PKG_VERSION").to_string(),
            },
            file_count: 0,
            symbol_count: 0,
        }
    }
}

impl Default for IndexMeta {
    fn default() -> Self {
        Self::new()
    }
}
