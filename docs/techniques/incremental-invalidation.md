# Incremental Invalidation

Dependency tracking for efficient re-indexing when files change.

## Problem

When `foo.rs` changes, which other files need re-indexing?

- **Same file only:** sufficient if changes are local (body edit, same symbols)
- **Dependent files:** needed if exports changed (signature, visibility, renamed symbol)

## Invalidation graph

Directed edges:

```
file A ──depends_on──► file B
```

`B` exports symbol S; `A` references S.

Built from:

- `refs` and `imports` edges during indexing
- Reverse map: `symbol_id → Vec<file_path>` referencing it

## Invalidation algorithm

On file `F` change:

1. Reparse `F`; diff symbols → `removed`, `added`, `changed`
2. For each `removed` or `changed` symbol:
   - Lookup `referrers(symbol_id)` → set of files `D`
3. Re-index `F` fully
4. For each file in `D`:
   - Re-extract refs/calls (may not need full reparse if only refs changed)
5. Bump `workspace_rev` if any public symbol ID changed

## Same-file incremental

When only function bodies change (same spans/names):

- Skip cross-file invalidation
- Update chunks + embeddings for changed spans only
- Update call edges within file

**Detection:** symbol diff shows only `body_changed` flag, not `signature_changed`.

## workspace_rev bump rules

| Event | Bump? |
|-------|-------|
| New file indexed | yes |
| Symbol deleted | yes |
| Symbol renamed or moved | yes |
| Body-only edit | no (chunk IDs may change; symbol IDs stable) |
| Comment-only edit | no |

## Full rebuild triggers

- `meta.json` format version mismatch
- Corrupt fjall database
- User runs `codesift index --force`
- Migration failure

## See also

- [../specs/incremental-indexing.md](../specs/incremental-indexing.md)
- [structural-indexing.md](structural-indexing.md)
