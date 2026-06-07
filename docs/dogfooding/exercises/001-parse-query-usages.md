# DF-001: parse_query callstack and usages

**Automated test:** `crates/codesift-index/tests/dogfood_exercises.rs::df001_parse_query_refs_and_callers`

## Agent setup

Launch a **new** subagent (`generalPurpose` or `shell`). Pass **Prompt** only + constraint "codesift CLI only; no grep/rg/read of ground-truth docs". Do not resume the session that dogfooded codesift or wrote this exercise. Orchestrator scores after subagent finishes.

**Forbidden:** grep, ripgrep, reading this file or other dogfooding ground-truth docs during the exercise.

**Allowed:** `mise run cli …`, `codesift` binary, index commands.

## Prerequisite

```bash
mise run cli index .
```

## Prompt

Using codesift CLI only (no grep), find the callstack and usages of `parse_query`.

## Ground truth

| Item | Expected |
|------|----------|
| Definition | `parser::parse_query`, `crates/codesift-query/src/parser.rs` |
| Usages | `main.rs` (query + refs paths), `parser.rs` (tests), `executor.rs` (tests) |
| Production callstack | `main` → `run` → `parse_query` |

## Target codesift sequence (≤5 commands)

```bash
mise run cli query "symbol:name=parse_query kind=function"
mise run cli refs --name parse_query
mise run cli query "callers:of={id} depth=2"
```

## Ripgrep baseline

```bash
rg -n "parse_query" --glob "*.rs"
```

Returns text matches with line numbers; no callstack; ~1 command.

## Baseline runs

| Date | Commit | Runner | CLI invocations | Wall time | Usages found | Callstack | Call-site granularity |
|------|--------|--------|-----------------|-----------|--------------|-----------|----------------------|
| 2026-06-07 | pre-fix | parent session | ~45–50 | ~8–12 min | 2 (+ workaround) | partial | enclosing symbol only |
| | post-fix | TBD | | | ≥4 | main via depth=2 | per-edge site |

Orchestrator records post-fix row after a clean subagent run.
