# Structural Depth

**Capability:** `structural-depth`  
**Status:** spec-draft  
**Prerequisite:** `structural-index-rust`

Extends the structural graph: imports, trait impls, workspace cross-crate resolution. Can be implemented in parallel with `semantic-search` / `mcp-server` once the base index exists.

## Covered

| Feature | Description |
|---------|-------------|
| Import graph | `imports:` queries; module dependency view |
| impl/trait relations | `implements` edges; `impl Trait for Type` |
| Cross-crate basics | Resolve `use` across workspace crates (not crates.io) |
| Unresolved ref tracking | `resolved: false` refs for agent awareness |

## Query additions

```
imports:std::io
implements:trait=Display
symbol:qualified_name=codesift_query::parse_query
```

## Cross-crate resolution (workspace)

For Cargo workspaces:

1. Build crate graph from `Cargo.toml` workspace members
2. Map `use crate::module` to on-disk paths
3. Resolve refs across member crates (syntax-level)

**Not in scope:** resolving external crates.io dependencies.

## Acceptance criteria

1. `implements:trait=Display` returns impl blocks for a known type
2. Import graph export includes cross-crate edges in workspace
3. Unresolved refs reported in `file_state.parse_errors` or ref records

## Prerequisites

- `structural-index-rust` verified
- Cargo metadata parsing in `codesift-core`

## See also

- [capabilities.md](../capabilities.md)
- [structural-indexing.md](../techniques/structural-indexing.md)
- [symbol-model.md](../specs/symbol-model.md)
