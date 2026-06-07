# DF-003: symbol by kind and path

**Automated test:** `crates/codesift-index/tests/dogfood_exercises.rs::df003_path_glob_and_kind_filter`

## Agent setup

Fresh subagent; prompt only. Forbidden: grep/rg, ground-truth docs.

## Prerequisite

```bash
mise run cli index .
```

## Prompt

Using codesift CLI only (no grep), list all functions defined in any `parser.rs` file in the workspace.

## Ground truth

Query returns at least one hit; includes `parse_query` in `crates/codesift-query/src/parser.rs`.

## Target codesift sequence

```bash
mise run cli query "path=**/parser.rs kind=function"
```

## Ripgrep baseline

```bash
rg -n "^pub fn |^fn " --glob "**/parser.rs"
```

Text-only; no symbol IDs or kind filtering.

## Baseline runs

| Date | Commit | Runner | Notes |
|------|--------|--------|-------|
| | | | |
