use chrono::Utc;
use codesift_core::{Language, Location, Result, Span, Workspace, discover_files};
use codesift_parse::{PsiProvider, RustPsiProvider};
use codesift_store::{
    EdgeRecord, FileStateRecord, IndexStore, RECORD_VERSION, RefKind, RefRecord, Relation,
    SiteLocation, SymbolRecord, make_symbol_id, make_unresolved_id,
};
use std::collections::HashMap;
use std::fs;
use std::time::Instant;
use tracing::info;

#[derive(Debug, Clone)]
pub struct IndexOptions {
    pub force: bool,
    pub jobs: usize,
}

impl Default for IndexOptions {
    fn default() -> Self {
        Self {
            force: false,
            jobs: std::thread::available_parallelism()
                .map(|n| n.get())
                .unwrap_or(1),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct IndexReport {
    pub status: String,
    pub files_indexed: u64,
    pub symbols: u64,
    pub duration_ms: u64,
    pub workspace_rev: u64,
}

pub struct Indexer;

impl Indexer {
    pub fn index(workspace: &Workspace, options: &IndexOptions) -> Result<IndexReport> {
        let started = Instant::now();
        let mut store = IndexStore::open(&workspace.index_dir)?;

        if options.force {
            store.meta.workspace_rev += 1;
            store.clear_graph()?;
        }

        let workspace_rev = store.meta.workspace_rev;
        let files = discover_files(workspace, true)?;
        let file_count = files.len() as u64;
        let provider =
            RustPsiProvider::for_path(&workspace.root.join("Cargo.toml"), &workspace.root);

        let mut files_indexed = 0u64;
        let mut changed = false;

        for file in files {
            if !options.force
                && let Some(state) = store.get_file_state(&file.path)?
                && state.content_hash == file.content_hash
            {
                continue;
            }

            changed = true;
            store.remove_file_symbols(&file.path)?;

            let source =
                fs::read_to_string(&file.absolute).map_err(|source| codesift_core::Error::Io {
                    path: file.absolute.to_string(),
                    source,
                })?;

            let parse = provider.parse_file(&source, &file.absolute);
            let mut symbol_ids = Vec::new();
            let mut name_to_id: HashMap<String, String> = HashMap::new();

            for draft in parse.symbols {
                let id = make_symbol_id(
                    workspace_rev,
                    &file.path,
                    draft.kind,
                    &draft.name,
                    draft.start_byte,
                    draft.end_byte,
                );

                let parent_id = draft
                    .parent_name
                    .as_ref()
                    .and_then(|p| name_to_id.get(p).cloned());

                let record = SymbolRecord {
                    id: id.clone(),
                    kind: draft.kind,
                    name: draft.name.clone(),
                    qualified_name: draft.qualified_name,
                    path: file.path.clone(),
                    language: Language::Rust,
                    location: Location::new(
                        &file.path,
                        Language::Rust,
                        Span {
                            start_byte: draft.start_byte,
                            end_byte: draft.end_byte,
                            start_line: draft.start_line,
                            start_column: draft.start_column,
                            end_line: draft.end_line,
                            end_column: draft.end_column,
                        },
                    ),
                    visibility: draft.visibility,
                    signature: draft.signature,
                    doc_comment: draft.doc_comment,
                    parent_id,
                    workspace_rev,
                    record_version: RECORD_VERSION,
                };

                if let Some(q) = &record.qualified_name {
                    name_to_id.insert(q.clone(), id.clone());
                }
                name_to_id.insert(draft.name.clone(), id.clone());

                store.put_symbol(&record)?;
                symbol_ids.push(id);
            }

            for call in parse.calls {
                let from_id = name_to_id.get(&call.caller_name).cloned().or_else(|| {
                    store
                        .lookup_by_name(last_segment(&call.caller_name))
                        .ok()
                        .and_then(|v| v.first().map(|s| s.id.clone()))
                });

                let to_id = resolve_callee(&store, &name_to_id, &call.callee_name, workspace_rev);

                if let Some(from_id) = from_id {
                    let edge_id = store.next_edge_id(workspace_rev, Relation::Calls);
                    let edge = EdgeRecord {
                        id: edge_id.clone(),
                        relation: Relation::Calls,
                        from_id: from_id.clone(),
                        to_id: to_id.clone(),
                        call_site: Some(SiteLocation {
                            path: file.path.clone(),
                            start_byte: call.start_byte,
                            end_byte: call.end_byte,
                            start_line: call.start_line,
                            start_column: call.start_column,
                        }),
                        resolved: !to_id.contains("/unresolved#"),
                        workspace_rev,
                        record_version: RECORD_VERSION,
                    };
                    store.put_edge(&edge)?;

                    let ref_record = RefRecord {
                        from_id,
                        to_id,
                        ref_kind: RefKind::Call,
                        site: SiteLocation {
                            path: file.path.clone(),
                            start_byte: call.start_byte,
                            end_byte: call.end_byte,
                            start_line: call.start_line,
                            start_column: call.start_column,
                        },
                        workspace_rev,
                        record_version: RECORD_VERSION,
                    };
                    store.put_ref(&ref_record)?;
                }
            }

            for reference in parse.refs {
                let from_id = name_to_id.get(&reference.from_name).cloned();
                let to_id = store
                    .lookup_by_name(&reference.to_name)
                    .ok()
                    .and_then(|v| v.first().map(|s| s.id.clone()));

                if let (Some(from_id), Some(to_id)) = (from_id, to_id) {
                    let ref_record = RefRecord {
                        from_id,
                        to_id,
                        ref_kind: RefKind::Reference,
                        site: SiteLocation {
                            path: file.path.clone(),
                            start_byte: reference.start_byte,
                            end_byte: reference.end_byte,
                            start_line: reference.start_line,
                            start_column: reference.start_column,
                        },
                        workspace_rev,
                        record_version: RECORD_VERSION,
                    };
                    store.put_ref(&ref_record)?;
                }
            }

            let state = FileStateRecord {
                path: file.path.clone(),
                content_hash: file.content_hash.clone(),
                language: file.language,
                symbol_ids,
                indexed_at: Utc::now(),
                parse_errors: parse.errors,
            };
            store.put_file_state(&state)?;
            files_indexed += 1;
        }

        if changed {
            store.meta.workspace_rev = workspace_rev;
        }
        store.meta.file_count = file_count;
        store.meta.symbol_count = store.all_symbols()?.len() as u64;
        store.save_meta()?;
        store.commit()?;

        info!(
            files_indexed,
            symbols = store.meta.symbol_count,
            "index complete in {:?}",
            started.elapsed()
        );

        Ok(IndexReport {
            status: "complete".to_string(),
            files_indexed,
            symbols: store.meta.symbol_count,
            duration_ms: started.elapsed().as_millis() as u64,
            workspace_rev: store.meta.workspace_rev,
        })
    }
}

fn last_segment(name: &str) -> &str {
    name.rsplit("::").next().unwrap_or(name)
}

fn resolve_callee(
    store: &IndexStore,
    name_to_id: &HashMap<String, String>,
    callee_name: &str,
    workspace_rev: u64,
) -> String {
    if let Some(id) = name_to_id.get(callee_name) {
        return id.clone();
    }

    let last = last_segment(callee_name);
    if let Some(id) = name_to_id.get(last) {
        return id.clone();
    }

    if callee_name.contains("::")
        && let Ok(symbols) = store.all_symbols()
    {
        for sym in symbols {
            if sym.qualified_name.as_deref() == Some(callee_name) {
                return sym.id.clone();
            }
            if let Some(q) = &sym.qualified_name
                && (q.ends_with(&format!("::{callee_name}")) || q.ends_with(&format!("::{last}")))
            {
                return sym.id.clone();
            }
        }
    }

    if let Ok(symbols) = store.lookup_by_name(last) {
        if let Some(s) = symbols
            .iter()
            .find(|s| matches!(s.kind, codesift_core::SymbolKind::Function))
        {
            return s.id.clone();
        }
        if let Some(s) = symbols.first() {
            return s.id.clone();
        }
    }

    make_unresolved_id(workspace_rev, codesift_core::SymbolKind::Function, last)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn indexes_fixture_crate() {
        let dir = tempdir().unwrap();
        let root = dir.path();
        std::fs::create_dir_all(root.join("src")).unwrap();
        std::fs::write(
            root.join("Cargo.toml"),
            "[package]\nname = \"fixture\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
        )
        .unwrap();
        std::fs::write(
            root.join("src/main.rs"),
            "fn main() { helper(); }\nfn helper() {}\n",
        )
        .unwrap();

        let ws = Workspace::discover(root).unwrap();
        let report = Indexer::index(&ws, &IndexOptions::default()).unwrap();
        assert!(report.symbols > 0);

        let store = IndexStore::open(&ws.index_dir).unwrap();
        assert!(ws.index_dir.join("meta.json").exists());
        assert!(store.meta.symbol_count > 0);
    }
}
