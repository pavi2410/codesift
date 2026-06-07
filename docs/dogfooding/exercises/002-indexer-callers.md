# DF-002: indexer callers

**Automated test:** `crates/codesift-index/tests/dogfood_exercises.rs::df002_helper_callers`

## Agent setup

Fresh subagent; prompt only. Forbidden: grep/rg, ground-truth docs.

## Prerequisite

Index a workspace containing a function `index()` and a caller `drive()` that calls it (see integration test fixture), or the codesift repo after `mise run cli index .`.

## Prompt

Using codesift CLI only (no grep), find all direct callers of the `index` function.

## Ground truth

| Item | Expected |
|------|----------|
| Definition | `index` function |
| Caller | `drive` calls `index` |

## Target codesift sequence

```bash
mise run cli query "symbol:name=index kind=function"
mise run cli query "callers:of={id} depth=1"
```

## Ripgrep baseline

```bash
rg -n "\bindex\s*\(" --glob "*.rs"
```

Text matches only; cannot distinguish calls from definitions without manual filtering.

## Baseline runs

| Date | Commit | Runner | Notes |
|------|--------|--------|-------|
| | | | |
