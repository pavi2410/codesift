# 0003. tree-sitter + ra_ap_syntax for Parsing

**Status:** accepted

**Date:** 2026-06-06

## Context

codesift needs incremental, error-tolerant parsing for multiple languages. Rust MVP requires deep structural fidelity (macros, editions, impl blocks). Parser choice affects PSI quality, watch-mode performance, and multi-language rollout.

## Decision

- **Default multi-language parser:** tree-sitter 0.26.x with per-language grammars and `.scm` queries
- **Rust MVP parser:** `ra_ap_syntax` (rust-analyzer syntax crate) for lossless CST and incremental reparse
- **Deferred:** `syn` (not incremental), custom hand-written parsers

## Consequences

### Positive

- tree-sitter provides broad language coverage for Phase 4
- ra_ap_syntax gives best-in-class Rust structure for MVP
- Both support incremental reparse for watch mode
- tree-sitter query DSL is maintainable for symbol extraction

### Negative

- Two parser backends to maintain for Rust until tree-sitter Rust depth catches up
- ra_ap_syntax ties Rust indexing to rust-analyzer release cadence
- tree-sitter gives syntax only — no type information

## Alternatives considered

| Alternative | Why not |
|-------------|---------|
| tree-sitter only | Shallower Rust semantics vs ra_ap_syntax |
| syn | No incremental reparse |
| rust-analyzer as library | Heavy dependency; we need index not LSP |
| Language Server protocol parse | Requires running per-language servers |

## References

- [parsing-and-psi.md](../techniques/parsing-and-psi.md)
- [multi-language-strategy.md](../techniques/multi-language-strategy.md)
