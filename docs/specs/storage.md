# Storage

**Status:** accepted

On-disk layout and backend configuration. ADR-0004, ADR-0006, ADR-0008.

**MVP scope:** `meta.json` + fjall `kv/` only. `vectors/`, `text/`, `embeddings/`, `snapshots/` deferred.

## Directory layout

```
.codesift/
├── meta.json
├── kv/                    # fjall database directory
├── vectors/               # usearch persisted index
│   └── index.usearch
├── text/                  # tantivy index
│   └── ...
├── embeddings/            # optional spill for large cache
└── snapshots/             # optional JSONL exports
```

Default location: `{workspace_root}/.codesift/`

Override: `--index-path` / `CODESIFT_INDEX_PATH` env (TBD at implementation).

## fjall keyspaces

| Keyspace | Contents |
|----------|----------|
| `symbols` | SymbolRecord |
| `refs` | RefRecord |
| `edges` | EdgeRecord + reverse indexes |
| `chunks` | ChunkRecord |
| `file_state` | FileStateRecord |
| `embedding_cache` | Vec<f32> by content hash |

Configuration: LZ4 compression (default), serializable transactions for batch commits.

## usearch

```json
{
  "dimensions": 768,
  "metric": "cos",
  "quantization": "f32",
  "connectivity": 16,
  "expansion_add": 128,
  "expansion_search": 64
}
```

- Persist via `index.save(path)` / `Index::load(path)`
- Filter predicates at search: `language == "rust"`, path prefix match

## tantivy

- Index path: `.codesift/text/`
- Schema: see [index-schema.md](index-schema.md)
- Commit policy: after each batch or watch debounce
- BM25 default similarity

## Serialization

| Context | Format |
|---------|--------|
| fjall values | postcard + serde |
| meta.json | serde_json (human-readable) |
| CLI/MCP output | serde_json |
| export | JSONL |

## Compaction

| Backend | Policy |
|---------|--------|
| fjall | Background LSM compaction (automatic) |
| tombstones | Remove entries older than 10 workspace_rev (TBD) |
| usearch | Rebuild on major model dimension change |
| tantivy | `index.writer.merge()` on schedule (TBD) |

## Backup

```bash
codesift export --format jsonl --output snapshot.jsonl
# or copy .codesift/ directory
```

## Migration

`meta.json.index_format_version` incremented on breaking schema changes. Migration tool: `codesift migrate` (future) or `--force` rebuild.

## See also

- [index-schema.md](index-schema.md)
- [../architecture/technology-stack.md](../architecture/technology-stack.md)
