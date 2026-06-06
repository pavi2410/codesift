# 0001. Record Architecture Decisions

**Status:** accepted

**Date:** 2026-06-06

## Context

codesift is a multi-component system (parsing, indexing, storage, query, CLI, MCP) with significant architectural choices. Without documented decisions, contributors will re-debate the same trade-offs and implementations may diverge from intent.

## Decision

We will record significant architectural decisions as Architecture Decision Records (ADRs) in `docs/adr/`.

- Use Michael Nygard's format (Context, Decision, Consequences, Alternatives)
- Number sequentially; supersede rather than edit accepted ADRs
- Link ADRs to affected specs and vice versa
- Mark status: proposed → accepted → deprecated/superseded

## Consequences

### Positive

- Single source of truth for "why we chose X"
- Onboarding is faster for new contributors
- Specs can reference ADRs instead of duplicating rationale

### Negative

- Maintenance overhead to keep ADRs current
- Risk of ADR/spec drift if not updated together

## Alternatives considered

| Alternative | Why not |
|-------------|---------|
| Wiki / discussions only | Hard to version, easy to lose |
| Comments in code only | Not visible during design phase; poor for cross-cutting decisions |
| RFC process | Heavier than needed for early project stage |

## References

- [ADR README](README.md)
- [CONTRIBUTING.md](../../CONTRIBUTING.md)
