# CLI Specification

**Status:** accepted

Command-line interface for codesift. Implementation: `clap` (ADR-0005).

**MVP scope:** `index`, `query`, `symbol`, `refs`, `status`, `export`, `mcp`. `watch`, `search` deferred.

**Agent path:** use MCP (`codesift mcp`) — not CLI `query`. CLI `query` is a **debug escape hatch** for engineers and CI; agents should use typed MCP tools per [ADR 0010](../adr/0010-mcp-agent-frontend-cli-ops.md).

## Binary name

`codesift`

## Global flags

| Flag | Description |
|------|-------------|
| `--workspace <path>` | Workspace root (default: `.`) |
| `--index-path <path>` | Override `.codesift` location |
| `-v, --verbose` | Increase log verbosity |
| `-q, --quiet` | Errors only |
| `--format <fmt>` | `json`, `table`, `plain`, `ascii` (default: `table` for TTY, `json` for pipe) |
| `--no-color` | Disable ANSI in `ascii` output (CI-friendly) |

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

**Debug only** — structural query DSL for engineers and CI. Agents should use MCP tools (`find_references`, `get_callers`, etc.) instead. See [query-language.md](query-language.md).

```bash
codesift query "symbol:name=parse_query kind=function"
codesift query 'callers:of=sym://42/...' --format json
codesift refs --name parse_query --format ascii --no-color
```

| `--format` | Description |
|------------|-------------|
| `table` | Default on TTY |
| `json` | Default when piped |
| `ascii` | Tree-style human output (recommended for debug) |
| `plain` | Pretty JSON alias |

Global `--no-color` disables ANSI in `ascii` mode.

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

Find references to a symbol. Returns one hit per ref/call edge with call-site location in JSON and table output.

```bash
codesift refs sym://42/...
codesift refs --name parse_query
```

`--name` aggregates refs across all symbols with that name, including unresolved placeholder targets.

### `export`

Export index to JSONL (symbols, refs, edges).

```bash
codesift export --format jsonl --output index.jsonl
codesift export --type symbol
codesift export --type ref
codesift export --type edge
```

| `--type` value | Records exported |
|----------------|------------------|
| (default) | symbols, refs, edges |
| `symbol` | symbol records only |
| `ref` | ref records only |
| `edge` | edge/call-graph records only |

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
  "semantic_ready": false,
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
