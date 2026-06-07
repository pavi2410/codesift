mod fixture;

use codesift_core::Workspace;
use codesift_mcp::server::{CodesiftServer, FindReferencesParams, GetCallersParams, GetSymbolParams};
use codesift_query::CodeIntel;
use fixture::index_fixture;
use rmcp::handler::server::wrapper::Parameters;
use tempfile::tempdir;

#[test]
fn mcp_find_references_by_name() {
    let dir = tempdir().unwrap();
    index_fixture(dir.path());
    let ws = Workspace::discover(dir.path()).unwrap();
    let intel = CodeIntel::open(&ws).unwrap();
    let server = CodesiftServer::new(intel);

    let result = server
        .find_references(Parameters(FindReferencesParams {
            name: Some("parse_query".to_string()),
            symbol_id: None,
            kind: None,
        }))
        .unwrap();
    let text = &result.content[0].as_text().unwrap().text;
    assert!(text.contains("main.rs"));
    assert!(text.contains("parser.rs"));
}

#[test]
fn mcp_get_callers_depth_two() {
    let dir = tempdir().unwrap();
    index_fixture(dir.path());
    let ws = Workspace::discover(dir.path()).unwrap();
    let intel = CodeIntel::open(&ws).unwrap();
    let server = CodesiftServer::new(intel);

    let result = server
        .get_callers(Parameters(GetCallersParams {
            name: Some("parse_query".to_string()),
            symbol_id: None,
            kind: None,
            depth: Some(2),
        }))
        .unwrap();
    let text = &result.content[0].as_text().unwrap().text;
    assert!(text.contains("main"));
    assert!(text.contains("run"));
}

#[test]
fn mcp_get_symbol_by_name() {
    let dir = tempdir().unwrap();
    index_fixture(dir.path());
    let ws = Workspace::discover(dir.path()).unwrap();
    let intel = CodeIntel::open(&ws).unwrap();
    let server = CodesiftServer::new(intel);

    let result = server
        .get_symbol(Parameters(GetSymbolParams {
            name: Some("parse_query".to_string()),
            symbol_id: None,
            kind: Some("function".to_string()),
            path: None,
        }))
        .unwrap();
    let text = &result.content[0].as_text().unwrap().text;
    assert!(text.contains("parser.rs"));
}
