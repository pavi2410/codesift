# CLI Specification

**Status:** accepted

Command-line interface for codesift. Implementation: `clap` (ADR-0005).

**MVP scope:** `index`, `query`, `symbol`, `refs`, `status`, `export` only. `watch`, `search`, `mcp` deferred.

## Binary name

`codesift`

## Global flags

| Flag | Description |
|------|-------------|
| `--workspace <path>` | Workspace root (default: `.`) |
| `--index-path <path>` | Override `.codesift` location |
| `-v, --verbose` | Increase log verbosity |
| `-q, --quiet` | Errors only |
| `--format <fmt>` | `json`, `table`, `plain` (default: `table` for TTY, `json` for pipe) |

## Commands

| Command | Capability | Prerequisite |
|---------|------------|--------------|
| `index` | `structural-index-rust` | — |
| `watch` | `incremental-watch` | `structural-index-rust` |
| `query` | `cli-structural` | `structural-index-rust` |
| `search` | `cli-search` | `semantic-search` |
| `symbol` | `cli-structural` | `structural-index-rust` |
| `refs` | `cli-structural` | `structural-index-rust` |
| `export` | `cli-structural` | `structural-index-rust` |
| `status` | `cli-structural` | `structural-index-rust` |
| `mcp` | `mcp-server` | `mcp-core-tools` |

See [capabilities.md](../capabilities.md) for coverage status.

### `index`

Build or update the index.

```bash
codesift index [PATH] [OPTIONS]
```

| Option | Description |
|--------|-------------|
| `--force` | Full rebuild |
| `--no-semantic` | Structural only (skip embeddings) |
| `-j, --jobs <n>` | Parallel file workers (default: CPU count) |

**Exit codes:** 0 success, 1 partial errors, 2 fatal error

**Example output (json):**

```json
{
  "status": "complete",
  "files_indexed": 1284,
  "symbols": 18420,
  "chunks": 15200,
  "duration_ms": 45000,
  "workspace_rev": 42
}
```

### `watch`

Index and watch for changes.

```bash
codesift watch [PATH]
```

Runs until SIGINT. Prints incremental update events to stderr in verbose mode.

### `query`

Structural query. See [query-language.md](query-language.md).

```bash
codesift query "symbol:name=parse_query kind=function"
codesift query 'callers:of=sym://42/...' --format json
```

### `search`

Semantic / hybrid search.

```bash
codesift search "retry with backoff" --lang rust --limit 10
```

| Option | Description |
|--------|-------------|
| `--lang <lang>` | Filter language |
| `--kind <kind>` | Filter symbol/chunk kind |
| `--path <glob>` | Path filter |
| `--limit <n>` | Max results (default 20) |
| `--offset <n>` | Pagination offset |

### `symbol`

Lookup symbol by ID or name.

```bash
codesift symbol sym://42/src/lib.rs#function:main@0:100
codesift symbol --name main --path src/
```

### `refs`

Find references to a symbol.

```bash
codesift refs sym://42/...
codesift refs --name parse_query
```

### `export`

Export index to JSONL.

```bash
codesift export --format jsonl --output symbols.jsonl
codesift export --type symbols,edges,chunks
```

### `status`

Index health and stats.

```bash
codesift status
```

```json
{
  "workspace_rev": 42,
  "index_format_version": 1,
  "files": 1284,
  "symbols": 18420,
  "chunks": 15200,
  "semantic_ready": true,
  "last_indexed": "2026-06-06T14:30:00Z",
  "index_path": ".codesift"
}
```

### `mcp`

Start MCP server on stdio.

```bash
codesift mcp [--workspace PATH]
```

See [mcp.md](mcp.md).

## Exit codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | Partial success / warnings |
| 2 | Error |
| 3 | Index not found (suggest `index`) |

## See also

- [capabilities.md](../capabilities.md)
- [mcp.md](mcp.md)
- [query-language.md](query-language.md)
