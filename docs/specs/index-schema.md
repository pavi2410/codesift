# Index Schema

**Status:** accepted

Logical schema for index records stored in fjall, usearch, and tantivy. Serialization: [postcard](../adr/0008-postcard-serialization.md) for KV records; JSON for export.

**MVP scope:** `symbols`, `refs`, `edges`, `file_state` keyspaces only. `chunks`, `embedding_cache`, usearch, and tantivy deferred.

## Versioning

Every record includes:

| Field | Type | Description |
|-------|------|-------------|
| `record_version` | `u16` | Schema version for this record type |
| `workspace_rev` | `u64` | Workspace revision at write time |

`meta.json` at index root:

```json
{
  "index_format_version": 1,
  "workspace_rev": 42,
  "created_at": "2026-06-06T12:00:00Z",
  "updated_at": "2026-06-06T14:30:00Z",
  "stack": {
    "fjall": "3.1.4",
    "usearch": "2.25.2",
    "tantivy": "0.26.1"
  },
  "file_count": 1284,
  "symbol_count": 18420
}
```

## fjall keyspaces

### `symbols`

| Key | Value |
|-----|-------|
| `symbol_id` (UTF-8) | `SymbolRecord` (postcard) |

Secondary indexes (key prefix design):

| Key pattern | Value |
|-------------|-------|
| `name:{lowercase_name}` | postcard `Vec<SymbolId>` |
| `path:{path}` | postcard `Vec<SymbolId>` |
| `kind:{kind}` | postcard `Vec<SymbolId>` |

### `refs`

Forward references: who references whom.

| Key | Value |
|-----|-------|
| `ref:{from_id}:{to_id}` | `RefRecord` (postcard) |

```json
{
  "from_id": "sym://42/src/main.rs#function:main@100:500",
  "to_id": "sym://42/src/query/parser.rs#function:parse_query@1204:1890",
  "ref_kind": "call",
  "site": { "path": "src/main.rs", "start_byte": 320, "end_byte": 340 }
}
```

### `edges`

Typed relation edges (calls, imports, implements).

| Key | Value |
|-----|-------|
| `edge:{relation}:{edge_id}` | `EdgeRecord` (postcard) |

Reverse index for callers:

| Key | Value |
|-----|-------|
| `rev:{relation}:{to_id}` | postcard `Vec<edge_id>` |

### `chunks`

| Key | Value |
|-----|-------|
| `chunk:{chunk_id}` | `ChunkRecord` (postcard) |

```json
{
  "chunk_id": "chunk://42/9a1b",
  "symbol_id": "sym://42/src/query/parser.rs#function:parse_query@1204:1890",
  "path": "src/query/parser.rs",
  "language": "rust",
  "kind": "function_body",
  "content_hash": "blake3:7f3a...",
  "start_byte": 1250,
  "end_byte": 1880,
  "text_preview": "pub fn parse_query(input: &str) -> ..."
}
```

### `file_state`

Per-file index metadata.

| Key | Value |
|-----|-------|
| `file:{path}` | `FileStateRecord` (postcard) |

```json
{
  "path": "src/query/parser.rs",
  "content_hash": "blake3:abc123...",
  "language": "rust",
  "symbol_ids": ["sym://42/..."],
  "chunk_ids": ["chunk://42/9a1b"],
  "indexed_at": "2026-06-06T14:30:00Z",
  "parse_errors": []
}
```

### `embedding_cache`

| Key | Value |
|-----|-------|
| `emb:{content_hash}` | postcard `Vec<f32>` (raw embedding) |

## usearch index

- **Key:** `chunk_id` (u64 internal or string hash)
- **Vector:** embedding from fastembed (dimension per model; default 768)
- **Metadata:** stored in fjall `chunks`; usearch holds id → vector only
- **Filters:** predicate on `language`, `path` prefix, `kind` at query time

## tantivy schema

| Field | Type | Stored | Indexed |
|-------|------|--------|---------|
| `symbol_id` | text | yes | yes |
| `chunk_id` | text | yes | yes |
| `name` | text | yes | yes |
| `signature` | text | yes | yes |
| `doc_comment` | text | yes | yes |
| `path` | text | yes | yes |
| `content` | text | yes | yes |
| `language` | text | yes | yes (facet) |
| `kind` | text | yes | yes (facet) |

## Tombstones

Deleted records write a tombstone key:

| Key | Value |
|-----|-------|
| `tombstone:{type}:{id}` | `{ "deleted_at": "ISO8601", "workspace_rev": 42 }` |

Compaction removes tombstoned entries older than N revisions (TBD at implementation).

## Export format (JSONL)

One JSON object per line for external tools:

```jsonl
{"type":"symbol","data":{...}}
{"type":"edge","data":{...}}
{"type":"chunk","data":{...}}
```

## See also

- [storage.md](storage.md)
- [symbol-model.md](symbol-model.md)
- [../adr/0008-postcard-serialization.md](../adr/0008-postcard-serialization.md)
