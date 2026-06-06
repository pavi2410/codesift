# 0004. fjall + usearch Storage Stack

**Status:** accepted

**Date:** 2026-06-06

## Context

codesift needs embedded persistence for structural records (symbols, refs, edges) and vector embeddings. The store must run locally without external services, support incremental writes, and ship in a single CLI binary. June 2026 Rust ecosystem evaluation compared fjall, redb, RocksDB, usearch, hnsw-rs, and server vector DBs.

## Decision

- **Structural KV:** fjall 3.1.x with multiple keyspaces
- **Vector ANN:** usearch 2.25.x with filter predicates and save/load
- **Embedded only:** no Qdrant, Milvus, or remote vector DB as default
- **On-disk layout:** `.codesift/kv/` and `.codesift/vectors/`

Text search (tantivy) is ADR-0006 — separate concern but co-located in `.codesift/`.

## Consequences

### Positive

- Pure Rust LSM (fjall) — no C++ build for KV
- usearch offers SIMD performance and pre-filter predicates for hybrid queries
- Keyspaces map cleanly to index-schema logical tables
- Both support persistence and recovery

### Negative

- usearch has C++ core — build complexity on some platforms
- fjall is younger than RocksDB — smaller production track record
- Two storage engines to backup (fjall + usearch files)

## Alternatives considered

| Alternative | Why not |
|-------------|---------|
| RocksDB | C++ dependency, tuning burden |
| redb (primary) | Weaker bulk/incremental write performance |
| hnsw-rs | Slower; lacks filter predicates |
| Qdrant embedded | Heavier; server-oriented design |
| SQLite + vectors | Weaker ANN performance at scale |

## References

- [storage.md](../specs/storage.md)
- [index-schema.md](../specs/index-schema.md)
- [technology-stack.md](../architecture/technology-stack.md)
