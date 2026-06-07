//! In-memory query store loaded from a snapshot file (no fjall lock).

use std::collections::HashMap;

use camino::Utf8Path;
use codesift_core::{Error as CoreError, Result as CoreResult};
use serde::{Deserialize, Serialize};

use crate::meta::IndexMeta;
use crate::records::{EdgeRecord, RefRecord, SymbolRecord};
use crate::store::make_unresolved_id;

const SNAPSHOT_VERSION: u32 = 1;

#[derive(Debug, Serialize, Deserialize)]
struct QuerySnapshot {
    version: u32,
    meta: IndexMeta,
    symbols: HashMap<String, Vec<u8>>,
    refs: HashMap<String, Vec<u8>>,
    edges: HashMap<String, Vec<u8>>,
}

#[derive(Debug)]
pub struct SnapshotStore {
    pub meta: IndexMeta,
    symbols: HashMap<String, Vec<u8>>,
    refs: HashMap<String, Vec<u8>>,
    edges: HashMap<String, Vec<u8>>,
}

impl SnapshotStore {
    pub fn load(path: &Utf8Path) -> CoreResult<Self> {
        let bytes = std::fs::read(path).map_err(|source| CoreError::Io {
            path: path.to_string(),
            source,
        })?;
        let snap: QuerySnapshot =
            postcard::from_bytes(&bytes).map_err(|e| CoreError::message(e.to_string()))?;
        if snap.version != SNAPSHOT_VERSION {
            return Err(CoreError::message(format!(
                "unsupported query snapshot version {}",
                snap.version
            )));
        }
        Ok(Self {
            meta: snap.meta,
            symbols: snap.symbols,
            refs: snap.refs,
            edges: snap.edges,
        })
    }

    pub fn snapshot_path(index_dir: &Utf8Path) -> camino::Utf8PathBuf {
        index_dir.join("query.snap")
    }

    fn get_bytes(map: &HashMap<String, Vec<u8>>, key: &str) -> CoreResult<Option<Vec<u8>>> {
        Ok(map.get(key).cloned())
    }

    fn get_postcard<T: serde::de::DeserializeOwned>(
        map: &HashMap<String, Vec<u8>>,
        key: &str,
    ) -> CoreResult<Option<T>> {
        let Some(bytes) = Self::get_bytes(map, key)? else {
            return Ok(None);
        };
        let value = postcard::from_bytes(&bytes).map_err(|e| CoreError::message(e.to_string()))?;
        Ok(Some(value))
    }

    pub fn get_symbol(&self, id: &str) -> CoreResult<Option<SymbolRecord>> {
        Self::get_postcard(&self.symbols, id)
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
        let ids: Vec<String> = Self::get_postcard(&self.symbols, key)?.unwrap_or_default();
        let mut out = Vec::new();
        for id in ids {
            if let Some(sym) = self.get_symbol(&id)? {
                out.push(sym);
            }
        }
        Ok(out)
    }

    pub fn refs_to(&self, to_id: &str) -> CoreResult<Vec<RefRecord>> {
        let mut out = Vec::new();
        let mut seen = std::collections::HashSet::new();

        if let Some(kind_name) = kind_name_key(to_id) {
            let index_key = format!("rev:refs:byname:{kind_name}");
            let ref_keys: Vec<String> =
                Self::get_postcard(&self.refs, &index_key)?.unwrap_or_default();
            for ref_key in ref_keys {
                if !seen.insert(ref_key.clone()) {
                    continue;
                }
                if !ref_key.starts_with("ref:") {
                    continue;
                }
                if let Some(record) = Self::get_ref_record(&self.refs, &ref_key)? {
                    out.push(record);
                }
            }
        }

        for target in self.ref_target_ids(to_id)? {
            let index_key = format!("rev:refs:{target}");
            let ref_keys: Vec<String> =
                Self::get_postcard(&self.refs, &index_key)?.unwrap_or_default();
            for ref_key in ref_keys {
                if !seen.insert(ref_key.clone()) {
                    continue;
                }
                if !ref_key.starts_with("ref:") {
                    continue;
                }
                if let Some(record) = Self::get_ref_record(&self.refs, &ref_key)? {
                    out.push(record);
                }
            }
        }
        Ok(out)
    }

    pub fn ref_target_ids(&self, to_id: &str) -> CoreResult<Vec<String>> {
        let mut ids = vec![to_id.to_string()];
        if let Some(sym) = self.get_symbol(to_id)? {
            let unresolved = make_unresolved_id(sym.workspace_rev, sym.kind, &sym.name);
            if unresolved != to_id {
                ids.push(unresolved);
            }
            for other in self.lookup_by_name(&sym.name)? {
                if other.kind == sym.kind && other.id != to_id {
                    ids.push(other.id.clone());
                }
            }
        } else if to_id.contains("/unresolved#")
            && let Some((kind, name)) = parse_unresolved_id(to_id)
        {
            for sym in self.lookup_by_name(&name)? {
                if sym.kind.as_str() == kind {
                    ids.push(sym.id.clone());
                }
            }
        }
        ids.sort();
        ids.dedup();
        Ok(ids)
    }

    pub fn refs_to_name(&self, name: &str) -> CoreResult<Vec<RefRecord>> {
        let mut out = Vec::new();
        let mut seen = std::collections::HashSet::new();
        for sym in self.lookup_by_name(name)? {
            for reference in self.refs_to(&sym.id)? {
                let key = format!(
                    "{}:{}:{}",
                    reference.from_id, reference.to_id, reference.site.start_byte
                );
                if seen.insert(key) {
                    out.push(reference);
                }
            }
        }
        let unresolved = make_unresolved_id(
            self.meta.workspace_rev,
            codesift_core::SymbolKind::Function,
            name,
        );
        for reference in self.refs_to(&unresolved)? {
            let key = format!(
                "{}:{}:{}",
                reference.from_id, reference.to_id, reference.site.start_byte
            );
            if seen.insert(key) {
                out.push(reference);
            }
        }
        Ok(out)
    }

    pub fn callers_of(&self, to_id: &str) -> CoreResult<Vec<EdgeRecord>> {
        let mut out = Vec::new();
        let mut seen = std::collections::HashSet::new();

        if let Some(kind_name) = kind_name_key(to_id) {
            let index_key = format!("rev:calls:byname:{kind_name}");
            let ids: Vec<String> = Self::get_postcard(&self.edges, &index_key)?.unwrap_or_default();
            for id in ids {
                if !seen.insert(id.clone()) {
                    continue;
                }
                let edge_key = format!("edge:calls:{id}");
                if let Some(edge) = Self::get_postcard(&self.edges, &edge_key)? {
                    out.push(edge);
                }
            }
        }

        for target in self.ref_target_ids(to_id)? {
            let key = format!("rev:calls:{target}");
            let ids: Vec<String> = Self::get_postcard(&self.edges, &key)?.unwrap_or_default();
            for id in ids {
                if !seen.insert(id.clone()) {
                    continue;
                }
                let edge_key = format!("edge:calls:{id}");
                if let Some(edge) = Self::get_postcard(&self.edges, &edge_key)? {
                    out.push(edge);
                }
            }
        }
        Ok(out)
    }

    pub fn all_symbols(&self) -> CoreResult<Vec<SymbolRecord>> {
        let mut out = Vec::new();
        for (key, value) in &self.symbols {
            if key.starts_with("name:") || key.starts_with("path:") || key.starts_with("kind:") {
                continue;
            }
            let record: SymbolRecord =
                postcard::from_bytes(value).map_err(|e| CoreError::message(e.to_string()))?;
            out.push(record);
        }
        Ok(out)
    }

    fn get_ref_record(map: &HashMap<String, Vec<u8>>, key: &str) -> CoreResult<Option<RefRecord>> {
        if !key.starts_with("ref:") {
            return Ok(None);
        }
        Self::get_postcard(map, key)
    }
}

pub fn write_query_snapshot(
    meta: &IndexMeta,
    symbols: &HashMap<String, Vec<u8>>,
    refs: &HashMap<String, Vec<u8>>,
    edges: &HashMap<String, Vec<u8>>,
    path: &Utf8Path,
) -> CoreResult<()> {
    let snap = QuerySnapshot {
        version: SNAPSHOT_VERSION,
        meta: meta.clone(),
        symbols: symbols.clone(),
        refs: refs.clone(),
        edges: edges.clone(),
    };
    let bytes = postcard::to_allocvec(&snap).map_err(|e| CoreError::message(e.to_string()))?;
    std::fs::write(path, bytes).map_err(|source| CoreError::Io {
        path: path.to_string(),
        source,
    })
}

fn kind_name_key(id: &str) -> Option<String> {
    if id.contains("/unresolved#") {
        return id
            .split("/unresolved#")
            .nth(1)
            .and_then(|rest| rest.split('@').next())
            .map(str::to_string);
    }
    id.split('#')
        .nth(1)
        .and_then(|rest| rest.split('@').next())
        .map(str::to_string)
}

fn parse_unresolved_id(id: &str) -> Option<(String, String)> {
    let rest = id.split("/unresolved#").nth(1)?;
    let kind_name = rest.split('@').next()?;
    let (kind, name) = kind_name.split_once(':')?;
    Some((kind.to_string(), name.to_string()))
}
