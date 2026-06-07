# DF-001-MCP: parse_query callstack and usages (MCP only)

**Automated test:** `crates/codesift-mcp/tests/mcp_tools.rs`

## Agent setup

Use **Agent mode with MCP enabled** (Cursor or OpenCode). Pass **Prompt** only — no CLI, no grep/rg, no ground-truth docs. Orchestrator scores after the run.

**Not valid:** shell-only Task subagents (no MCP tools).

**Forbidden:** grep, ripgrep, `codesift query`, reading this file or other dogfooding ground-truth docs during the exercise.

**Allowed:** MCP tools `get_symbol`, `find_references`, `get_callers`, `index_status` only.

## Prerequisite

```bash
mise trust
mise run cli index .
```

MCP launcher: **`mise run mcp`** — see [mcp.md](../../specs/mcp.md).

| Client | Config file |
|--------|-------------|
| Cursor | [`.cursor/mcp.json`](../../../.cursor/mcp.json) |
| OpenCode | [`opencode.json`](../../../opencode.json) (preferred for cross-client baseline) |

Reload MCP in Cursor after first clone.

## Prompt

Using codesift MCP tools only (no CLI, no grep), find the callstack and usages of `parse_query`.

## Ground truth

| Item | Expected |
|------|----------|
| Definition | `parser::parse_query`, `crates/codesift-query/src/parser.rs` |
| Usages | `main.rs` (query + refs paths), `parser.rs` (tests), `executor.rs` (tests) |
| Production callstack | `main` → `run` → `parse_query` |

## Target MCP sequence (≤3 tool calls)

```
1. find_references { "name": "parse_query" }
2. get_callers { "name": "parse_query", "depth": 2 }
```

Optional third: `get_symbol { "name": "parse_query", "kind": "function" }` for definition.

## Baseline runs

| Date | Commit | Runner | MCP tool calls | Wall time | Usages found | Callstack | Notes |
|------|--------|--------|----------------|-----------|--------------|-----------|-------|
| | post-mcp | 05dc01c | generalPurpose | **3** | ~3–5 s | 11 | validate chain (depth 2) | hit ≤3 target |

Compare with [CLI-only DF-001](001-parse-query-usages.md) (~38 CLI invocations pre-MCP).

## Ripgrep baseline

Same as DF-001 — ~1 command, text only, no callstack.
