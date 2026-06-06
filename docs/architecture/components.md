# Components

Responsibility matrix for codesift engine components. Maps to future crates in [future-crate-layout.md](future-crate-layout.md).

## Component matrix

| Component | Responsibility | Inputs | Outputs | Crate |
|-----------|----------------|--------|---------|-------|
| **VFS** | File discovery, ignore rules, content hashing, change events | Paths, notify events | `FileId`, content hash, text | `codesift-core` |
| **Workspace** | Root config, `workspace_rev`, meta | VFS | Workspace context | `codesift-core` |
| **Parser** | Syntax trees per language | Source text | PSI / CST | `codesift-parse` |
| **PSI bridge** | Unified symbol extraction API | CST | `Symbol`, `Relation` drafts | `codesift-parse` |
| **Structural indexer** | Symbols, refs, calls, imports | PSI | Index records | `codesift-index` |
| **Semantic indexer** | Chunking, embedding orchestration | PSI, text | Chunks, vectors | `codesift-index` |
| **Embedder** | Vector generation, cache lookup | Text, model config | `f32[]` embedding | `codesift-index` |
| **Store (KV)** | Persistent symbol graph | Records | Read/write API | `codesift-store` |
| **Store (vector)** | ANN index | Embeddings | Nearest neighbors | `codesift-store` |
| **Store (text)** | BM25 index | Text fields | Scored hits | `codesift-store` |
| **Query engine** | Parse DSL, execute plans, rank | Query + store | `QueryResult` | `codesift-query` |
| **CLI** | User commands, formatting | argv | stdout/stderr | `codesift-cli` |
| **MCP server** | Tool handlers, session | JSON-RPC stdio | Tool results | `codesift-mcp` |

## Dependency direction

```
codesift-cli  ──┐
codesift-mcp  ──┼──► codesift-query ──► codesift-store
                │           │
                │           ▼
                └────► codesift-index ──► codesift-parse
                              │
                              ▼
                        codesift-core
```

- **codesift-core** — no internal codesift dependencies
- **codesift-parse** — depends on core
- **codesift-index** — depends on parse, core
- **codesift-store** — depends on core (index records types)
- **codesift-query** — depends on store, index types
- **codesift-cli / mcp** — depend on query, index (for `index` command)

## Cross-cutting concerns

| Concern | Owner | Notes |
|---------|-------|-------|
| Error types | `codesift-core` | Unified `Error` enum |
| Logging | CLI/MCP | `tracing` subscriber |
| Config | `codesift-core` | TOML in `.codesift/config.toml` (future) |
| Metrics | `codesift-core` | Optional; index stats in `status` |

## See also

- [future-crate-layout.md](future-crate-layout.md)
- [technology-stack.md](technology-stack.md)
