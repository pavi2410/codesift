# 0010. MCP as Agent Frontend, CLI for Ops and Debug

**Status:** accepted

**Date:** 2026-06-07

## Context

ADR 0005 established CLI and MCP as co-primary interfaces. Dogfooding (DF-001) showed agents burn dozens of CLI invocations discovering query DSL syntax (`callers:of=`, `refs:to=`, mixed delimiters). The structural index and query engine work correctly when the query is valid; the **CLI query string is a poor agent discovery surface**.

codesift is positioned as **IntelliJ-grade intelligence for agents** — agents should call typed IDE-like tools (find usages, callers), not invent filter grammar.

## Decision

- **MCP** is the **primary frontend for AI agents** — typed tools with JSON Schema (`get_symbol`, `find_references`, `get_callers`, `index_status` in v1).
- **CLI** is the **ops and debug surface** — `index`, `status`, `export`, plus escape hatches (`symbol`, `refs --name`, `query` for engineers and CI).
- **Do not** expose `query_structural` as an MCP tool in v1 (DSL discovery tax). The query DSL remains an internal/power-user CLI mechanism.
- **Name-first tool inputs** — MCP tools accept `name` + optional `kind`/`path`; `symbol_id` optional for disambiguation.

## Consequences

### Positive

- Agents discover capabilities via MCP tool list and schemas, not `--help` probing
- Same index engine serves MCP (agents) and CLI (humans/CI)
- Clear product story: MCP = IDE brain API; CLI = maintenance and troubleshooting

### Negative

- Two interface layers still require testing (MCP integration tests + existing CLI/dogfood tests)
- Agents without MCP configuration fall back to CLI (document debug path only)
- `query_structural` deferred may frustrate power users until named tools cover their cases

## Alternatives considered

**MCP only** — rejected in ADR 0005; CI and index lifecycle still need CLI. **CLI-first with better help** — insufficient; 38 probes with help available. **Composite `explore_symbol` tool** — deferred to v2 after core tools proven.

## References

- [ADR 0005](0005-cli-and-mcp-as-primary-interfaces.md)
- [mcp.md](../specs/mcp.md)
- [cli.md](../specs/cli.md)
- [dogfooding/README.md](../dogfooding/README.md)
