# MCP Specification

**Status:** draft

Model Context Protocol server for AI agents. Implementation: `rmcp` stdio transport (ADR-0005).

## Server identity

| Field | Value |
|-------|-------|
| Name | `codesift` |
| Transport | stdio (primary) |
| Protocol | MCP 2025-11-25 |

## Startup

```bash
codesift mcp --workspace /path/to/repo
```

Server loads or creates index at `{workspace}/.codesift/`. Reports readiness via `index_status` tool.

## Session model

- One workspace per server process
- Index shared across tool calls
- Watch mode optional: `--watch` flag starts background indexer (future)

## Tools

### `index_status`

Returns index freshness and stats.

**Input schema:**

```json
{
  "type": "object",
  "properties": {},
  "additionalProperties": false
}
```

**Output:**

```json
{
  "workspace_rev": 42,
  "files": 1284,
  "symbols": 18420,
  "semantic_ready": true,
  "last_indexed": "2026-06-06T14:30:00Z"
}
```

### `query_structural`

Execute structural query DSL.

**Input:**

```json
{
  "type": "object",
  "properties": {
    "query": { "type": "string", "description": "Structural query string" },
    "limit": { "type": "integer", "default": 20, "maximum": 100 },
    "offset": { "type": "integer", "default": 0 }
  },
  "required": ["query"]
}
```

**Output:** Same as [query-language.md](query-language.md) structural response.

### `search_semantic`

Natural language search over code chunks.

**Input:**

```json
{
  "type": "object",
  "properties": {
    "query": { "type": "string" },
    "lang": { "type": "string" },
    "kind": { "type": "string" },
    "path": { "type": "string", "description": "Path glob" },
    "limit": { "type": "integer", "default": 10, "maximum": 50 }
  },
  "required": ["query"]
}
```

**Output:** Semantic hits from [query-language.md](query-language.md).

### `get_symbol`

Resolve symbol by ID.

**Input:**

```json
{
  "type": "object",
  "properties": {
    "symbol_id": { "type": "string" }
  },
  "required": ["symbol_id"]
}
```

### `find_references`

Find references to a symbol.

**Input:**

```json
{
  "type": "object",
  "properties": {
    "symbol_id": { "type": "string" },
    "limit": { "type": "integer", "default": 50 }
  },
  "required": ["symbol_id"]
}
```

### `get_callers`

Callers of a function.

**Input:**

```json
{
  "type": "object",
  "properties": {
    "symbol_id": { "type": "string" },
    "depth": { "type": "integer", "default": 1, "maximum": 5 }
  },
  "required": ["symbol_id"]
}
```

### `read_chunk`

Fetch exact source for a chunk.

**Input:**

```json
{
  "type": "object",
  "properties": {
    "chunk_id": { "type": "string" },
    "context_lines": { "type": "integer", "default": 0, "description": "Extra lines before/after" }
  },
  "required": ["chunk_id"]
}
```

**Output:**

```json
{
  "chunk_id": "chunk://42/9a1b",
  "path": "src/net/retry.rs",
  "content": "pub fn retry_with_backoff<...> { ... }",
  "location": { "start_line": 12, "end_line": 45 }
}
```

## Payload limits

| Limit | Value |
|-------|-------|
| Max `read_chunk` content | 32 KiB |
| Max tool response | 256 KiB |
| Default `limit` | 10 (MCP), 20 (CLI) |

Truncate with `"truncated": true` field when exceeded.

## Error responses

```json
{
  "error": {
    "code": "INDEX_NOT_FOUND",
    "message": "No index at .codesift. Run codesift index first."
  }
}
```

## Agent workflow example

```
1. index_status
2. search_semantic { "query": "auth token validation", "lang": "rust" }
3. get_symbol { "symbol_id": "sym://..." }
4. get_callers { "symbol_id": "sym://...", "depth": 2 }
5. read_chunk { "chunk_id": "chunk://..." }
```

## See also

- [cli.md](cli.md)
- [query-language.md](query-language.md)
- [../use-cases.md](../use-cases.md)
