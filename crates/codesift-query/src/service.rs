//! High-level code intelligence API for CLI and MCP.

use camino::Utf8PathBuf;
use codesift_core::{Error, Result, Workspace};
use codesift_store::{IndexStore, SymbolRecord};

use crate::executor::QueryExecutor;
use crate::parser::StructuralQuery;
use crate::response::{QueryHit, StatusResponse};
use crate::validate::parse_query_with_hints;

pub struct CodeIntel {
    store: IndexStore,
    index_path: Utf8PathBuf,
}

impl CodeIntel {
    pub fn open(workspace: &Workspace) -> Result<Self> {
        if !workspace.index_dir.exists() {
            return Err(Error::message(
                "index not found; run `codesift index` first",
            ));
        }
        let store = IndexStore::open(&workspace.index_dir)?;
        Ok(Self {
            index_path: workspace.index_dir.clone(),
            store,
        })
    }

    pub fn store(&self) -> &IndexStore {
        &self.store
    }

    pub fn index_status(&self) -> StatusResponse {
        StatusResponse {
            workspace_rev: self.store.meta.workspace_rev,
            index_format_version: self.store.meta.index_format_version,
            files: self.store.meta.file_count,
            symbols: self.store.meta.symbol_count,
            semantic_ready: false,
            last_indexed: self.store.meta.updated_at.to_rfc3339(),
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
        let mut symbols = if let Some(id) = symbol_id {
            self.store
                .get_symbol(id)?
                .into_iter()
                .collect::<Vec<_>>()
        } else if let Some(name) = name {
            QueryExecutor::new(&self.store).lookup_by_name(name, path_prefix)?
        } else {
            return Err(Error::message("provide name or symbol_id"));
        };

        if let Some(kind) = kind {
            symbols.retain(|s| s.kind.as_str() == kind);
        }
        Ok(symbols)
    }

    pub fn find_references_by_name(&self, name: &str) -> Result<Vec<QueryHit>> {
        QueryExecutor::new(&self.store).refs_by_name(name)
    }

    pub fn find_references_to_id(&self, symbol_id: &str) -> Result<Vec<QueryHit>> {
        let query = parse_query_with_hints(&format!("refs:to={symbol_id}"))?;
        Ok(QueryExecutor::new(&self.store).execute(&query)?.hits)
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
        let mut hits = self.find_references_by_name(name)?;
        if let Some(kind) = kind {
            hits.retain(|h| {
                self.store
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
        Ok(QueryExecutor::new(&self.store).execute(&query)?.hits)
    }

    pub fn execute_query(&self, query: &StructuralQuery) -> Result<crate::response::QueryResponse> {
        QueryExecutor::new(&self.store).execute(query)
    }
}
