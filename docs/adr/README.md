# Architecture Decision Records

This directory contains Architecture Decision Records (ADRs) for codesift. ADRs capture significant technical decisions with context, consequences, and alternatives.

## Format

Based on [Michael Nygard's ADR template](https://cognitect.com/blog/2011/11/15/documenting-architecture-decisions).

Each ADR includes:

- **Status**: proposed | accepted | deprecated | superseded
- **Context**: What forces are at play?
- **Decision**: What did we decide?
- **Consequences**: What becomes easier or harder?
- **Alternatives considered**: What else was evaluated?

## When to write an ADR

Write an ADR when a decision:

- Is hard to reverse (storage format, public API, protocol)
- Affects multiple components or crates
- Has meaningful trade-offs worth documenting
- Will confuse future contributors if undocumented

Do **not** write ADRs for trivial choices (e.g. "use `thiserror` for errors") unless they set a project-wide precedent.

## Numbering

- Sequential four-digit prefix: `0001`, `0002`, …
- Filename: `NNNN-short-title.md`
- Never reuse numbers; supersede with a new ADR

## Template

```markdown
# NNNN. Title

**Status:** proposed

**Date:** YYYY-MM-DD

## Context

…

## Decision

…

## Consequences

### Positive

- …

### Negative

- …

## Alternatives considered

| Alternative | Why not |
|-------------|---------|
| … | … |

## References

- [Related spec](../specs/…)
```

## Index

| ADR | Title | Status |
|-----|-------|--------|
| [0001](0001-record-architecture-decisions.md) | Record architecture decisions | accepted |
| [0002](0002-rust-as-implementation-language.md) | Rust as implementation language | accepted |
| [0003](0003-tree-sitter-for-parsing.md) | tree-sitter + ra_ap_syntax for parsing | accepted |
| [0004](0004-storage-and-retrieval-stack.md) | fjall + usearch storage stack | accepted |
| [0005](0005-cli-and-mcp-as-primary-interfaces.md) | CLI + MCP as primary interfaces | accepted |
| [0006](0006-tantivy-for-text-search.md) | tantivy for BM25 text search | accepted |
| [0007](0007-fastembed-for-local-embeddings.md) | fastembed for local embeddings | draft |
| [0008](0008-postcard-serialization.md) | postcard for index serialization | accepted |
| [0009](0009-query-filter-syntax.md) | Query filter syntax (first-equals) | accepted |
