# 0007. fastembed for Local Embeddings

**Status:** draft

**Date:** 2026-06-06

## Context

Semantic search requires embedding generation. codesift prioritizes offline operation, privacy, and low agent latency. June 2026: fastembed 5.15.x provides ONNX-based local inference, built-in rerankers, and 1.2M crate downloads.

## Decision

- **Embedding library:** fastembed 5.15.x
- **Default embedding model:** `nomic-embed-text-v1.5` (768-dim)
- **Default reranker:** `BAAI/bge-reranker-base`
- **Cache:** content-hash keyed vectors in fjall `embedding_cache` keyspace
- **Remote APIs:** opt-in later, not default

**Status is draft** until benchmarked on a Rust code retrieval corpus.

## Consequences

### Positive

- No network required for semantic search
- Same library for embeddings and reranking
- Sync API suits CLI batch indexing
- GPU acceleration via ort providers (CUDA, CoreML, DirectML)

### Negative

- First index downloads ONNX models (~hundreds of MB)
- CPU embedding is slow on large repos without GPU
- Model choice may not be optimal for all languages until benchmarked

## Alternatives considered

| Alternative | Why not |
|-------------|---------|
| OpenAI / remote APIs | Privacy, latency, offline requirement |
| candle-only custom | More DIY; fastembed already wraps ONNX |
| Ollama subprocess | Extra process dependency |
| BGE-M3 only | Heavier; defer sparse+dense to Phase 2 |

## References

- [chunking-and-semantic.md](../specs/chunking-and-semantic.md)
- [semantic-retrieval.md](../techniques/semantic-retrieval.md)
