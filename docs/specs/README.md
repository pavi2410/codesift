# Specifications

Implementation contracts for codesift. **Specs are the primary reference for coding.**

Each spec has a status: `draft` | `accepted` | `deprecated`.

| Spec | Status | Description |
|------|--------|-------------|
| [symbol-model.md](symbol-model.md) | draft | Symbols, IDs, relations |
| [index-schema.md](index-schema.md) | draft | Storage keys and record formats |
| [query-language.md](query-language.md) | draft | Structural and semantic query DSL |
| [incremental-indexing.md](incremental-indexing.md) | draft | Incremental update algorithm |
| [chunking-and-semantic.md](chunking-and-semantic.md) | draft | Chunks, embeddings, reranking |
| [storage.md](storage.md) | draft | `.codesift/` layout and backends |
| [vfs-and-workspace.md](vfs-and-workspace.md) | draft | File discovery and workspace |
| [cli.md](cli.md) | draft | CLI commands and output |
| [mcp.md](mcp.md) | draft | MCP tools and schemas |

**Reading order for implementers:** symbol-model → index-schema → storage → query-language → interfaces
