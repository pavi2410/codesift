//! Query parsing and execution.

mod executor;
mod parser;
mod render;
mod response;
mod service;
mod validate;

pub use executor::{QueryExecutor, invalid_query};
pub use parser::{StructuralQuery, parse_query};
pub use render::{RenderOptions, render_query_hits, render_refs_tree, render_status};
pub use response::{QueryError, QueryHit, QueryResponse, StatusResponse};
pub use service::CodeIntel;
pub use validate::parse_query_with_hints;
