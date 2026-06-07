use camino::Utf8Path;
use codesift_core::{Error as CoreError, Result as CoreResult};
use fjall::{Database, Keyspace, KeyspaceCreateOptions, PersistMode};
use serde::{Serialize, de::DeserializeOwned};

use crate::meta::IndexMeta;
use crate::records::{EdgeRecord, FileStateRecord, RefRecord, SymbolRecord};

pub struct IndexStore {
    pub meta: IndexMeta,
    meta_path: camino::Utf8PathBuf,
    db: Database,
    symbols: Keyspace,
    refs: Keyspace,
    edges: Keyspace,
    file_state: Keyspace,
    edge_counter: u64,
}

impl IndexStore {
    pub fn open(index_dir: &Utf8Path) -> CoreResult<Self> {
        std::fs::create_dir_all(index_dir).map_err(|source| CoreError::Io {
            path: index_dir.to_string(),
            source,
        })?;

        let meta_path = index_dir.join("meta.json");
        let meta = if meta_path.exists() {
            let text = std::fs::read_to_string(&meta_path).map_err(|source| CoreError::Io {
                path: meta_path.to_string(),
                source,
            })?;
            serde_json::from_str(&text).map_err(|e| CoreError::message(e.to_string()))?
        } else {
            IndexMeta::new()
        };

        let kv_path = index_dir.join("kv");
        std::fs::create_dir_all(&kv_path).map_err(|source| CoreError::Io {
            path: kv_path.to_string(),
            source,
        })?;

        let db = Database::builder(kv_path.as_std_path())
            .open()
            .map_err(|e| CoreError::message(e.to_string()))?;

        let symbols = open_keyspace(&db, "symbols")?;
        let refs = open_keyspace(&db, "refs")?;
        let edges = open_keyspace(&db, "edges")?;
        let file_state = open_keyspace(&db, "file_state")?;
        let edge_counter = count_edges(&edges);

        Ok(Self {
            meta,
            meta_path,
            db,
            symbols,
            refs,
            edges,
            file_state,
            edge_counter,
        })
    }

    pub fn save_meta(&mut self) -> CoreResult<()> {
        self.meta.updated_at = chrono::Utc::now();
        let text = serde_json::to_string_pretty(&self.meta)
            .map_err(|e| CoreError::message(e.to_string()))?;
        std::fs::write(&self.meta_path, text).map_err(|source| CoreError::Io {
            path: self.meta_path.to_string(),
            source,
        })
    }

    pub fn commit(&self) -> CoreResult<()> {
        self.db
            .persist(PersistMode::Buffer)
            .map_err(|e| CoreError::message(e.to_string()))
    }

    pub fn put_symbol(&mut self, symbol: &SymbolRecord) -> CoreResult<()> {
        put_postcard(&self.symbols, symbol.id.as_bytes(), symbol)?;
        append_index(
            &self.symbols,
            format!("name:{}", symbol.name.to_lowercase()),
            &symbol.id,
        )?;
        append_index(&self.symbols, format!("path:{}", symbol.path), &symbol.id)?;
        append_index(
            &self.symbols,
            format!("kind:{}", symbol.kind.as_str()),
            &symbol.id,
        )?;
        Ok(())
    }

    pub fn get_symbol(&self, id: &str) -> CoreResult<Option<SymbolRecord>> {
        get_postcard(&self.symbols, id.as_bytes())
    }

    pub fn lookup_by_name(&self, name: &str) -> CoreResult<Vec<SymbolRecord>> {
        self.lookup_index(&format!("name:{}", name.to_lowercase()))
    }

    pub fn lookup_by_path(&self, path: &str) -> CoreResult<Vec<SymbolRecord>> {
        self.lookup_index(&format!("path:{path}"))
    }

    pub fn lookup_by_kind(&self, kind: &str) -> CoreResult<Vec<SymbolRecord>> {
        self.lookup_index(&format!("kind:{kind}"))
    }

    fn lookup_index(&self, key: &str) -> CoreResult<Vec<SymbolRecord>> {
        let ids: Vec<String> = get_postcard(&self.symbols, key.as_bytes())?.unwrap_or_default();
        let mut out = Vec::new();
        for id in ids {
            if let Some(sym) = self.get_symbol(&id)? {
                out.push(sym);
            }
        }
        Ok(out)
    }

    pub fn put_ref(&self, record: &RefRecord) -> CoreResult<()> {
        let key = format!("ref:{}:{}", record.from_id, record.to_id);
        put_postcard(&self.refs, key.as_bytes(), record)
    }

    pub fn refs_to(&self, to_id: &str) -> CoreResult<Vec<RefRecord>> {
        let mut out = Vec::new();
        for guard in self.refs.iter() {
            let (key, value) = guard
                .into_inner()
                .map_err(|e| CoreError::message(e.to_string()))?;
            let key = String::from_utf8_lossy(&key);
            if key.starts_with("ref:") && key.ends_with(to_id) {
                let record: RefRecord =
                    postcard::from_bytes(&value).map_err(|e| CoreError::message(e.to_string()))?;
                out.push(record);
            }
        }
        Ok(out)
    }

    pub fn put_edge(&mut self, edge: &EdgeRecord) -> CoreResult<()> {
        let key = format!("edge:{}:{}", edge.relation.as_str(), edge.id);
        put_postcard(&self.edges, key.as_bytes(), edge)?;
        append_index(
            &self.edges,
            format!("rev:{}:{}", edge.relation.as_str(), edge.to_id),
            &edge.id,
        )
    }

    pub fn next_edge_id(
        &mut self,
        workspace_rev: u64,
        relation: crate::records::Relation,
    ) -> String {
        self.edge_counter += 1;
        crate::records::make_edge_id(workspace_rev, relation, self.edge_counter)
    }

    pub fn callers_of(&self, to_id: &str) -> CoreResult<Vec<EdgeRecord>> {
        let key = format!("rev:calls:{to_id}");
        let ids: Vec<String> = get_postcard(&self.edges, key.as_bytes())?.unwrap_or_default();
        let mut out = Vec::new();
        for id in ids {
            let edge_key = format!("edge:calls:{id}");
            if let Some(edge) = get_postcard(&self.edges, edge_key.as_bytes())? {
                out.push(edge);
            }
        }
        Ok(out)
    }

    pub fn put_file_state(&self, state: &FileStateRecord) -> CoreResult<()> {
        let key = format!("file:{}", state.path);
        put_postcard(&self.file_state, key.as_bytes(), state)
    }

    pub fn get_file_state(&self, path: &str) -> CoreResult<Option<FileStateRecord>> {
        get_postcard(&self.file_state, format!("file:{path}").as_bytes())
    }

    pub fn remove_file_symbols(&mut self, path: &str) -> CoreResult<Vec<String>> {
        let Some(state) = self.get_file_state(path)? else {
            return Ok(Vec::new());
        };

        for id in &state.symbol_ids {
            self.symbols
                .remove(id.as_bytes())
                .map_err(|e| CoreError::message(e.to_string()))?;
        }

        self.file_state
            .remove(format!("file:{path}").as_bytes())
            .map_err(|e| CoreError::message(e.to_string()))?;

        Ok(state.symbol_ids)
    }

    pub fn all_symbols(&self) -> CoreResult<Vec<SymbolRecord>> {
        let mut out = Vec::new();
        for guard in self.symbols.iter() {
            let (key, value) = guard
                .into_inner()
                .map_err(|e| CoreError::message(e.to_string()))?;
            let key = String::from_utf8_lossy(&key);
            if key.starts_with("name:") || key.starts_with("path:") || key.starts_with("kind:") {
                continue;
            }
            let record: SymbolRecord =
                postcard::from_bytes(&value).map_err(|e| CoreError::message(e.to_string()))?;
            out.push(record);
        }
        Ok(out)
    }

    pub fn all_refs(&self) -> CoreResult<Vec<RefRecord>> {
        let mut out = Vec::new();
        for guard in self.refs.iter() {
            let (_key, value) = guard
                .into_inner()
                .map_err(|e| CoreError::message(e.to_string()))?;
            let record: RefRecord =
                postcard::from_bytes(&value).map_err(|e| CoreError::message(e.to_string()))?;
            out.push(record);
        }
        Ok(out)
    }
}

fn open_keyspace(db: &Database, name: &str) -> CoreResult<Keyspace> {
    db.keyspace(name, KeyspaceCreateOptions::default)
        .map_err(|e| CoreError::message(e.to_string()))
}

fn count_edges(edges: &Keyspace) -> u64 {
    edges
        .iter()
        .filter_map(|guard| guard.into_inner().ok())
        .filter(|(key, _)| {
            let key = String::from_utf8_lossy(key);
            key.starts_with("edge:calls:")
        })
        .count() as u64
}

fn put_postcard<T: Serialize>(keyspace: &Keyspace, key: &[u8], value: &T) -> CoreResult<()> {
    let bytes = postcard::to_allocvec(value).map_err(|e| CoreError::message(e.to_string()))?;
    keyspace
        .insert(key, bytes)
        .map_err(|e| CoreError::message(e.to_string()))
}

fn get_postcard<T: DeserializeOwned>(keyspace: &Keyspace, key: &[u8]) -> CoreResult<Option<T>> {
    let Some(bytes) = keyspace
        .get(key)
        .map_err(|e| CoreError::message(e.to_string()))?
    else {
        return Ok(None);
    };
    let value = postcard::from_bytes(&bytes).map_err(|e| CoreError::message(e.to_string()))?;
    Ok(Some(value))
}

fn append_index(keyspace: &Keyspace, key: String, id: &str) -> CoreResult<()> {
    let mut ids: Vec<String> = get_postcard(keyspace, key.as_bytes())?.unwrap_or_default();
    if !ids.iter().any(|existing| existing == id) {
        ids.push(id.to_string());
    }
    put_postcard(keyspace, key.as_bytes(), &ids)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::RECORD_VERSION;
    use codesift_core::{Language, Location, Span, SymbolKind};
    use tempfile::tempdir;

    fn sample_symbol(id: &str, name: &str) -> SymbolRecord {
        SymbolRecord {
            id: id.to_string(),
            kind: SymbolKind::Function,
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
    fn round_trip_symbol_and_secondary_lookup() {
        let dir = tempdir().unwrap();
        let index_dir = camino::Utf8PathBuf::from_path_buf(dir.path().join(".codesift")).unwrap();
        let mut store = IndexStore::open(&index_dir).unwrap();

        let sym = sample_symbol("sym://1/src/lib.rs#function:hello@0:10", "hello");
        store.put_symbol(&sym).unwrap();
        store.commit().unwrap();

        let loaded = store
            .get_symbol("sym://1/src/lib.rs#function:hello@0:10")
            .unwrap()
            .unwrap();
        assert_eq!(loaded.name, "hello");

        let by_name = store.lookup_by_name("hello").unwrap();
        assert_eq!(by_name.len(), 1);

        store.save_meta().unwrap();
        assert!(index_dir.join("meta.json").exists());
    }
}
