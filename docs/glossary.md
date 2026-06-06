# Glossary

Terms used throughout codesift documentation.

## Core concepts

### PSI (Program Structure Interface)

A language-aware syntax tree with semantic annotations. Inspired by IntelliJ's PSI. codesift builds a PSI-like layer on top of parsers (tree-sitter, `ra_ap_syntax`) to extract symbols and relations.

### Symbol

A named, addressable program element: function, type, module, field, macro, import, or doc section. Each symbol has a stable **symbol ID** and a **location** in source.

### Symbol ID

A stable URI identifying a symbol within a workspace revision:

```
sym://{workspace_rev}/{path}#{kind}:{name}@{start_byte}:{end_byte}
```

See [specs/symbol-model.md](specs/symbol-model.md).

### Chunk

A semantic unit of text indexed for embedding search: function body, struct block, markdown section, or doc comment. Chunks link to optional `symbol_id` metadata.

### Structural search

Queries over the symbol graph: definitions, references, callers, imports. Uses the query DSL (`symbol:`, `refs:`, `callers:`).

### Semantic search

Natural-language retrieval over embedded chunks. Uses vector similarity (usearch) with optional structural filters.

### Hybrid search

Combines semantic (vector), lexical (tantivy BM25), and reranking (fastembed) with structural boosts.

## Indexing

### Workspace

A root directory (or multi-root set) of files to index. Identified by path and configuration.

### Workspace revision (`workspace_rev`)

Monotonic identifier bumped when the index changes materially. Symbol IDs include this revision for stability across renames within a revision scope.

### VFS (Virtual File System)

Abstraction over files on disk: paths, content hashes, ignore rules, and change notifications.

### Tombstone

A marker indicating a previously indexed symbol or record is deleted. Used in incremental updates before compaction.

### Incremental indexing

Re-indexing only changed files and patching affected cross-file references, rather than full rebuild.

### Invalidation graph

Tracks which files depend on symbols defined in other files. Drives incremental re-index scope.

## Storage

### Keyspace

A logical table in fjall (similar to RocksDB column families): `symbols`, `refs`, `edges`, `chunks`, etc.

### `.codesift/`

Default on-disk index directory at workspace root. See [specs/storage.md](specs/storage.md).

## IntelliJ mapping

| IntelliJ term | codesift term |
|---------------|---------------|
| Stub index | fjall symbol/ref keyspaces |
| FileBasedIndex | Per-file records in `file_state` keyspace |
| ReferencesSearch | `refs:` / `callers:` queries |
| PsiElement | PSI node / symbol extractor input |

## Analysis (planned)

### Cyclomatic complexity

McCabe metric: number of linearly independent paths through a function's control-flow graph. Indexed per symbol for threshold inspections. See [future/complexity-metrics.md](future/complexity-metrics.md).

### Cognitive complexity

Sonar-style metric penalizing nested control flow — correlates with "hard to understand safely." Computed alongside cyclomatic complexity. See [future/complexity-metrics.md](future/complexity-metrics.md).

### PURL (Package URL)

Canonical identifier for a resolved library version (`pkg:cargo/serde@1.0.203`). Used for cross-project doc cache keys. See [future/purl-library-docs.md](future/purl-library-docs.md).

### Library store

Global on-disk cache (`~/.codesift/global/`) of chunked, embedded dependency documentation shared across workspace indexes.

## Interfaces

### MCP (Model Context Protocol)

Standard protocol for AI tools. codesift exposes search and symbol tools via `rmcp` over stdio.

### CLI

Command-line interface (`codesift` binary) for index, query, search, and export.

## See also

- [techniques/intellij-inspiration.md](techniques/intellij-inspiration.md)
- [specs/symbol-model.md](specs/symbol-model.md)
