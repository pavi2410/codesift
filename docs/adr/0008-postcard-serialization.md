# 0008. postcard for Index Serialization

**Status:** accepted

**Date:** 2026-06-06

## Context

Index records (symbols, refs, edges, chunks) are stored millions of times in fjall. Serialization format affects disk size, read speed, and schema evolution. Options: JSON, bincode, postcard, Protocol Buffers.

## Decision

- **fjall values:** postcard 1.x with serde
- **meta.json, CLI/MCP output, export:** serde_json / JSONL
- **Not used for KV:** JSON (too large), bincode (less compact for small records)

## Consequences

### Positive

- postcard is compact and fast for serde types
- Human-readable JSON at API boundaries for agents and debugging
- Clear separation: binary internal, JSON external

### Negative

- postcard is less common — contributors may need to learn debugging approach
- Schema changes require `record_version` migration discipline
- No cross-language schema IDL (acceptable — Rust-only engine)

## Alternatives considered

| Alternative | Why not |
|-------------|---------|
| bincode | Larger on average; less ideal for small structs |
| JSON in fjall | Disk and parse overhead |
| protobuf | Codegen friction; overkill for embedded index |
| rkyv | Zero-copy complexity not needed yet |

## References

- [index-schema.md](../specs/index-schema.md)
- [storage.md](../specs/storage.md)
