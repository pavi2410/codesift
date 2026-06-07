# Goals and Non-Goals

## Goals

### Structural indexing

- Index definitions: modules, functions, types, fields, macros, imports
- Index references and call sites with stable symbol IDs
- Support queries: find symbol, find references, find callers, import graph (`structural-depth`)
- Persist indexes to disk for fast cold start

### Semantic indexing

- Chunk code and docs at meaningful boundaries (functions, sections, doc comments)
- Embed chunks locally (fastembed) with content-hash caching
- Support natural-language search with optional structural filters
- Hybrid retrieval: vector similarity + BM25 + reranking

### Interfaces

- **CLI** for scripting, CI, and local development (`clap`)
- **MCP** for AI coding agents with structured tool responses (`rmcp`, stdio)
- **Library API** (future) for IDE plugins and embedded use

### Operations

- Incremental indexing on file change (`notify` + invalidation graph)
- gitignore-aware workspace traversal (`ignore`)
- Export index data (JSONL) for downstream SAST/lint tools

### Multi-consumer design

One index serves:

- AI agents (MCP)
- Developers (CLI)
- CI / code review (blast radius queries)
- Future IDE/LSP integrations
- Future pluggable analyzers over PSI

## Non-Goals

| Non-goal | Rationale |
|----------|-----------|
| Full type inference | High complexity; defer to per-language phases |
| Replacing rust-analyzer / IDE language servers | Complementary; codesift indexes and queries, LSPs type-check |
| Real-time collaborative editing | Out of scope for indexing engine |
| Hosted SaaS / cloud index | Local-first; remote optional later |
| Refactoring engine | Requires deep resolve; not an IDE |
| Building a full IDE | Inspiration source only |
| Remote embedding APIs as default | Privacy, offline, latency; opt-in later |

## Capability coverage

What we build and verify is tracked in [capabilities.md](capabilities.md) — by feature, not timeline. Acceptance criteria live in [milestones/](milestones/).

Long-term analysis goals (inspections, duplicates, data flow, agent quality gates) are in [big-picture.md](big-picture.md); many are `exploring` until promoted to the coverage matrix.
