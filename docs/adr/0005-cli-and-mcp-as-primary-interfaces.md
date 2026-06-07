# 0005. CLI + MCP as Primary Interfaces

**Status:** accepted

**Date:** 2026-06-06

## Context

codesift serves developers (CLI, CI), AI agents (MCP), and future IDE plugins. We need to choose primary interfaces for MVP and defer heavier integrations.

## Decision

- **CLI:** `clap` 4.x — commands per [cli.md](../specs/cli.md)
- **MCP:** `rmcp` 1.7.x — official SDK, stdio transport, tools per [mcp.md](../specs/mcp.md)
- **LSP:** deferred to `lsp-adapter` capability
- **HTTP REST API:** deferred; streamable HTTP on rmcp optional later

## Consequences

### Positive

- CLI is universal for CI and scripting
- MCP is standard for AI agents (Cursor, Claude, etc.)
- rmcp has largest adoption (11M+ downloads) and official status
- stdio MCP is simple to configure in agent settings

### Negative

- No LSP until `lsp-adapter` — IDE users use CLI or MCP meanwhile
- MCP stdio is one workspace per process
- Two interface layers to test and maintain

## Alternatives considered

| Alternative | Why not |
|-------------|---------|
| MCP only | Poor CI/scripting ergonomics |
| REST API first | Agents prefer MCP; extra server complexity |
| mcpkit | Pre-1.0; less adoption |
| LSP first | Heavier protocol; agents need MCP sooner |

## References

- [cli.md](../specs/cli.md)
- [mcp.md](../specs/mcp.md)
- [use-cases.md](../product/use-cases.md)
