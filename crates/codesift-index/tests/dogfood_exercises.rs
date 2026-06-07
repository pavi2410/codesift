use codesift_core::Workspace;
use codesift_index::{IndexOptions, Indexer};
use codesift_query::{QueryExecutor, parse_query};
use codesift_store::IndexStore;
use tempfile::tempdir;

fn index_fixture(root: &std::path::Path) -> camino::Utf8PathBuf {
    std::fs::create_dir_all(root.join("crates/query/src")).unwrap();
    std::fs::create_dir_all(root.join("crates/cli/src")).unwrap();
    std::fs::write(
        root.join("Cargo.toml"),
        "[workspace]\nmembers = [\"crates/query\", \"crates/cli\"]\n",
    )
    .unwrap();
    std::fs::write(
        root.join("crates/query/Cargo.toml"),
        "[package]\nname = \"query-crate\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    )
    .unwrap();
    std::fs::write(
        root.join("crates/cli/Cargo.toml"),
        "[package]\nname = \"cli-crate\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    )
    .unwrap();
    std::fs::write(
        root.join("crates/query/src/parser.rs"),
        r#"
pub fn parse_query(input: &str) -> Result<(), ()> {
    let _ = input.trim();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse() {
        let _ = parse_query("x");
    }
}
"#,
    )
    .unwrap();
    std::fs::write(
        root.join("crates/query/src/lib.rs"),
        "mod parser;\npub use parser::parse_query;\n",
    )
    .unwrap();
    std::fs::write(
        root.join("crates/cli/src/main.rs"),
        r#"
use query_crate::parse_query;

fn run() {
    let _ = parse_query("symbol:name=main");
}

fn main() {
    run();
    let _ = parse_query("again");
}
"#,
    )
    .unwrap();

    let ws = Workspace::discover(root).unwrap();
    Indexer::index(&ws, &IndexOptions::default()).unwrap();
    ws.index_dir
}

/// DF-001: refs by name finds all call sites including cross-crate usages.
#[test]
fn df001_parse_query_refs_and_callers() {
    let dir = tempdir().unwrap();
    let index_dir = index_fixture(dir.path());
    let store = IndexStore::open(&index_dir).unwrap();
    let executor = QueryExecutor::new(&store);

    let hits = executor.refs_by_name("parse_query").unwrap();
    assert!(
        hits.len() >= 3,
        "expected at least main, run, and test refs; got {}",
        hits.len()
    );

    let paths: Vec<_> = hits
        .iter()
        .map(|h| h.site.as_ref().map(|s| s.path.as_str()).unwrap_or(""))
        .collect();
    assert!(paths.iter().any(|p| p.contains("main.rs")));
    assert!(paths.iter().any(|p| p.contains("parser.rs")));

    let symbols = store.lookup_by_name("parse_query").unwrap();
    let parse_id = symbols
        .iter()
        .find(|s| s.path.contains("parser.rs"))
        .map(|s| s.id.clone())
        .expect("parse_query definition");

    let query = parse_query(&format!("callers:of={parse_id} depth=2")).unwrap();
    let callers = executor.execute(&query).unwrap();
    let caller_names: Vec<_> = callers.hits.iter().map(|h| h.symbol.name.as_str()).collect();
    assert!(caller_names.contains(&"run"));
    assert!(caller_names.contains(&"main"));
}

/// DF-003: path glob + kind filter lists query-crate functions.
#[test]
fn df003_path_glob_and_kind_filter() {
    let dir = tempdir().unwrap();
    let index_dir = index_fixture(dir.path());
    let store = IndexStore::open(&index_dir).unwrap();

    let query = parse_query("path=**/parser.rs kind=function").unwrap();
    let response = QueryExecutor::new(&store).execute(&query).unwrap();
    assert!(!response.hits.is_empty());
    assert!(
        response
            .hits
            .iter()
            .any(|h| h.symbol.name == "parse_query")
    );
}

/// DF-002: callers of a method in the indexed fixture (Indexer::index proxy via helper).
#[test]
fn df002_helper_callers() {
    let dir = tempdir().unwrap();
    std::fs::create_dir_all(dir.path().join("src")).unwrap();
    std::fs::write(
        dir.path().join("Cargo.toml"),
        "[package]\nname = \"fixture\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    )
    .unwrap();
    std::fs::write(
        dir.path().join("src/lib.rs"),
        r#"
pub fn index() {}
fn drive() { index(); }
"#,
    )
    .unwrap();

    let ws = Workspace::discover(dir.path()).unwrap();
    Indexer::index(&ws, &IndexOptions::default()).unwrap();
    let store = IndexStore::open(&ws.index_dir).unwrap();

    let index_sym = store
        .lookup_by_name("index")
        .unwrap()
        .into_iter()
        .find(|s| s.kind.as_str() == "function")
        .expect("index function");

    let query = parse_query(&format!("callers:of={} depth=1", index_sym.id)).unwrap();
    let response = QueryExecutor::new(&store).execute(&query).unwrap();
    assert!(
        response
            .hits
            .iter()
            .any(|h| h.symbol.name == "drive"),
        "expected drive to call index"
    );
}
