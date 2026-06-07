//! Query store trait for fjall-backed and snapshot-backed indexes.

use codesift_core::Result as CoreResult;

use crate::meta::IndexMeta;
use crate::records::{EdgeRecord, RefRecord, SymbolRecord};
use crate::snapshot::SnapshotStore;
use crate::store::IndexStore;

pub trait QueryStore {
    fn meta(&self) -> &IndexMeta;
    fn get_symbol(&self, id: &str) -> CoreResult<Option<SymbolRecord>>;
    fn lookup_by_name(&self, name: &str) -> CoreResult<Vec<SymbolRecord>>;
    fn lookup_by_path(&self, path: &str) -> CoreResult<Vec<SymbolRecord>>;
    fn lookup_by_kind(&self, kind: &str) -> CoreResult<Vec<SymbolRecord>>;
    fn refs_to(&self, to_id: &str) -> CoreResult<Vec<RefRecord>>;
    fn refs_to_name(&self, name: &str) -> CoreResult<Vec<RefRecord>>;
    fn callers_of(&self, to_id: &str) -> CoreResult<Vec<EdgeRecord>>;
    fn all_symbols(&self) -> CoreResult<Vec<SymbolRecord>>;
}

impl QueryStore for IndexStore {
    fn meta(&self) -> &IndexMeta {
        &self.meta
    }

    fn get_symbol(&self, id: &str) -> CoreResult<Option<SymbolRecord>> {
        self.get_symbol(id)
    }

    fn lookup_by_name(&self, name: &str) -> CoreResult<Vec<SymbolRecord>> {
        self.lookup_by_name(name)
    }

    fn lookup_by_path(&self, path: &str) -> CoreResult<Vec<SymbolRecord>> {
        self.lookup_by_path(path)
    }

    fn lookup_by_kind(&self, kind: &str) -> CoreResult<Vec<SymbolRecord>> {
        self.lookup_by_kind(kind)
    }

    fn refs_to(&self, to_id: &str) -> CoreResult<Vec<RefRecord>> {
        self.refs_to(to_id)
    }

    fn refs_to_name(&self, name: &str) -> CoreResult<Vec<RefRecord>> {
        self.refs_to_name(name)
    }

    fn callers_of(&self, to_id: &str) -> CoreResult<Vec<EdgeRecord>> {
        self.callers_of(to_id)
    }

    fn all_symbols(&self) -> CoreResult<Vec<SymbolRecord>> {
        self.all_symbols()
    }
}

impl QueryStore for SnapshotStore {
    fn meta(&self) -> &IndexMeta {
        &self.meta
    }

    fn get_symbol(&self, id: &str) -> CoreResult<Option<SymbolRecord>> {
        self.get_symbol(id)
    }

    fn lookup_by_name(&self, name: &str) -> CoreResult<Vec<SymbolRecord>> {
        self.lookup_by_name(name)
    }

    fn lookup_by_path(&self, path: &str) -> CoreResult<Vec<SymbolRecord>> {
        self.lookup_by_path(path)
    }

    fn lookup_by_kind(&self, kind: &str) -> CoreResult<Vec<SymbolRecord>> {
        self.lookup_by_kind(kind)
    }

    fn refs_to(&self, to_id: &str) -> CoreResult<Vec<RefRecord>> {
        self.refs_to(to_id)
    }

    fn refs_to_name(&self, name: &str) -> CoreResult<Vec<RefRecord>> {
        self.refs_to_name(name)
    }

    fn callers_of(&self, to_id: &str) -> CoreResult<Vec<EdgeRecord>> {
        self.callers_of(to_id)
    }

    fn all_symbols(&self) -> CoreResult<Vec<SymbolRecord>> {
        self.all_symbols()
    }
}
