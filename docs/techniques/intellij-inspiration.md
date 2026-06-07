# IntelliJ Inspiration

codesift adopts architectural ideas from the IntelliJ Platform without cloning a full IDE.

## Concept mapping

| IntelliJ | codesift | Notes |
|----------|----------|-------|
| PSI (`PsiElement`) | PSI layer over CST | Unified extraction API per language |
| Stub indexes | fjall `symbols`, `refs` keyspaces | Persistent, file-independent keys |
| FileBasedIndex | `file_state` keyspace | Per-file data keyed by path + hash |
| ReferencesSearch | `refs:` / `callers:` queries | Reverse edge index |
| ShortNamesCache | `name:` secondary index in fjall | Fast name lookup |
| Find Usages | `refs:to=` query | Same data as ReferencesSearch |
| Goto Class/Symbol | `symbol:name=` query | Name + kind filters |
| Hierarchy view | `callers:` / `callees:` with depth | Call graph traversal |
| Indexing on startup | `codesift index` / watch | Background incremental |

## What we adopt

### Stub-based persistence

IntelliJ stubs let indexes survive file close and IDE restart. codesift persists the same class of data to disk (`.codesift/`) so CLI and MCP sessions do not require re-parsing.

### Separation of syntax and resolve

IntelliJ PSI separates syntax structure from resolve (binding names to targets). codesift indexes **syntax-level** refs in MVP; full resolve is deferred per language.

### Incremental indexing

IntelliJ re-indexes only affected files on change. codesift uses the same principle with file-level invalidation and incremental parsers.

## What we defer

| IntelliJ feature | Why deferred |
|------------------|--------------|
| Type inference | Requires compiler integration per language |
| Resolve to typed symbols | Cross-crate, macro expansion complexity |
| Refactoring engine | Needs resolve + PSI rewrite |
| Inspections / intentions | Unified via adapters (`inspections-unified`); not a full IDE intention UI |
| Plugin ecosystem | Future; MCP/CLI first |

## Design lesson: index everything queryable

IntelliJ's power comes from **many specialized indexes** (stub index per key type). codesift starts with a smaller set (symbols, refs, edges, text, vectors) but uses the same principle: **precompute what you query often**.

## See also

- [structural-indexing.md](structural-indexing.md)
- [../references/intellij-and-ide-indexing.md](../references/intellij-and-ide-indexing.md)
- [../glossary.md](../glossary.md)
