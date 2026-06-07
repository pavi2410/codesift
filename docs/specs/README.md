# Specifications

Implementation contracts for codesift. **Specs are the primary reference for coding.**

Each spec has a status: `draft` | `accepted` | `deprecated`.

| Spec | Status | Description |
|------|--------|-------------|
| [symbol-model.md](symbol-model.md) | accepted | Symbols, IDs, relations |
| [index-schema.md](index-schema.md) | accepted | Storage keys and record formats |
| [query-language.md](query-language.md) | accepted | Structural and semantic query DSL |
| [incremental-indexing.md](incremental-indexing.md) | draft | Incremental update algorithm |
| [chunking-and-semantic.md](chunking-and-semantic.md) | draft | Chunks, embeddings, reranking |
| [storage.md](storage.md) | accepted | `.codesift/` layout and backends |
| [vfs-and-workspace.md](vfs-and-workspace.md) | accepted | File discovery and workspace |
| [cli.md](cli.md) | accepted | CLI commands and output |
| [mcp.md](mcp.md) | draft | MCP tools and schemas |

**Reading order for implementers:** [capabilities.md](../capabilities.md) → symbol-model → index-schema → storage → query-language → interfaces
