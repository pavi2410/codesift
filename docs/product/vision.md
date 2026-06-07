# Vision

## Problem

Developers and tools need to find and reason about code across large, evolving codebases. Today this is fragmented:

| Approach | Strength | Gap |
|----------|----------|-----|
| Text search (grep, ripgrep) | Fast literal match | No structure, no semantics |
| Language servers (rust-analyzer, etc.) | Rich semantics per language | Ephemeral, IDE-bound, not unified across artifacts |
| Embedding-only RAG | Natural-language retrieval | No stable symbols, weak structural queries |
| ctags / universal-ctags | Lightweight symbol index | Shallow, no refs/call graph, no semantics |

No single system offers a **persistent, incremental, language-aware index** that combines **structural queries** (definitions, references, callers) with **semantic search** (intent-based retrieval) and serves multiple consumers: CLI, MCP agents, CI, and future IDE plugins.

Agents working at scale also need **deep static intelligence** — inspections, impact analysis, duplicate detection, and quality gates — to avoid unreliable, vibe-driven edits. That long-term direction is described in [big-picture.md](big-picture.md).

## Solution

**codesift** is a Rust-based indexing engine that:

1. **Parses** source and docs into a unified PSI-like model
2. **Indexes** symbols, references, call edges, and semantic chunks
3. **Persists** everything to a local `.codesift/` store with stable symbol IDs
4. **Updates incrementally** on file changes (target: sub-second per save)
5. **Queries** via a structural DSL, semantic search, and hybrid retrieval
6. **Exposes** results through CLI and MCP for humans and agents

## Inspiration: IntelliJ for agents

**codesift is the brain and engine of IntelliJ, rebuilt for agents** — not the GUI, but the indexing, analysis, inspections, and refactor intelligence that make professional development safe and fast.

IntelliJ's power comes from separating parsing (PSI), persistent indexes (stubs), reference resolution, and global analysis. codesift adopts that **intelligence stack** and exposes it via MCP and CLI so agents can write **idiomatic, safe, correct, secure, performant, efficient, and scalable** code.

| IntelliJ concept | codesift adoption |
|------------------|-------------------|
| PSI (Program Structure Interface) | Unified AST + symbol layer per language |
| Stub indexes | Persistent symbol/ref/call indexes in fjall |
| FileBasedIndex | Per-file metadata keyed by path + content hash |
| Find Usages / ReferencesSearch | `refs:` and `callers:` queries |
| Inspections / Qodana | Pluggable analysis + CI gates (phased) |
| Refactorings | Preview and impact (phased); apply elsewhere |
| Type inference / resolve | Per language, phased |

Near term: **index + query engine**. Long term: **full code intelligence engine** for agents and CI. Details: [big-picture.md](big-picture.md).

## Success criteria

| Criterion | Target |
|-----------|--------|
| Incremental update latency | Sub-second on typical file save (goal, not guarantee until benchmarked) |
| Query modes | Structural + semantic + hybrid |
| Agent integration | MCP tools return structured symbol IDs and locations |
| Code quality | Agents aided toward idiomatic, safe, correct, secure, performant, efficient, scalable changes (phased) |
| Offline / local-first | Embedded storage and local embeddings by default |
| Multi-consumer | Same index serves CLI, MCP, and future library API |

## Principles

1. **Stable symbol IDs** — agents and tools can reference symbols across edits without path-only fragility
2. **Specs before code** — every crate traces to a spec or ADR
3. **Embedded by default** — no required external services for core operation
4. **Language depth where it matters** — Rust-first MVP, tree-sitter breadth later
5. **Open source** — Apache-2.0, public capability coverage, contributor-friendly docs

## See also

- [Big picture](big-picture.md) — long-term intelligence vision for agents and CI
- [Goals and non-goals](goals-and-non-goals.md)
- [Use cases](use-cases.md)
- [Capabilities](../capabilities.md)
- [Architecture overview](../architecture/overview.md)
