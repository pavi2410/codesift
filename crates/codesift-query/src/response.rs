use codesift_store::{RefKind, SiteLocation, SymbolRecord};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct QueryResponse {
    pub query: String,
    pub workspace_rev: u64,
    pub took_ms: u64,
    pub hits: Vec<QueryHit>,
    pub total: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<QueryError>,
}

#[derive(Debug, Clone, Serialize)]
pub struct QueryHit {
    pub symbol: SymbolRecord,
    pub score: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site: Option<SiteLocation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_kind: Option<RefKind>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth: Option<u32>,
}

#[derive(Debug, Clone, Serialize)]
pub struct QueryError {
    pub code: String,
    pub message: String,
    pub query: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct StatusResponse {
    pub workspace_rev: u64,
    pub index_format_version: u32,
    pub files: u64,
    pub symbols: u64,
    pub semantic_ready: bool,
    pub last_indexed: String,
    pub index_path: String,
}
