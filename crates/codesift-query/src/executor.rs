use std::collections::{HashSet, VecDeque};
use std::time::Instant;

use codesift_core::Result;
use codesift_store::{QueryStore, RefKind, SymbolRecord};

use crate::parser::StructuralQuery;
use crate::response::{QueryError, QueryHit, QueryResponse};

pub struct QueryExecutor<'a, S: QueryStore + ?Sized> {
    store: &'a S,
}

impl<'a, S: QueryStore + ?Sized> QueryExecutor<'a, S> {
    pub fn new(store: &'a S) -> Self {
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
        hits.sort_by(|a, b| {
            a.symbol
                .name
                .cmp(&b.symbol.name)
                .then(a.symbol.path.cmp(&b.symbol.path))
        });

        let total = hits.len();
        Ok(QueryResponse {
            query: query.raw.clone(),
            workspace_rev: self.store.meta().workspace_rev,
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
            if path.contains('*') || path.contains('?') {
                self.store.all_symbols()?
            } else {
                self.store.lookup_by_path(path)?
            }
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

        if let Some(lang) = &query.lang {
            symbols.retain(|s| s.language.as_str() == lang);
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
                hits.push(QueryHit {
                    symbol,
                    score: 1.0,
                    site: Some(reference.site),
                    ref_kind: Some(reference.ref_kind),
                    depth: None,
                });
            }
        }
        hits.sort_by(|a, b| {
            a.symbol
                .path
                .cmp(&b.symbol.path)
                .then(a.site.as_ref().map(|s| s.start_line).cmp(&b.site.as_ref().map(|s| s.start_line)))
        });
        let total = hits.len();
        Ok(QueryResponse {
            query: query.raw.clone(),
            workspace_rev: self.store.meta().workspace_rev,
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
        let max_depth = query.depth.unwrap_or(1);
        let mut hits = Vec::new();
        let mut seen = HashSet::new();
        let mut queue: VecDeque<(String, u32)> = VecDeque::new();
        queue.push_back((to_id.to_string(), 0));

        while let Some((current_id, depth)) = queue.pop_front() {
            if depth >= max_depth {
                continue;
            }
            for edge in self.store.callers_of(&current_id)? {
                let next_depth = depth + 1;
                if !seen.insert((edge.from_id.clone(), next_depth)) {
                    continue;
                }
                if let Some(symbol) = self.store.get_symbol(&edge.from_id)? {
                    hits.push(QueryHit {
                        symbol,
                        score: 1.0,
                        site: edge.call_site.clone(),
                        ref_kind: Some(RefKind::Call),
                        depth: Some(next_depth),
                    });
                }
                if next_depth < max_depth {
                    queue.push_back((edge.from_id.clone(), next_depth));
                }
            }
        }

        hits.sort_by(|a, b| {
            a.depth
                .cmp(&b.depth)
                .then(a.symbol.path.cmp(&b.symbol.path))
                .then(a.site.as_ref().map(|s| s.start_line).cmp(&b.site.as_ref().map(|s| s.start_line)))
        });
        let total = hits.len();
        Ok(QueryResponse {
            query: query.raw.clone(),
            workspace_rev: self.store.meta().workspace_rev,
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

    pub fn refs_by_name(&self, name: &str) -> Result<Vec<QueryHit>> {
        let refs = self.store.refs_to_name(name)?;
        let mut hits = Vec::new();
        for reference in refs {
            if let Some(symbol) = self.store.get_symbol(&reference.from_id)? {
                hits.push(QueryHit {
                    symbol,
                    score: 1.0,
                    site: Some(reference.site),
                    ref_kind: Some(reference.ref_kind),
                    depth: None,
                });
            }
        }
        hits.sort_by(|a, b| {
            a.symbol
                .path
                .cmp(&b.symbol.path)
                .then(a.site.as_ref().map(|s| s.start_line).cmp(&b.site.as_ref().map(|s| s.start_line)))
        });
        Ok(hits)
    }
}

fn symbol_hit(symbol: SymbolRecord) -> QueryHit {
    QueryHit {
        symbol,
        score: 1.0,
        site: None,
        ref_kind: None,
        depth: None,
    }
}

fn glob_match(pattern: &str, value: &str) -> bool {
    if !pattern.contains('*') && !pattern.contains('?') {
        return pattern.eq_ignore_ascii_case(value) || value.ends_with(pattern);
    }
    match globset::Glob::new(pattern) {
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
    use codesift_store::{IndexStore, RECORD_VERSION, RefKind, RefRecord, SiteLocation};
    use tempfile::tempdir;

    fn sample(id: &str, name: &str, kind: SymbolKind, path: &str) -> SymbolRecord {
        SymbolRecord {
            id: id.to_string(),
            kind,
            name: name.to_string(),
            qualified_name: None,
            path: path.to_string(),
            language: Language::Rust,
            location: Location::new(
                path,
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
                "src/lib.rs",
            ))
            .unwrap();
        store
            .put_symbol(&sample(
                "sym://1/src/lib.rs#type:Hello@0:10",
                "Hello",
                SymbolKind::Type,
                "src/lib.rs",
            ))
            .unwrap();
        store.commit().unwrap();

        let query = parse_query("symbol:name=hello kind=function").unwrap();
        let response = QueryExecutor::new(&store).execute(&query).unwrap();
        assert_eq!(response.total, 1);
        assert_eq!(response.hits[0].symbol.name, "hello");
    }

    #[test]
    fn path_glob_filter_matches_nested_paths() {
        let dir = tempdir().unwrap();
        let index_dir = camino::Utf8PathBuf::from_path_buf(dir.path().join(".codesift")).unwrap();
        let mut store = IndexStore::open(&index_dir).unwrap();
        store
            .put_symbol(&sample(
                "sym://1/crates/query/src/parser.rs#function:parse_query@0:10",
                "parse_query",
                SymbolKind::Function,
                "crates/query/src/parser.rs",
            ))
            .unwrap();
        store
            .put_symbol(&sample(
                "sym://1/crates/index/src/lib.rs#function:index@0:10",
                "index",
                SymbolKind::Function,
                "crates/index/src/lib.rs",
            ))
            .unwrap();
        store.commit().unwrap();

        let query = parse_query("path=**/parser.rs kind=function").unwrap();
        let response = QueryExecutor::new(&store).execute(&query).unwrap();
        assert_eq!(response.total, 1);
        assert_eq!(response.hits[0].symbol.name, "parse_query");
    }

    #[test]
    fn refs_include_unresolved_targets_for_same_name() {
        let dir = tempdir().unwrap();
        let index_dir = camino::Utf8PathBuf::from_path_buf(dir.path().join(".codesift")).unwrap();
        let mut store = IndexStore::open(&index_dir).unwrap();
        let target = "sym://1/crates/query/src/parser.rs#function:parse_query@0:10";
        let caller = "sym://1/crates/cli/src/main.rs#function:run@0:10";
        store
            .put_symbol(&sample(
                target,
                "parse_query",
                SymbolKind::Function,
                "crates/query/src/parser.rs",
            ))
            .unwrap();
        store
            .put_symbol(&sample(
                caller,
                "run",
                SymbolKind::Function,
                "crates/cli/src/main.rs",
            ))
            .unwrap();
        let unresolved = "sym://1/unresolved#function:parse_query@0:0";
        store
            .put_ref(&RefRecord {
                from_id: caller.to_string(),
                to_id: unresolved.to_string(),
                ref_kind: RefKind::Call,
                site: SiteLocation {
                    path: "crates/cli/src/main.rs".to_string(),
                    start_byte: 100,
                    end_byte: 110,
                    start_line: 10,
                    start_column: 4,
                },
                workspace_rev: 1,
                record_version: RECORD_VERSION,
            })
            .unwrap();
        store.commit().unwrap();

        let query = parse_query(&format!("refs:to={target}")).unwrap();
        let response = QueryExecutor::new(&store).execute(&query).unwrap();
        assert_eq!(response.total, 1);
        assert_eq!(response.hits[0].symbol.name, "run");
        assert_eq!(response.hits[0].site.as_ref().unwrap().start_line, 10);
    }

    #[test]
    fn callers_respect_depth() {
        let dir = tempdir().unwrap();
        let index_dir = camino::Utf8PathBuf::from_path_buf(dir.path().join(".codesift")).unwrap();
        let mut store = IndexStore::open(&index_dir).unwrap();
        let main_id = "sym://1/src/main.rs#function:main@0:10";
        let run_id = "sym://1/src/main.rs#function:run@0:10";
        let parse_id = "sym://1/src/parser.rs#function:parse_query@0:10";
        for (id, name, path) in [
            (main_id, "main", "src/main.rs"),
            (run_id, "run", "src/main.rs"),
            (parse_id, "parse_query", "src/parser.rs"),
        ] {
            store
                .put_symbol(&sample(id, name, SymbolKind::Function, path))
                .unwrap();
        }
        use codesift_store::{EdgeRecord, Relation, RECORD_VERSION as RV};
        for (from, to) in [(run_id, parse_id), (main_id, run_id)] {
            let edge_id = store.next_edge_id(1, Relation::Calls);
            store
                .put_edge(&EdgeRecord {
                    id: edge_id,
                    relation: Relation::Calls,
                    from_id: from.to_string(),
                    to_id: to.to_string(),
                    call_site: Some(SiteLocation {
                        path: "src/main.rs".to_string(),
                        start_byte: 0,
                        end_byte: 1,
                        start_line: 1,
                        start_column: 0,
                    }),
                    resolved: true,
                    workspace_rev: 1,
                    record_version: RV,
                })
                .unwrap();
        }
        store.commit().unwrap();

        let query = parse_query(&format!("callers:of={parse_id} depth=2")).unwrap();
        let response = QueryExecutor::new(&store).execute(&query).unwrap();
        let names: Vec<_> = response
            .hits
            .iter()
            .map(|h| h.symbol.name.as_str())
            .collect();
        assert!(names.contains(&"run"));
        assert!(names.contains(&"main"));
    }
}
