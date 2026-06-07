# MCP Specification

**Status:** accepted (v1 core tools)

Model Context Protocol server for AI agents. Implementation: `rmcp` stdio transport (ADR-0005, ADR-0010).

**v1 scope:** `index_status`, `get_symbol`, `find_references`, `get_callers` only. Agents should use these named tools — not the CLI query DSL. `query_structural` is **deferred** (DSL discovery tax).

## Server identity

| Field | Value |
|-------|-------|
| Name | `codesift` |
| Transport | stdio (primary) |
| Protocol | MCP 2025-11-25 |

## Startup

```bash
mise run mcp
```

Equivalent: `mise run cli -- --workspace . mcp` or `codesift --workspace /path/to/repo mcp`.

Server requires an existing index at `{workspace}/.codesift/`. Run `mise run cli index .` first.

Indexing writes **`query.snap`** — a lock-free read snapshot used by MCP (and queries when present). MCP never opens the fjall database, so reconnects and parallel readers do not hit `FjallError: Locked`.

### Canonical launcher

| Item | Value |
|------|-------|
| Command | `mise run mcp` |
| Task | [`mise.toml`](../../mise.toml) → `[tasks.mcp]` |

All agent clients should spawn **`mise run mcp`** from the repo root (builds release CLI if needed, then stdio MCP).

### Cursor (this repo)

Project config: [`.cursor/mcp.json`](../../.cursor/mcp.json) — committed for team onboarding.

```json
{
  "mcpServers": {
    "codesift": {
      "command": "mise",
      "args": ["run", "mcp"]
    }
  }
}
```

Requires [mise](https://mise.jdx.dev/) on `PATH`. Reload MCP in Cursor after first clone (`mise trust` in repo).

Use **Agent mode** (not Ask) so MCP tools appear. Subagents spawned from Agent inherit MCP tools.

### OpenCode

Project config: [`opencode.json`](../../opencode.json) at repo root.

```json
{
  "$schema": "https://opencode.ai/config.json",
  "mcp": {
    "codesift": {
      "type": "local",
      "command": ["mise", "run", "mcp"],
      "enabled": true
    }
  }
}
```

OpenCode merges project `opencode.json` over global config. Prefer OpenCode for **cross-client dogfood** baselines (see [agent-harness-next.md](../future/agent-harness-next.md)).

### Other clients

Same stdio process; only the config envelope differs (VS Code: `.vscode/mcp.json` with `servers` key).

## Session model

- One workspace per server process
- Index shared across tool calls
- Watch mode optional: `--watch` flag (future)

## Tools (v1)

| Tool | Capability | Prerequisite |
|------|------------|--------------|
| `index_status` | `mcp-core-tools` | `structural-index-rust` |
| `get_symbol` | `mcp-core-tools` | `structural-index-rust` |
| `find_references` | `mcp-core-tools` | `structural-index-rust` |
| `get_callers` | `mcp-core-tools` | `structural-index-rust` |

Deferred: `query_structural`, `search_semantic`, `read_chunk`, `explore_symbol`.

See [capabilities.md](../capabilities.md) for coverage status.

### `index_status`

Returns index freshness and stats.

**Input:** `{}`

**Output:**

```json
{
  "workspace_rev": 42,
  "index_format_version": 1,
  "files": 1284,
  "symbols": 18420,
  "semantic_ready": false,
  "last_indexed": "2026-06-06T14:30:00Z",
  "index_path": "/path/to/repo/.codesift"
}
```

### `get_symbol`

Resolve symbol(s) by **name** (preferred) or `symbol_id`.

**Input (name-first):**

```json
{
  "name": "parse_query",
  "kind": "function",
  "path": "crates/codesift-query"
}
```

**Input (by id):**

```json
{
  "symbol_id": "sym://1/crates/codesift-query/src/parser.rs#function:parse_query@0:10"
}
```

Provide `name` **or** `symbol_id`. Optional `kind` and `path` disambiguate when multiple symbols share a name.

**Output:** Array of symbol records (same shape as [query-language.md](query-language.md)).

### `find_references`

Find references to a symbol by **name** or `symbol_id`.

**Input:**

```json
{
  "name": "parse_query",
  "kind": "function"
}
```

**Output:**

```json
{
  "hits": [
    {
      "symbol": { "name": "run", "path": "crates/codesift-cli/src/main.rs", "...": "..." },
      "site": { "path": "crates/codesift-cli/src/main.rs", "start_line": 110, "start_column": 4 },
      "ref_kind": "call"
    }
  ],
  "total": 12
}
```

### `get_callers`

Callers of a function up the call graph (BFS).

**Input:**

```json
{
  "name": "parse_query",
  "depth": 2
}
```

`depth` defaults to `1`, maximum `5`.

**Output:** Same hit shape as `find_references`, with optional `depth` on each hit.

## Payload limits

| Limit | Value |
|-------|-------|
| Max tool response | 256 KiB |
| Default result size | unbounded in v1 (truncate in v2) |

## Error responses

Tool errors use MCP `invalid_params` with JSON body:

```json
{
  "error": {
    "code": "INDEX_NOT_FOUND",
    "message": "index not found; run `codesift index` first"
  }
}
```

| Code | When |
|------|------|
| `INDEX_NOT_FOUND` | No `.codesift` index |
| `SYMBOL_NOT_FOUND` | Name/id not in index |
| `INVALID_INPUT` | Missing name/id, malformed symbol id |

Never expose raw serde or store errors to agents.

## Agent workflow example (DF-001)

```
1. find_references { "name": "parse_query" }
2. get_callers { "name": "parse_query", "depth": 2 }
```

Optional: `get_symbol { "name": "parse_query", "kind": "function" }` for definition location.

Target: **≤3 tool calls** for parse_query usages + callstack.

## See also

- [capabilities.md](../capabilities.md)
- [cli.md](cli.md) — ops/debug CLI
- [query-language.md](query-language.md)
- [../adr/0010-mcp-agent-frontend-cli-ops.md](../adr/0010-mcp-agent-frontend-cli-ops.md)
- [../dogfooding/exercises/001-parse-query-usages-mcp.md](../dogfooding/exercises/001-parse-query-usages-mcp.md)
