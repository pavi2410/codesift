use codesift_core::{Language, Location, SymbolKind, Visibility};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SymbolRecord {
    pub id: String,
    pub kind: SymbolKind,
    pub name: String,
    pub qualified_name: Option<String>,
    pub path: String,
    pub language: Language,
    pub location: Location,
    pub visibility: Option<Visibility>,
    pub signature: Option<String>,
    pub doc_comment: Option<String>,
    pub parent_id: Option<String>,
    pub workspace_rev: u64,
    pub record_version: u16,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RefKind {
    Reference,
    Call,
    Import,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SiteLocation {
    pub path: String,
    pub start_byte: u32,
    pub end_byte: u32,
    pub start_line: u32,
    pub start_column: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RefRecord {
    pub from_id: String,
    pub to_id: String,
    pub ref_kind: RefKind,
    pub site: SiteLocation,
    pub workspace_rev: u64,
    pub record_version: u16,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Relation {
    Calls,
    Imports,
    Implements,
    References,
}

impl Relation {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Calls => "calls",
            Self::Imports => "imports",
            Self::Implements => "implements",
            Self::References => "references",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EdgeRecord {
    pub id: String,
    pub relation: Relation,
    pub from_id: String,
    pub to_id: String,
    pub call_site: Option<SiteLocation>,
    pub resolved: bool,
    pub workspace_rev: u64,
    pub record_version: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FileStateRecord {
    pub path: String,
    pub content_hash: String,
    pub language: Language,
    pub symbol_ids: Vec<String>,
    pub indexed_at: chrono::DateTime<chrono::Utc>,
    pub parse_errors: Vec<String>,
}

pub fn make_symbol_id(
    workspace_rev: u64,
    path: &str,
    kind: SymbolKind,
    name: &str,
    start_byte: u32,
    end_byte: u32,
) -> String {
    format!(
        "sym://{workspace_rev}/{path}#{kind}:{name}@{start_byte}:{end_byte}",
        kind = kind.as_str(),
    )
}

pub fn make_edge_id(workspace_rev: u64, relation: Relation, counter: u64) -> String {
    format!("edge://{workspace_rev}/{}/{:x}", relation.as_str(), counter)
}
