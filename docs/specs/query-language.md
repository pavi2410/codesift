# Query Language

**Status:** draft

Structural query DSL, semantic search, and hybrid retrieval. See [chunking-and-semantic.md](chunking-and-semantic.md) for embedding pipeline.

## Query modes

| Mode | Entry | Example |
|------|-------|---------|
| Structural | `codesift query` | `symbol:name=parse_query kind=function` |
| Semantic | `codesift search` | `"exponential backoff retry"` |
| Hybrid | `codesift search --structural` | semantic + `lang:rust kind:function` |

## Structural grammar

Whitespace-separated filters. All filters are ANDed unless `OR` is specified (TBD at implementation).

### Filters

| Filter | Syntax | Description |
|--------|--------|-------------|
| Symbol name | `symbol:name={pattern}` | Exact or glob (`*`) |
| Kind | `kind:{kind}` | `function`, `type`, `trait`, ... |
| Path | `path:{glob}` | File path glob |
| Language | `lang:{lang}` | `rust`, `typescript`, ... |
| References | `refs:to={symbol_id}` | Symbols referencing target |
| Callers | `callers:of={symbol_id}` | Callers of function |
| Callees | `callees:of={symbol_id}` | Functions called by target |
| Imports | `imports:{pattern}` | Import statements matching |
| Depth | `depth={n}` | Graph traversal depth (default 1) |

### Examples

```
symbol:name=UserService kind=function
refs:to=sym://42/src/models.rs#type:User@100:200
callers:of=sym://42/src/db.rs#function:connect@50:400 depth=3
path:src/** lang:rust kind=trait
symbol:name=*Error* kind=type
```

## Semantic search

Natural language string; optional structural filters as flags:

```bash
codesift search "handle websocket reconnection" --lang rust --kind function --limit 20
```

### Pipeline

1. Embed query (fastembed)
2. usearch ANN top-k (with structural pre-filters)
3. tantivy BM25 boost on identifiers/docs
4. fastembed reranker (`bge-reranker-base`)
5. Structural boosts: same module +0.1, recency +0.05, kind match +0.05

## Response format

### Structural query response

```json
{
  "query": "symbol:name=parse_query kind=function",
  "workspace_rev": 42,
  "took_ms": 12,
  "hits": [
    {
      "symbol": {
        "id": "sym://42/src/query/parser.rs#function:parse_query@1204:1890",
        "kind": "function",
        "name": "parse_query",
        "path": "src/query/parser.rs",
        "location": {
          "start_line": 45,
          "end_line": 72
        },
        "signature": "pub fn parse_query(input: &str) -> Result<QueryAst, ParseError>"
      },
      "score": 1.0
    }
  ],
  "total": 1
}
```

### Semantic search response

```json
{
  "query": "exponential backoff retry",
  "workspace_rev": 42,
  "took_ms": 145,
  "hits": [
    {
      "chunk_id": "chunk://42/9a1b",
      "symbol_id": "sym://42/src/net/retry.rs#function:retry_with_backoff@200:800",
      "path": "src/net/retry.rs",
      "language": "rust",
      "kind": "function_body",
      "score": 0.89,
      "snippet": "pub fn retry_with_backoff<F, T>(mut f: F, config: RetryConfig) -> Result<T, Error>\nwhere\n    F: FnMut() -> Result<T, Error>,\n{",
      "location": {
        "start_line": 12,
        "end_line": 45
      }
    }
  ],
  "total": 15,
  "limit": 20,
  "offset": 0
}
```

### Callers response

```json
{
  "query": "callers:of=sym://42/src/query/parser.rs#function:parse_query@1204:1890",
  "workspace_rev": 42,
  "took_ms": 8,
  "hits": [
    {
      "caller": {
        "id": "sym://42/src/main.rs#function:main@100:500",
        "name": "main",
        "path": "src/main.rs"
      },
      "call_site": {
        "path": "src/main.rs",
        "start_line": 28,
        "start_column": 4
      },
      "depth": 1
    }
  ],
  "total": 3
}
```

## Pagination

| Parameter | Default | Max |
|-----------|---------|-----|
| `limit` | 20 | 100 |
| `offset` | 0 | — |

MCP tools use the same limits; see [mcp.md](mcp.md).

## Errors

```json
{
  "error": {
    "code": "INVALID_QUERY",
    "message": "unknown filter 'foo:'",
    "query": "foo:bar"
  }
}
```

| Code | Description |
|------|-------------|
| `INVALID_QUERY` | Parse error |
| `SYMBOL_NOT_FOUND` | ID does not exist in current revision |
| `INDEX_STALE` | `workspace_rev` mismatch; re-index suggested |
| `SEMANTIC_UNAVAILABLE` | Embeddings not built |

## See also

- [symbol-model.md](symbol-model.md)
- [cli.md](cli.md)
- [mcp.md](mcp.md)
