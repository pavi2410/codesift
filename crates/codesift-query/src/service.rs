//! High-level code intelligence API for CLI and MCP.

use camino::Utf8PathBuf;
use codesift_core::{Error, Result, Workspace};
use codesift_store::{IndexStore, QueryStore, SnapshotStore, SymbolRecord};

use crate::executor::QueryExecutor;
use crate::parser::StructuralQuery;
use crate::response::{QueryHit, StatusResponse};
use crate::validate::parse_query_with_hints;

enum QueryBackend {
    Live(IndexStore),
    Snapshot(SnapshotStore),
}

impl QueryBackend {
    fn as_store(&self) -> &dyn QueryStore {
        match self {
            Self::Live(store) => store,
            Self::Snapshot(store) => store,
        }
    }
}

pub struct CodeIntel {
    backend: QueryBackend,
    index_path: Utf8PathBuf,
}

impl CodeIntel {
    /// Open for queries. Prefers lock-free `query.snap` when present (MCP-safe).
    pub fn open(workspace: &Workspace) -> Result<Self> {
        if !workspace.index_dir.exists() {
            return Err(Error::message(
                "index not found; run `codesift index` first",
            ));
        }

        let snap_path = SnapshotStore::snapshot_path(&workspace.index_dir);
        let backend = if snap_path.exists() {
            QueryBackend::Snapshot(SnapshotStore::load(&snap_path)?)
        } else {
            QueryBackend::Live(IndexStore::open_with_retry(&workspace.index_dir, 3)?)
        };

        Ok(Self {
            index_path: workspace.index_dir.clone(),
            backend,
        })
    }

    pub fn index_status(&self) -> StatusResponse {
        let meta = self.backend.as_store().meta();
        StatusResponse {
            workspace_rev: meta.workspace_rev,
            index_format_version: meta.index_format_version,
            files: meta.file_count,
            symbols: meta.symbol_count,
            semantic_ready: false,
            last_indexed: meta.updated_at.to_rfc3339(),
            index_path: self.index_path.to_string(),
        }
    }

    pub fn resolve_symbol(
        &self,
        name: Option<&str>,
        symbol_id: Option<&str>,
        kind: Option<&str>,
        path_prefix: Option<&str>,
    ) -> Result<Vec<SymbolRecord>> {
        let store = self.backend.as_store();
        let mut symbols = if let Some(id) = symbol_id {
            store.get_symbol(id)?.into_iter().collect::<Vec<_>>()
        } else if let Some(name) = name {
            QueryExecutor::new(store).lookup_by_name(name, path_prefix)?
        } else {
            return Err(Error::message("provide name or symbol_id"));
        };

        if let Some(kind) = kind {
            symbols.retain(|s| s.kind.as_str() == kind);
        }
        Ok(symbols)
    }

    pub fn find_references_by_name(&self, name: &str) -> Result<Vec<QueryHit>> {
        QueryExecutor::new(self.backend.as_store()).refs_by_name(name)
    }

    pub fn find_references_to_id(&self, symbol_id: &str) -> Result<Vec<QueryHit>> {
        let query = parse_query_with_hints(&format!("refs:to={symbol_id}"))?;
        Ok(QueryExecutor::new(self.backend.as_store())
            .execute(&query)?
            .hits)
    }

    pub fn find_references(
        &self,
        name: Option<&str>,
        symbol_id: Option<&str>,
        kind: Option<&str>,
    ) -> Result<Vec<QueryHit>> {
        if let Some(id) = symbol_id {
            return self.find_references_to_id(id);
        }
        let name = name.ok_or_else(|| Error::message("provide name or symbol_id"))?;
        let store = self.backend.as_store();
        let mut hits = self.find_references_by_name(name)?;
        if let Some(kind) = kind {
            hits.retain(|h| {
                store
                    .get_symbol(&h.symbol.id)
                    .ok()
                    .flatten()
                    .is_none_or(|s| s.kind.as_str() == kind)
            });
        }
        Ok(hits)
    }

    pub fn get_callers(
        &self,
        name: Option<&str>,
        symbol_id: Option<&str>,
        kind: Option<&str>,
        depth: u32,
    ) -> Result<Vec<QueryHit>> {
        let target = if let Some(id) = symbol_id {
            id.to_string()
        } else {
            let name = name.ok_or_else(|| Error::message("provide name or symbol_id"))?;
            let symbols = self.resolve_symbol(Some(name), None, kind, None)?;
            symbols
                .first()
                .map(|s| s.id.clone())
                .ok_or_else(|| Error::message(format!("symbol not found: {name}")))?
        };
        let query = parse_query_with_hints(&format!("callers:of={target} depth={depth}"))?;
        Ok(QueryExecutor::new(self.backend.as_store())
            .execute(&query)?
            .hits)
    }

    pub fn execute_query(&self, query: &StructuralQuery) -> Result<crate::response::QueryResponse> {
        QueryExecutor::new(self.backend.as_store()).execute(query)
    }
}
