# Dogfooding Results

Changelog of benchmark runs. Orchestrator fills rows after scoring offline.

| Date | Exercise | Commit | Tool calls | rg runs | Completeness | Latency | Notes |
|------|----------|--------|------------|---------|--------------|---------|-------|
| 2026-06-07 | DF-001 (CLI) | pre-fix | ~45–50 CLI | 1 | partial | ~8–12 min | parent session, not isolated subagent |
| | DF-001-MCP | post-mcp | target ≤3 MCP | — | TBD | TBD | integration tests pass; subagent baseline pending |
