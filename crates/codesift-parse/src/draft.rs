use codesift_core::{SymbolKind, Visibility};

#[derive(Debug, Clone)]
pub struct SymbolDraft {
    pub kind: SymbolKind,
    pub name: String,
    pub qualified_name: Option<String>,
    pub start_byte: u32,
    pub end_byte: u32,
    pub start_line: u32,
    pub start_column: u32,
    pub end_line: u32,
    pub end_column: u32,
    pub visibility: Option<Visibility>,
    pub signature: Option<String>,
    pub doc_comment: Option<String>,
    pub parent_name: Option<String>,
}

#[derive(Debug, Clone)]
pub struct RefDraft {
    pub from_name: String,
    pub to_name: String,
    pub start_byte: u32,
    pub end_byte: u32,
    pub start_line: u32,
    pub start_column: u32,
}

#[derive(Debug, Clone)]
pub struct CallDraft {
    pub caller_name: String,
    pub callee_name: String,
    pub start_byte: u32,
    pub end_byte: u32,
    pub start_line: u32,
    pub start_column: u32,
}

#[derive(Debug, Clone, Default)]
pub struct ParseOutput {
    pub symbols: Vec<SymbolDraft>,
    pub refs: Vec<RefDraft>,
    pub calls: Vec<CallDraft>,
    pub errors: Vec<String>,
}
