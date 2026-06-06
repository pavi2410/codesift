# 0002. Rust as Implementation Language

**Status:** accepted

**Date:** 2026-06-06

## Context

codesift is an indexing engine requiring high performance, memory safety, embedded storage, and integration with the Rust ecosystem (tree-sitter bindings, rust-analyzer syntax, MCP SDK). We need a language that supports CLI tools, long-running watch processes, and future library embedding.

## Decision

Implement codesift in **Rust** as a Cargo workspace with multiple crates (see [future-crate-layout.md](../architecture/future-crate-layout.md)).

## Consequences

### Positive

- Direct use of mature crates: fjall, usearch, tantivy, tree-sitter, rmcp, fastembed
- Memory safety without GC pauses — important for indexing large repos
- Single binary distribution for CLI and MCP
- Strong alignment with rust-analyzer syntax for Rust MVP
- Active systems-programming ecosystem for embedded databases and search

### Negative

- Slower initial development than Python/TypeScript for some tasks
- C++ dependencies in usearch require build tooling
- Steeper contributor onboarding for non-Rust developers

## Alternatives considered

| Alternative | Why not |
|-------------|---------|
| Go | Weaker embedded search/vector ecosystem; no ra_ap_syntax |
| TypeScript | Poor fit for embedded KV and CPU-heavy indexing |
| C++ | Manual memory safety; higher maintenance burden |
| Zig | Immature search/indexing ecosystem |

## References

- [technology-stack.md](../architecture/technology-stack.md)
- [rust-ecosystem.md](../references/rust-ecosystem.md)
