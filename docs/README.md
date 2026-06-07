# codesift Documentation

Documentation index and reading paths.

## Quick start

| I want to… | Start here |
|------------|------------|
| Understand what codesift is | [vision.md](vision.md) → [big-picture.md](big-picture.md) |
| See feature coverage | [capabilities.md](capabilities.md) |
| Implement a component | [specs/README.md](specs/README.md) |
| Understand a design decision | [adr/README.md](adr/README.md) |
| Look up a term | [glossary.md](glossary.md) |
| Explore future ideas | [future/README.md](future/README.md) |

## Reading paths

### Newcomer (15 minutes)

1. [../README.md](../README.md) — project pitch
2. [vision.md](vision.md) — problem and solution
3. [big-picture.md](big-picture.md) — long-term intelligence for agents
4. [use-cases.md](use-cases.md) — who uses this
5. [capabilities.md](capabilities.md) — feature coverage matrix

### Contributor (1 hour)

1. Newcomer path
2. [goals-and-non-goals.md](goals-and-non-goals.md)
3. [architecture/overview.md](architecture/overview.md)
4. [architecture/technology-stack.md](architecture/technology-stack.md)
5. [CONTRIBUTING.md](../CONTRIBUTING.md)

### Implementer (half day)

1. Contributor path
2. [specs/symbol-model.md](specs/symbol-model.md)
3. [specs/index-schema.md](specs/index-schema.md)
4. [specs/storage.md](specs/storage.md)
5. [specs/query-language.md](specs/query-language.md)
6. [specs/cli.md](specs/cli.md) and [specs/mcp.md](specs/mcp.md)
7. [architecture/future-crate-layout.md](architecture/future-crate-layout.md)
8. Relevant [adr/](adr/) for your area

### Deep dive (techniques)

1. [techniques/intellij-inspiration.md](techniques/intellij-inspiration.md)
2. [techniques/parsing-and-psi.md](techniques/parsing-and-psi.md)
3. [techniques/structural-indexing.md](techniques/structural-indexing.md)
4. [techniques/semantic-retrieval.md](techniques/semantic-retrieval.md)
5. [techniques/incremental-invalidation.md](techniques/incremental-invalidation.md)

## Document map

```
docs/
├── vision.md, big-picture.md, goals-and-non-goals.md, use-cases.md, glossary.md
├── capabilities.md   # Feature coverage (primary planning doc)
├── roadmap.md        # Pointer to capabilities.md
├── architecture/     # System design
├── specs/            # Implementation contracts
├── techniques/       # Algorithms and rationale
├── adr/              # Architecture decisions
├── references/       # External links and comparisons
├── future/           # Exploring ideas (not accepted specs)
└── milestones/       # Per-capability acceptance criteria
```

## Spec status legend

| Status | Meaning |
|--------|---------|
| draft | Written; may change before implementation |
| accepted | Locked for implementation |
| deprecated | Superseded; do not implement |

## See also

- [GitHub repository](https://github.com/pavi2410/codesift)
