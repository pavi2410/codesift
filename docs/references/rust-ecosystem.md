# Rust Ecosystem

Curated crate references for codesift implementation (June 2026 versions).

## Core stack

| Crate | Version | Role | Docs |
|-------|---------|------|------|
| fjall | 3.1.x | Embedded LSM KV | https://docs.rs/fjall |
| usearch | 2.25.x | Vector ANN | https://docs.rs/usearch |
| tantivy | 0.26.x | BM25 text search | https://docs.rs/tantivy |
| tree-sitter | 0.26.x | Multi-lang parser | https://docs.rs/tree-sitter |
| fastembed | 5.15.x | Local embeddings | https://docs.rs/fastembed |
| rmcp | 1.7.x | MCP SDK | https://docs.rs/rmcp |
| clap | 4.x | CLI | https://docs.rs/clap |
| postcard | 1.x | Serialization | https://docs.rs/postcard |

## Parsing

| Crate | Role |
|-------|------|
| ra_ap_syntax | Rust CST (rust-analyzer) — https://rust-lang.github.io/rust-analyzer/syntax/ |
| tree-sitter-rust | Fallback / comparison — https://crates.io/crates/tree-sitter-rust |

## Infrastructure

| Crate | Version | Role |
|-------|---------|------|
| notify | 8.x | File watcher |
| ignore | 0.4.x | gitignore-aware walk |
| blake3 | 1.x | Content hashing |
| tokio | 1.x | Async runtime (MCP) |
| serde / serde_json | 1.x | JSON export |
| tracing | 0.1.x | Logging (TBD) |
| thiserror | 2.x | Errors (TBD) |
| camino | 1.x | UTF-8 paths (TBD) |

## Benchmark references

| Comparison | Source |
|------------|--------|
| tantivy vs Lucene | https://tantivy-search.github.io/bench/ (~2x faster latency) |
| fjall vs redb vs RocksDB | fjall README benchmarks; redb excels reads, fjall/rocksdb excel writes |
| usearch vs FAISS | USearch project claims ~10x vs FAISS HNSW on selected benchmarks |

## Cargo workspace patterns

- Workspace resolver `"2"`
- Shared `[workspace.dependencies]` for version pinning
- See [future-crate-layout.md](../architecture/future-crate-layout.md)

## MCP ecosystem

| Resource | URL |
|----------|-----|
| MCP specification | https://modelcontextprotocol.io |
| rust-sdk (rmcp) | https://github.com/modelcontextprotocol/rust-sdk |

## See also

- [technology-stack.md](../architecture/technology-stack.md)
- [adr/README.md](../adr/README.md)
