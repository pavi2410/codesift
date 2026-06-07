# Capability Coverage

**Primary planning doc for codesift.** Tracks **what** we build and whether it is covered — not **when** or in what order it ships.

Implementation sequence is a project-management choice (issues, board). This matrix is the **source of truth for feature coverage**.

## Status legend

| Status | Meaning |
|--------|---------|
| `exploring` | Idea or scratchpad; no implementation contract |
| `spec-draft` | Spec written; may change |
| `spec-accepted` | Locked for implementation |
| `not-started` | Spec ready; no code |
| `implemented` | Code exists; may lack full verification |
| `verified` | Meets acceptance criteria in [milestones/](milestones/) |

## Coverage prerequisites

Capabilities may depend on others (**logical** coverage, not schedule):

```
structural-index-rust ─┬─► structural-depth
                       ├─► mcp-core-tools
                       ├─► cli-structural
                       └─► incremental-watch

semantic-search ───────┬─► mcp-semantic-tools
                       └─► cli-search

cfg-analysis ──────────┬─► complexity-metrics
                       └─► dataflow-lite (future)

lockfile-resolution ───► purl-library-docs
```

## Coverage matrix

### Indexing and storage

| ID | Capability | Spec / doc | Prerequisite | Status |
|----|------------|------------|--------------|--------|
| `structural-index-rust` | Rust symbols, refs, callers; persistent `.codesift/` | [symbol-model](specs/symbol-model.md), [index-schema](specs/index-schema.md), [storage](specs/storage.md) | — | spec-draft |
| `structural-depth` | Import graph, impl/trait edges, workspace cross-crate | [structural-depth](milestones/structural-depth.md), [structural-indexing](techniques/structural-indexing.md) | `structural-index-rust` | spec-draft |
| `semantic-search` | Chunking, embeddings, hybrid BM25+vector | [chunking-and-semantic](specs/chunking-and-semantic.md), [semantic-retrieval](techniques/semantic-retrieval.md) | `structural-index-rust` | spec-draft |
| `incremental-watch` | File watcher, invalidation graph, sub-second updates | [incremental-indexing](specs/incremental-indexing.md) | `structural-index-rust` | spec-draft |
| `multi-language` | tree-sitter PSI for TS, Python, Go | [multi-language-strategy](techniques/multi-language-strategy.md) | `structural-index-rust` | spec-draft |
| `vfs-workspace` | gitignore walk, hashing, workspace roots | [vfs-and-workspace](specs/vfs-and-workspace.md) | — | spec-draft |

### Interfaces

| ID | Capability | Spec / doc | Prerequisite | Status |
|----|------------|------------|--------------|--------|
| `cli-structural` | `index`, `query`, `symbol`, `refs`, `status`, `export` | [cli](specs/cli.md), [structural-index-rust](milestones/structural-index-rust.md) | `structural-index-rust` | spec-draft |
| `cli-search` | `search` hybrid command | [cli](specs/cli.md) | `semantic-search` | spec-draft |
| `mcp-core-tools` | `index_status`, `query_structural`, `get_symbol`, `find_references`, `get_callers` | [mcp](specs/mcp.md) | `structural-index-rust` | spec-draft |
| `mcp-semantic-tools` | `search_semantic`, `read_chunk` | [mcp](specs/mcp.md) | `semantic-search` | spec-draft |
| `mcp-server` | `codesift mcp` stdio transport | [mcp](specs/mcp.md), ADR-0005 | `mcp-core-tools` | spec-draft |
| `lsp-adapter` | Symbol search via LSP | [use-cases](use-cases.md) | `structural-index-rust` | exploring |
| `library-api` | Embeddable Rust API for IDEs | [future-crate-layout](architecture/future-crate-layout.md) | `structural-index-rust` | exploring |

### Analysis and intelligence

| ID | Capability | Spec / doc | Prerequisite | Status |
|----|------------|------------|--------------|--------|
| `analyzer-hooks` | JSONL export, pluggable rules over PSI | [use-cases](use-cases.md), [big-picture](big-picture.md) | `structural-index-rust` | exploring |
| `inspections-unified` | Adapter findings (Clippy, Semgrep, …) on `sym://` | [big-picture](big-picture.md) Layer 3 | `structural-index-rust` | exploring |
| `quality-signals` | Duplicates, dead-code candidates, doc drift | [big-picture](big-picture.md) Layer 2 | `semantic-search` | exploring |
| `cfg-analysis` | Intra-procedural CFG (Rust first) | [complexity-metrics](future/complexity-metrics.md) | `structural-index-rust` | exploring |
| `complexity-metrics` | Cyclomatic + cognitive per function | [complexity-metrics](future/complexity-metrics.md) | `cfg-analysis` | exploring |
| `dataflow-lite` | Flow summaries, taint hints | [big-picture](big-picture.md) Layer 5 | `cfg-analysis` | exploring |
| `refactor-preview` | `preview_rename`, impact before apply | [big-picture](big-picture.md) Layer 6 | `structural-depth` | exploring |
| `entry-discovery` | Manifest index, `bootstrap` / `entrypoint:` | [big-picture](big-picture.md) | `structural-index-rust`, `semantic-search` | exploring |

### Platform

| ID | Capability | Spec / doc | Prerequisite | Status |
|----|------------|------------|--------------|--------|
| `purl-library-docs` | Local PURL-addressed dependency doc index | [purl-library-docs](future/purl-library-docs.md) | `semantic-search`, lockfile resolution | exploring |
| `documentation` | Vision, specs, ADRs, architecture | [docs/README](README.md) | — | spec-draft |

## Minimum agent-usable coverage

Named bundle for “first useful for agents” — **not** a mandatory build sequence. Check off as each capability reaches `verified`:

| Capability | In bundle? |
|------------|------------|
| `structural-index-rust` | yes |
| `cli-structural` | yes |
| `mcp-server` + `mcp-core-tools` | yes |
| `semantic-search` | yes |
| `mcp-semantic-tools` | yes |
| `incremental-watch` | recommended |
| `structural-depth` | recommended |
| `inspections-unified` | later |

## Current focus (optional)

> **Documentation** capability is largely complete. **Next implementation focus:** `structural-index-rust` + `cli-structural` — unless you choose to parallelize `semantic-search` / `mcp-server` once prerequisites are met.

Update this blurb as work shifts; it does **not** block other capabilities.

## Layer mapping (big picture)

Intellectual grouping from [big-picture.md](big-picture.md) — not a schedule:

| Layer | Capabilities |
|-------|--------------|
| Layer 1 — Discovery | `structural-index-rust`, `semantic-search`, `mcp-*`, `cli-*`, `entry-discovery` |
| Layer 2 — Quality signals | `quality-signals`, `complexity-metrics` |
| Layer 3 — Inspections | `inspections-unified`, `analyzer-hooks` |
| Layer 4 — Resolve depth | `structural-depth`, type-aware resolve (future) |
| Layer 5 — Data flow | `dataflow-lite`, `cfg-analysis` |
| Layer 6 — Refactor preview | `refactor-preview` |

## See also

- [milestones/](milestones/) — acceptance criteria per capability area
- [roadmap.md](roadmap.md) — pointer to this doc (phases retired)
- [goals-and-non-goals.md](goals-and-non-goals.md) — boundaries
- [future/](future/) — exploring ideas before promotion here
