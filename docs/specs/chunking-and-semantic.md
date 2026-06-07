# Chunking and Semantic Index

**Status:** draft

Chunk boundaries, embeddings, and reranking. ADR-0007 (draft), ADR-0006.

## Chunk types

| `kind` | Boundary |
|--------|----------|
| `function_body` | Function item including signature line |
| `type_def` | struct/enum/union |
| `impl_block` | impl { ... } |
| `module` | File-level fallback |
| `doc_comment` | Orphan or attached doc block |
| `markdown_section` | `##` heading through next heading |

## Chunk record

See [index-schema.md](index-schema.md). Key fields:

- `chunk_id`, `symbol_id` (optional), `content_hash`, `path`, `language`, `kind`
- `start_byte`, `end_byte`, `text_preview` (first 200 chars)

## Embedding (fastembed)

| Setting | Default |
|---------|---------|
| Model | `nomic-embed-text-v1.5` |
| Dimensions | 768 |
| Cache key | `blake3:{hash}` in `embedding_cache` keyspace |
| Batch size | 256 |

### When to embed

| Mode | Behavior |
|------|----------|
| `codesift index` | Embed all new/changed chunks |
| `codesift watch` | Embed on idle (debounce 2s) |
| MCP `search_semantic` | On-demand if chunk missing embedding |

## Reranking pipeline

After usearch + tantivy fusion:

1. Take top 50 candidates
2. Run `BAAI/bge-reranker-base` via fastembed
3. Apply structural boosts:

| Boost | Condition | Delta |
|-------|-----------|-------|
| Same module | chunk path dirname matches query context | +0.1 |
| Recency | file indexed in last hour | +0.05 |
| Kind match | filter `kind=function` and chunk is function | +0.05 |

## Optional: BGE-M3 sparse+dense (`semantic-search` enhancement)

Optional `fastembed` BGE-M3 for hybrid sparse+dense in single model. Deferred until MVP semantic search validated.

## Example: semantic index result

```json
{
  "chunk_id": "chunk://42/9a1b",
  "content_hash": "blake3:7f3a2b...",
  "embedding_model": "nomic-embed-text-v1.5",
  "vector_dims": 768,
  "indexed_in_usearch": true,
  "indexed_in_tantivy": true
}
```

## See also

- [query-language.md](query-language.md)
- [../techniques/semantic-retrieval.md](../techniques/semantic-retrieval.md)
- [../adr/0007-fastembed-for-local-embeddings.md](../adr/0007-fastembed-for-local-embeddings.md)
