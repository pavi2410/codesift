//! Persistence layer for codesift indexes.

mod meta;
mod records;
mod store;

pub use meta::{IndexMeta, StackVersions};
pub use records::{
    EdgeRecord, FileStateRecord, RefKind, RefRecord, Relation, SiteLocation, SymbolRecord,
    make_edge_id, make_symbol_id,
};
pub use store::IndexStore;

pub const RECORD_VERSION: u16 = 1;
pub const INDEX_FORMAT_VERSION: u32 = 1;
