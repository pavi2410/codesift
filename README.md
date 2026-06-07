# codesift

**IntelliJ-grade code intelligence for agents** — persistent structural + semantic indexing, exposed via CLI and MCP.

[![Status](https://img.shields.io/badge/status-structural%20MVP-green)](docs/capabilities.md)
[![License](https://img.shields.io/badge/license-Apache--2.0-green)](LICENSE)

Rust-based indexing engine for AI agents and CI. Structural index, MCP agent frontend, and CLI ops/debug implemented; semantic search next.

## Development

Uses [mise](https://mise.jdx.dev/) for the Rust toolchain (see [mise.toml](mise.toml)):

```bash
mise trust
mise run fix    # cargo fmt + clippy --fix
mise run check  # CI-equivalent
mise run cli index .
mise run mcp          # MCP stdio server (Cursor / OpenCode)
```

**Agents → MCP** (`find_references`, `get_callers`, …). Config: [`.cursor/mcp.json`](.cursor/mcp.json), [`opencode.json`](opencode.json) — both use `mise run mcp`. **Humans/CI → CLI** (`index`, `status`, `export`).

## Dogfooding

Agent exercises with ground truth and baselines: [docs/dogfooding/](docs/dogfooding/).

## Quick links

- [Vision](docs/product/vision.md) — problem, solution, principles
- [Documentation hub](docs/README.md) — reading paths by role
- [Capabilities](docs/capabilities.md) — feature coverage matrix
- [Specs](docs/specs/README.md) — implementation contracts
- [Contributing](CONTRIBUTING.md)

## Interfaces

```bash
# Agents — MCP (primary)
codesift --workspace . mcp

# Humans / CI — CLI ops and debug
codesift index .
codesift status
codesift refs --name parse_query --format ascii
codesift export --type ref
```

## License

Apache-2.0. See [LICENSE](LICENSE).
