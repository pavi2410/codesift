# Semantic Retrieval

Chunking, embedding, hybrid search, and evaluation.

## Chunking principles

Chunks should be **semantic units**, not fixed token windows:

| Boundary | Content |
|----------|---------|
| Function body | Full function including signature context |
| Type definition | struct/enum/union block |
| impl block | Methods grouped |
| Module-level | Top-level items without parent |
| Doc comment | Attached `///` or `//!` blocks |
| Markdown | Per heading section |

Max chunk size: TBD at implementation (target ~512–2048 tokens).

## Embedding pipeline

1. Compute blake3 hash of chunk text
2. Lookup `embedding_cache` keyspace
3. On miss: fastembed `nomic-embed-text-v1.5` (default)
4. Store vector in usearch; cache in fjall
5. Index text fields in tantivy

## Hybrid retrieval

```
Query text
    │
    ├─► fastembed embed(query)
    │       └─► usearch search(k=50, filters)
    │
    ├─► tantivy BM25(query terms, fields=name,content,doc_comment)
    │
    └─► merge scores (weighted sum)
            └─► fastembed reranker top-20
                    └─► structural boosts
                            └─► final ranking
```

### Score fusion (initial weights — tune in benchmarks)

| Signal | Weight |
|--------|--------|
| usearch cosine | 0.5 |
| tantivy BM25 (normalized) | 0.3 |
| reranker | replaces top-k ordering |
| same module boost | +0.1 |
| recency (if indexed recently) | +0.05 |

## Structural pre-filters

Applied in usearch predicates and tantivy facets:

- `lang:rust`
- `path:src/net/**`
- `kind:function`

## Evaluation metrics

Track on benchmark corpus (TBD):

| Metric | Description |
|--------|-------------|
| Precision@k | Relevant in top k |
| MRR | Mean reciprocal rank of first relevant |
| Recall@k | Coverage for code search tasks |

Reference: MTEB-style code retrieval benchmarks; internal Rust crate corpus.

## See also

- [../specs/chunking-and-semantic.md](../specs/chunking-and-semantic.md)
- [../adr/0007-fastembed-for-local-embeddings.md](../adr/0007-fastembed-for-local-embeddings.md)
- [../adr/0006-tantivy-for-text-search.md](../adr/0006-tantivy-for-text-search.md)
