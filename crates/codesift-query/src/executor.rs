use std::time::Instant;

use codesift_core::Result;
use codesift_store::{IndexStore, SymbolRecord};
use globset::Glob;

use crate::parser::StructuralQuery;
use crate::response::{QueryError, QueryHit, QueryResponse};

pub struct QueryExecutor<'a> {
    store: &'a IndexStore,
}

impl<'a> QueryExecutor<'a> {
    pub fn new(store: &'a IndexStore) -> Self {
        Self { store }
    }

    pub fn execute(&self, query: &StructuralQuery) -> Result<QueryResponse> {
        let started = Instant::now();

        if let Some(to_id) = &query.refs_to {
            return self.execute_refs(query, to_id, started);
        }

        if let Some(to_id) = &query.callers_of {
            return self.execute_callers(query, to_id, started);
        }

        let mut hits = self.filter_symbols(query)?;
        hits.sort_by(|a, b| a.symbol.name.cmp(&b.symbol.name));

        let total = hits.len();
        Ok(QueryResponse {
            query: query.raw.clone(),
            workspace_rev: self.store.meta.workspace_rev,
            took_ms: started.elapsed().as_millis() as u64,
            hits,
            total,
            error: None,
        })
    }

    fn filter_symbols(&self, query: &StructuralQuery) -> Result<Vec<QueryHit>> {
        let mut symbols = if let Some(name) = &query.symbol_name {
            if name.contains('*') {
                self.store.all_symbols()?
            } else {
                self.store.lookup_by_name(name)?
            }
        } else if let Some(path) = &query.path {
            self.store.lookup_by_path(path)?
        } else if let Some(kind) = &query.kind {
            self.store.lookup_by_kind(kind)?
        } else {
            self.store.all_symbols()?
        };

        if let Some(name) = &query.symbol_name {
            symbols.retain(|s| glob_match(name, &s.name));
        }

        if let Some(kind) = &query.kind {
            symbols.retain(|s| s.kind.as_str() == kind);
        }

        if let Some(path_glob) = &query.path {
            symbols.retain(|s| glob_match(path_glob, &s.path));
        }

        Ok(symbols.into_iter().map(symbol_hit).collect())
    }

    fn execute_refs(
        &self,
        query: &StructuralQuery,
        to_id: &str,
        started: Instant,
    ) -> Result<QueryResponse> {
        let refs = self.store.refs_to(to_id)?;
        let mut hits = Vec::new();
        for reference in refs {
            if let Some(symbol) = self.store.get_symbol(&reference.from_id)? {
                hits.push(symbol_hit(symbol));
            }
        }
        let total = hits.len();
        Ok(QueryResponse {
            query: query.raw.clone(),
            workspace_rev: self.store.meta.workspace_rev,
            took_ms: started.elapsed().as_millis() as u64,
            hits,
            total,
            error: None,
        })
    }

    fn execute_callers(
        &self,
        query: &StructuralQuery,
        to_id: &str,
        started: Instant,
    ) -> Result<QueryResponse> {
        let edges = self.store.callers_of(to_id)?;
        let mut hits = Vec::new();
        for edge in edges {
            if let Some(symbol) = self.store.get_symbol(&edge.from_id)? {
                hits.push(symbol_hit(symbol));
            }
        }
        let total = hits.len();
        Ok(QueryResponse {
            query: query.raw.clone(),
            workspace_rev: self.store.meta.workspace_rev,
            took_ms: started.elapsed().as_millis() as u64,
            hits,
            total,
            error: None,
        })
    }

    pub fn lookup_symbol(&self, id: &str) -> Result<Option<SymbolRecord>> {
        self.store.get_symbol(id)
    }

    pub fn lookup_by_name(
        &self,
        name: &str,
        path_prefix: Option<&str>,
    ) -> Result<Vec<SymbolRecord>> {
        let mut symbols = self.store.lookup_by_name(name)?;
        if let Some(prefix) = path_prefix {
            symbols.retain(|s| s.path.starts_with(prefix));
        }
        Ok(symbols)
    }
}

fn symbol_hit(symbol: SymbolRecord) -> QueryHit {
    QueryHit { symbol, score: 1.0 }
}

fn glob_match(pattern: &str, value: &str) -> bool {
    if !pattern.contains('*') && !pattern.contains('?') {
        return pattern.eq_ignore_ascii_case(value);
    }
    match Glob::new(pattern) {
        Ok(g) => g.compile_matcher().is_match(value),
        Err(_) => value.contains(pattern.trim_matches('*')),
    }
}

pub fn invalid_query(raw: &str, message: impl Into<String>) -> QueryResponse {
    QueryResponse {
        query: raw.to_string(),
        workspace_rev: 0,
        took_ms: 0,
        hits: Vec::new(),
        total: 0,
        error: Some(QueryError {
            code: "INVALID_QUERY".to_string(),
            message: message.into(),
            query: raw.to_string(),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_query;
    use codesift_core::{Language, Location, Span, SymbolKind};
    use codesift_store::{IndexStore, RECORD_VERSION};
    use tempfile::tempdir;

    fn sample(id: &str, name: &str, kind: SymbolKind) -> SymbolRecord {
        SymbolRecord {
            id: id.to_string(),
            kind,
            name: name.to_string(),
            qualified_name: None,
            path: "src/lib.rs".to_string(),
            language: Language::Rust,
            location: Location::new(
                "src/lib.rs",
                Language::Rust,
                Span {
                    start_byte: 0,
                    end_byte: 10,
                    start_line: 0,
                    start_column: 0,
                    end_line: 0,
                    end_column: 10,
                },
            ),
            visibility: None,
            signature: None,
            doc_comment: None,
            parent_id: None,
            workspace_rev: 1,
            record_version: RECORD_VERSION,
        }
    }

    #[test]
    fn filters_by_name_and_kind() {
        let dir = tempdir().unwrap();
        let index_dir = dir.path().join(".codesift");
        let index_dir = camino::Utf8PathBuf::try_from(index_dir).unwrap();
        let mut store = IndexStore::open(&index_dir).unwrap();
        store
            .put_symbol(&sample(
                "sym://1/src/lib.rs#function:hello@0:10",
                "hello",
                SymbolKind::Function,
            ))
            .unwrap();
        store
            .put_symbol(&sample(
                "sym://1/src/lib.rs#type:Hello@0:10",
                "Hello",
                SymbolKind::Type,
            ))
            .unwrap();
        store.commit().unwrap();

        let query = parse_query("symbol:name=hello kind=function").unwrap();
        let response = QueryExecutor::new(&store).execute(&query).unwrap();
        assert_eq!(response.total, 1);
        assert_eq!(response.hits[0].symbol.name, "hello");
    }
}
