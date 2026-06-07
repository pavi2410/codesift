//! Persistence layer for codesift indexes.

mod meta;
mod query_store;
mod records;
mod snapshot;
mod store;

pub use meta::{IndexMeta, StackVersions};
pub use records::{
    EdgeRecord, FileStateRecord, RefKind, RefRecord, Relation, SiteLocation, SymbolRecord,
    make_edge_id, make_symbol_id,
};
pub use query_store::QueryStore;
pub use snapshot::SnapshotStore;
pub use store::{IndexStore, make_unresolved_id};

pub const RECORD_VERSION: u16 = 1;
pub const INDEX_FORMAT_VERSION: u32 = 1;
