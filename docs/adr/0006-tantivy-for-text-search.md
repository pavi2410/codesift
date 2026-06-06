# 0006. tantivy for Text Search

**Status:** accepted

**Date:** 2026-06-06

## Context

Hybrid retrieval needs BM25-style lexical search over symbol names, signatures, doc comments, and chunk text. Raw trigram indexes in fjall lack relevance ranking. June 2026: tantivy 0.26.x is mature (13M+ downloads, Lucene-inspired BM25, sub-10ms startup).

## Decision

Use **tantivy 0.26.x** for full-text and identifier search, stored at `.codesift/text/`.

Indexed fields: `name`, `signature`, `doc_comment`, `path`, `content`, facets for `language` and `kind`.

## Consequences

### Positive

- Industry-grade BM25 ranking
- Incremental indexing aligns with watch mode
- Fast cold start for CLI
- Pure Rust — consistent with project stack

### Negative

- Third index to maintain (fjall, usearch, tantivy)
- Disk footprint larger than KV-only trigrams
- Schema migrations require reindex on breaking changes

## Alternatives considered

| Alternative | Why not |
|-------------|---------|
| Trigrams in fjall | No BM25; poor relevance |
| ripgrep at query time | No persistent index; slow on large repos |
| Elasticsearch | External server; not embedded |

## References

- [storage.md](../specs/storage.md)
- [semantic-retrieval.md](../techniques/semantic-retrieval.md)
- [query-language.md](../specs/query-language.md)
