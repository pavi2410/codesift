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

Other status vocabularies (specs, future ideas): [glossary.md](glossary.md#status-terms).

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
| `structural-index-rust` | Rust symbols, refs, callers; persistent `.codesift/` | [symbol-model](specs/symbol-model.md) | — | verified |
| `structural-depth` | Import graph, impl/trait edges, workspace cross-crate | [structural-depth](milestones/structural-depth.md) | `structural-index-rust` | spec-draft |
| `semantic-search` | Chunking, embeddings, hybrid BM25+vector | [chunking-and-semantic](specs/chunking-and-semantic.md) | `structural-index-rust` | spec-draft |
| `incremental-watch` | File watcher, invalidation graph, sub-second updates | [incremental-indexing](specs/incremental-indexing.md) | `structural-index-rust` | spec-draft |
| `multi-language` | tree-sitter PSI for TS, Python, Go | [multi-language-strategy](techniques/multi-language-strategy.md) | `structural-index-rust` | spec-draft |
| `vfs-workspace` | gitignore walk, hashing, workspace roots | [vfs-and-workspace](specs/vfs-and-workspace.md) | — | verified |

### Interfaces

| ID | Capability | Spec / doc | Prerequisite | Status |
|----|------------|------------|--------------|--------|
| `cli-structural` | `index`, `query`, `symbol`, `refs`, `status`, `export` | [cli](specs/cli.md) | `structural-index-rust` | verified |
| `cli-search` | `search` hybrid command | [cli](specs/cli.md) | `semantic-search` | spec-draft |
| `mcp-core-tools` | `index_status`, `query_structural`, `get_symbol`, `find_references`, `get_callers` | [mcp](specs/mcp.md) | `structural-index-rust` | spec-draft |
| `mcp-semantic-tools` | `search_semantic`, `read_chunk` | [mcp](specs/mcp.md) | `semantic-search` | spec-draft |
| `mcp-server` | `codesift mcp` stdio transport | [mcp](specs/mcp.md) | `mcp-core-tools` | spec-draft |
| `lsp-adapter` | Symbol search via LSP | [use-cases](product/use-cases.md) | `structural-index-rust` | exploring |
| `library-api` | Embeddable Rust API for IDEs | [future-crate-layout](architecture/future-crate-layout.md) | `structural-index-rust` | exploring |

### Analysis and intelligence

| ID | Capability | Spec / doc | Prerequisite | Status |
|----|------------|------------|--------------|--------|
| `analyzer-hooks` | JSONL export, pluggable rules over PSI | [inspections-and-findings](future/inspections-and-findings.md) | `structural-index-rust` | exploring |
| `inspections-unified` | Adapter findings (Clippy, Semgrep, …) on `sym://` | [inspections-and-findings](future/inspections-and-findings.md) | `structural-index-rust` | exploring |
| `quality-signals` | Duplicates, dead-code candidates, doc drift | [inspections-and-findings](future/inspections-and-findings.md) | `semantic-search` | exploring |
| `cfg-analysis` | Intra-procedural CFG (Rust first) | [complexity-metrics](future/complexity-metrics.md) | `structural-index-rust` | exploring |
| `complexity-metrics` | Cyclomatic + cognitive per function | [complexity-metrics](future/complexity-metrics.md) | `cfg-analysis` | exploring |
| `dataflow-lite` | Flow summaries, taint hints | [inspections-and-findings](future/inspections-and-findings.md) | `cfg-analysis` | exploring |
| `refactor-preview` | `preview_rename`, impact before apply | [inspections-and-findings](future/inspections-and-findings.md) | `structural-depth` | exploring |
| `entry-discovery` | Manifest index, `bootstrap` / `entrypoint:` | [inspections-and-findings](future/inspections-and-findings.md) | `structural-index-rust`, `semantic-search` | exploring |

### Platform

| ID | Capability | Spec / doc | Prerequisite | Status |
|----|------------|------------|--------------|--------|
| `purl-library-docs` | Local PURL-addressed dependency doc index | [purl-library-docs](future/purl-library-docs.md) | `semantic-search`, lockfile resolution | exploring |
| `documentation` | Product docs, specs, ADRs, architecture | [product/](product/), [README](README.md) | — | spec-draft |

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

> **Implementation focus:** `structural-index-rust` + `cli-structural` verified on the codesift workspace (2026-06-07). Next: `incremental-watch`, then `mcp-core-tools`.

Update this blurb as work shifts; it does **not** block other capabilities.

## Layer mapping (big picture)

Intellectual grouping from [big-picture.md](product/big-picture.md) — not a schedule:

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
- [product/](product/) — vision, use cases, boundaries, strategy
- [goals-and-non-goals.md](product/goals-and-non-goals.md) — boundaries
- [future/](future/) — exploring ideas before promotion here
