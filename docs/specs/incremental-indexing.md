# Incremental Indexing

**Status:** draft

File-level incremental update algorithm. See [../techniques/incremental-invalidation.md](../techniques/incremental-invalidation.md).

## Triggers

| Trigger | Source |
|---------|--------|
| `codesift index` | Full or incremental based on hash diff |
| `codesift watch` | `notify` file events |
| MCP `index_status` stale | Agent-requested re-index (future) |

## Algorithm

```
on_file_change(path):
  1. Read new content; compute blake3 hash
  2. Compare with file_state[path].content_hash
     - if equal: skip
  3. Incremental reparse(path, old_tree, edit)
  4. symbol_diff = diff(old_symbols, new_symbols)
  5. affected_files = referrers(symbol_diff.removed ∪ symbol_diff.changed)
  6. tombstone(symbol_diff.removed)
  7. write(symbol_diff.added ∪ symbol_diff.changed)
  8. for f in affected_files: reindex_refs(f)
  9. update_chunks(path) for changed spans
  10. re_embed changed chunks (cache miss only)
  11. update file_state[path]
  12. if symbol_ids_changed: workspace_rev += 1
```

## Symbol diff categories

| Category | Action |
|----------|--------|
| `added` | Insert symbol + indexes |
| `removed` | Tombstone + remove edges |
| `unchanged` | Skip |
| `body_changed` | Update chunks/embeddings; keep symbol ID |
| `signature_changed` | New symbol ID; tombstone old; invalidate referrers |

## Cross-file ref patch

When symbol S in file B changes signature or is removed:

1. Query `rev:refs:{S}` and `rev:calls:{S}`
2. For each referencing file F: re-run ref/call extraction only
3. Patch `refs` and `edges` keyspaces

## Batch commits

- Per-file writes buffered in memory
- Commit to fjall every N files or 100ms (TBD)
- usearch/tantivy: batch add per commit window

## Failure modes

| Failure | Recovery |
|---------|----------|
| Parse panic | Log; skip file; continue |
| fjall write error | Abort batch; retry or `--force` rebuild |
| Partial watch crash | Next `index` reconciles via hash diff |
| workspace_rev overflow | u64 — not practical concern |

## Full rebuild

`codesift index --force`:

1. Delete `.codesift/kv`, `vectors/`, `text/` (keep config)
2. `workspace_rev = 0`
3. Full index all files

## See also

- [vfs-and-workspace.md](vfs-and-workspace.md)
- [storage.md](storage.md)
