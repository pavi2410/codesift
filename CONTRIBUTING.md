# Contributing to codesift

Thank you for your interest in contributing. This project is currently in the **design phase** — contributions to documentation, specs, and ADRs are especially welcome.

## Getting started

1. Read [docs/product/vision.md](docs/product/vision.md) and [docs/capabilities.md](docs/capabilities.md) for context.
2. Browse [docs/README.md](docs/README.md) for curated reading paths.
3. Check open issues and [docs/capabilities.md](docs/capabilities.md) before starting work.

## What to contribute

| Area | How to help |
|------|-------------|
| Documentation | Fix typos, clarify specs, add examples |
| ADRs | Propose architectural decisions via new ADRs |
| Specs | Refine schemas, query grammar, interface contracts |
| Implementation | Available once core capabilities move to `not-started` (see [capabilities](docs/capabilities.md)) |

## Documentation conventions

- **Diagrams**: Use mermaid in markdown. Keep node IDs in camelCase.
- **Examples**: Every spec should include at least one JSON request/response example where applicable.
- **Status headers**: Specs use `Status: draft | accepted | deprecated` at the top.
- **Cross-links**: Specs reference ADRs for decisions; ADRs link back to affected specs.
- **Symbol IDs**: Use the `sym://` prefix consistently (see [docs/specs/symbol-model.md](docs/specs/symbol-model.md)).
- **No premature code**: Use pseudocode and schema blocks in design docs. Mark `TBD at implementation` where details are intentionally deferred.

## Architecture Decision Records (ADRs)

Significant design decisions are recorded in [docs/adr/](docs/adr/). To propose a new ADR:

1. Copy the template from [docs/adr/README.md](docs/adr/README.md).
2. Number sequentially (e.g. `0009-my-decision.md`).
3. Include: Context, Decision, Consequences, Alternatives considered.
4. Open a PR for review.

Do not change accepted ADRs in place — supersede them with a new ADR.

## Pull request process

1. Fork the repository and create a feature branch.
2. Make focused changes — one concern per PR when possible.
3. Ensure markdown links resolve and mermaid diagrams render.
4. Describe what changed and why in the PR body.
5. A maintainer will review and merge.

## Code contributions (future)

Once the Rust workspace lands:

```bash
cargo build
cargo test
cargo fmt --check
cargo clippy -- -D warnings
```

CI will enforce these checks. See [docs/architecture/future-crate-layout.md](docs/architecture/future-crate-layout.md) for the planned crate structure.

## Code of conduct

This project follows the [Contributor Covenant](CODE_OF_CONDUCT.md). By participating, you agree to uphold it.
