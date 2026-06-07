use codesift_core::Workspace;
use codesift_index::{IndexOptions, Indexer};

pub fn index_fixture(root: &std::path::Path) -> camino::Utf8PathBuf {
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
