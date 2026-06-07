# codesift

**IntelliJ-grade code intelligence for agents** — the brain and engine (not the GUI) for exploring, analyzing, and writing idiomatic, safe, correct, secure, performant, efficient, and scalable code at any scale.

[![Status](https://img.shields.io/badge/status-design%20phase-blue)](docs/capabilities.md)
[![License](https://img.shields.io/badge/license-Apache--2.0-green)](LICENSE)

codesift indexes source code and documentation into a persistent, incremental intelligence layer — structural queries, semantic search, and (over time) inspections and quality gates. Inspired by JetBrains' deep IDE intelligence (PSI, indexes, analysis), implemented in Rust, and exposed via **CLI** and **MCP** for AI agents and CI.

## Status

**Design phase** — specs and ADRs before implementation. Feature coverage (not timeline) is tracked in [docs/capabilities.md](docs/capabilities.md).

## Who is this for?

| Consumer | Example use |
|----------|-------------|
| AI coding agents (primary) | Find callers, semantically similar code, read chunks via MCP |
| IDE / LSP plugins (future) | Go-to-definition, find references, symbol hierarchy |
| Code review / CI | Impact analysis, blast radius of changed symbols |
| SAST / linting | Pluggable rules over a shared PSI/index substrate |
| General development | Cross-repo symbol search, unified code + doc search |

## Quick links

- [Vision](docs/vision.md) — problem, solution, success criteria
- [Big picture](docs/big-picture.md) — long-term code intelligence for agents
- [Capabilities](docs/capabilities.md) — feature coverage matrix
- [Architecture](docs/architecture/overview.md) — system design
- [Specs](docs/specs/README.md) — implementation contracts
- [Technology stack](docs/architecture/technology-stack.md) — crate choices (June 2026)
- [Contributing](CONTRIBUTING.md)

## Interfaces (planned)

```bash
# CLI
codesift index .
codesift query "callers:of sym://..."
codesift search "retry with exponential backoff" --lang rust

# MCP (stdio transport)
codesift mcp --workspace .
```

## License

Apache-2.0. See [LICENSE](LICENSE).
