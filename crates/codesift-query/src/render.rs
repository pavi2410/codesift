//! ASCII tree rendering for query results.

use crate::response::{QueryHit, QueryResponse, StatusResponse};

pub struct RenderOptions {
    pub color: bool,
}

impl Default for RenderOptions {
    fn default() -> Self {
        Self {
            color: std::io::IsTerminal::is_terminal(&std::io::stdout()),
        }
    }
}

pub fn render_status(status: &StatusResponse) -> String {
    format!(
        "index  rev={}  files={}  symbols={}  path={}",
        status.workspace_rev, status.files, status.symbols, status.index_path
    )
}

pub fn render_query_hits(response: &QueryResponse, opts: &RenderOptions) -> String {
    if response.hits.is_empty() {
        return "no results".to_string();
    }

    let mut out = String::new();
    let header = if response.query.starts_with("callers:of=") {
        format!("callers ({})", response.total)
    } else if response.query.starts_with("refs:to=") || response.query.contains("refs --name") {
        format!("refs ({})", response.total)
    } else {
        format!("hits ({})", response.total)
    };
    out.push_str(&header);
    out.push('\n');

    for hit in &response.hits {
        out.push_str(&render_hit_line(hit, opts, "  "));
    }
    out
}

fn render_hit_line(hit: &QueryHit, _opts: &RenderOptions, prefix: &str) -> String {
    let kind = hit.symbol.kind.as_str();
    let name = &hit.symbol.name;
    if let Some(site) = &hit.site {
        let depth = hit
            .depth
            .map(|d| format!("  depth={d}"))
            .unwrap_or_default();
        format!(
            "{prefix}{kind} {name}  {}:{}:{}{}\n",
            site.path, site.start_line, site.start_column, depth
        )
    } else {
        let loc = &hit.symbol.location;
        format!(
            "{prefix}{kind} {name}  {}:{}-{}:{}:{}\n",
            hit.symbol.path,
            loc.start_line,
            loc.end_line,
            loc.start_column,
            loc.end_column
        )
    }
}

pub fn render_refs_tree(symbol_name: &str, hits: &[QueryHit], opts: &RenderOptions) -> String {
    let mut out = format!("{symbol_name}\n");
    out.push_str(&format!("├─ refs ({})\n", hits.len()));
    for (i, hit) in hits.iter().enumerate() {
        let branch = if i + 1 == hits.len() { "└─" } else { "├─" };
        if let Some(site) = &hit.site {
            out.push_str(&format!(
                "│  {branch} {}:{}  {}()\n",
                site.path, site.start_line, hit.symbol.name
            ));
        }
    }
    let _ = opts.color;
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use codesift_core::{Language, Location, Span, SymbolKind};
    use codesift_store::{RECORD_VERSION, RefKind, SiteLocation, SymbolRecord};

    fn sample_hit(name: &str, line: u32) -> QueryHit {
        QueryHit {
            symbol: SymbolRecord {
                id: format!("sym://1/a.rs#function:{name}@0:1"),
                kind: SymbolKind::Function,
                name: name.to_string(),
                qualified_name: None,
                path: "a.rs".to_string(),
                language: Language::Rust,
                location: Location::new(
                    "a.rs",
                    Language::Rust,
                    Span {
                        start_byte: 0,
                        end_byte: 1,
                        start_line: line,
                        start_column: 0,
                        end_line: line,
                        end_column: 1,
                    },
                ),
                visibility: None,
                signature: None,
                doc_comment: None,
                parent_id: None,
                workspace_rev: 1,
                record_version: RECORD_VERSION,
            },
            score: 1.0,
            site: Some(SiteLocation {
                path: "a.rs".to_string(),
                start_byte: 0,
                end_byte: 1,
                start_line: line,
                start_column: 4,
            }),
            ref_kind: Some(RefKind::Call),
            depth: None,
        }
    }

    #[test]
    fn renders_refs_tree() {
        let hits = vec![sample_hit("run", 10), sample_hit("test_fn", 20)];
        let text = render_refs_tree("parse_query", &hits, &RenderOptions { color: false });
        assert!(text.contains("refs (2)"));
        assert!(text.contains("a.rs:10"));
    }
}
