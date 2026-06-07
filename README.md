# codesift

**IntelliJ-grade code intelligence for agents** — persistent structural + semantic indexing, exposed via CLI and MCP.

[![Status](https://img.shields.io/badge/status-design%20phase-blue)](docs/capabilities.md)
[![License](https://img.shields.io/badge/license-Apache--2.0-green)](LICENSE)

Rust-based indexing engine for AI agents and CI. Design phase: specs and ADRs before implementation.

## Quick links

- [Vision](docs/product/vision.md) — problem, solution, principles
- [Documentation hub](docs/README.md) — reading paths by role
- [Capabilities](docs/capabilities.md) — feature coverage matrix
- [Specs](docs/specs/README.md) — implementation contracts
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
