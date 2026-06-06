# codesift

**IntelliJ-grade code intelligence for agents** — the brain and engine (not the GUI) for exploring, analyzing, and writing idiomatic, safe, correct, secure, performant, efficient, and scalable code at any scale.

[![Status](https://img.shields.io/badge/status-design%20phase-blue)](docs/roadmap.md)
[![License](https://img.shields.io/badge/license-Apache--2.0-green)](LICENSE)

codesift indexes and analyzes source code and documentation into a persistent, incremental intelligence layer — structural queries, semantic search, inspections, and quality gates. Inspired by JetBrains' deep IDE intelligence (PSI, indexes, analysis), implemented in Rust, and exposed via **CLI** and **MCP** so AI agents and CI can sift any codebase without vibe-coded slop.

## Status

This repository is in the **design phase**. Specifications, architecture documents, and ADRs are being written before implementation begins. See [docs/roadmap.md](docs/roadmap.md) for the phased plan.

## Who is this for?

| Consumer | Example use |
|----------|-------------|
| AI coding agents | Find callers, semantically similar code, read chunks via MCP |
| IDE / LSP plugins | Go-to-definition, find references, symbol hierarchy |
| Code review / CI | Impact analysis, blast radius of changed symbols |
| SAST / linting | Pluggable rules over a shared PSI/index substrate |
| General development | Cross-repo symbol search, unified code + doc search |

## Quick links

- [Vision](docs/vision.md) — problem, solution, success criteria
- [Big picture](docs/big-picture.md) — long-term code intelligence for agents
- [Roadmap](docs/roadmap.md) — phased delivery plan
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
