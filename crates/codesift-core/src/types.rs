use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    Rust,
    Markdown,
}

impl Language {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Rust => "rust",
            Self::Markdown => "markdown",
        }
    }

    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext {
            "rs" => Some(Self::Rust),
            "md" | "mdx" => Some(Self::Markdown),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SymbolKind {
    Module,
    Function,
    Type,
    Field,
    Const,
    Macro,
    Import,
    Trait,
    Impl,
}

impl SymbolKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Module => "module",
            Self::Function => "function",
            Self::Type => "type",
            Self::Field => "field",
            Self::Const => "const",
            Self::Macro => "macro",
            Self::Import => "import",
            Self::Trait => "trait",
            Self::Impl => "impl",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "module" => Some(Self::Module),
            "function" => Some(Self::Function),
            "type" => Some(Self::Type),
            "field" => Some(Self::Field),
            "const" => Some(Self::Const),
            "macro" => Some(Self::Macro),
            "import" => Some(Self::Import),
            "trait" => Some(Self::Trait),
            "impl" => Some(Self::Impl),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Visibility {
    Pub,
    PubCrate,
    Private,
}

impl Visibility {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Pub => "pub",
            Self::PubCrate => "pub(crate)",
            Self::Private => "private",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Span {
    pub start_byte: u32,
    pub end_byte: u32,
    pub start_line: u32,
    pub start_column: u32,
    pub end_line: u32,
    pub end_column: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Location {
    pub path: String,
    pub language: Language,
    pub start_byte: u32,
    pub end_byte: u32,
    pub start_line: u32,
    pub start_column: u32,
    pub end_line: u32,
    pub end_column: u32,
}

impl Location {
    pub fn new(path: impl Into<String>, language: Language, span: Span) -> Self {
        Self {
            path: path.into(),
            language,
            start_byte: span.start_byte,
            end_byte: span.end_byte,
            start_line: span.start_line,
            start_column: span.start_column,
            end_line: span.end_line,
            end_column: span.end_column,
        }
    }
}
