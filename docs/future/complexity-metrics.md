# Complexity Metrics

**Status:** exploring

Cyclomatic and cognitive complexity per function/method — grounded metrics agents can use for impact analysis and quality gates. Part of **Layer 2 — Quality signals** in [big-picture.md](../product/big-picture.md).

## Goals

| Goal | Why |
|------|-----|
| **Grounded numbers** | One metric + evidence beats vague “this looks complex” |
| **Agent gates** | Flag hot-path functions before agents add nested logic |
| **Inspection input** | Threshold-based findings (`complexity/high`) with `sym://` links |
| **High SNR** | Computed from CFG/AST — no LLM opinion layer |

## Non-goals

| Non-goal | Rationale |
|----------|-----------|
| **Big-O / asymptotic class labels** | Low SNR; heuristics mislead agents (“O(n²)” as gospel) |
| **Formal cost proofs** | Out of scope for an indexing engine |
| **Replace Clippy lints** | Complement; unify via findings schema |

Instead of Big-O, ship **structural hints** as separate findings when useful (e.g. nested loops, unbounded recursion) — see [Planned structural hints](#planned-structural-hints).

## Metrics

### Cyclomatic complexity (McCabe)

**Definition:** Number of linearly independent paths through a function’s control-flow graph.

**Counting rules (baseline):**

- Start at **1** for the function entry
- **+1** per decision point: `if`, `else if`, `match` arm (non-trivial), `while`, `for`, `loop`, `&&` / `||` in conditions (language-specific rules per backend)
- **+1** per `catch` / `?` early-exit that branches control (Rust: `?` counts as +1 when it exits the function)
- **Do not** count macro-expanded code unless expanded in PSI

**Scope:** Per **function**, **method**, **closure** (closures optional later). Not whole-file unless aggregating for module summary.

### Cognitive complexity (Sonar-style)

**Definition:** How hard a human (or agent) must hold context to understand the function — penalizes **nesting**, not flat branches.

**Counting rules (baseline):**

- **+1** per break in linear flow (`if`, `for`, `match`, `&&`, `||`, `?`, etc.)
- **+N** for nesting: increment nested constructs inside `+1`, `+2`, `+3`… (nesting depth)
- **+1** for `else` / `elif` attached to an `if` (flat penalty)
- Recursion: **+1** when function calls itself (direct)

Cognitive complexity correlates better with “hard to change safely” than cyclomatic alone.

## Data source

```
PSI / AST  →  CFG builder (per language)  →  metric pass  →  index record + findings
```

| Prerequisite | Capability |
|--------------|------------|
| Function boundaries from symbol index | `structural-index-rust` |
| Intra-procedural CFG (Rust first) | `cfg-analysis` |
| Multi-language CFG (tree-sitter) | `multi-language` |

Rust MVP path: build CFG from `ra_ap_syntax` or HIR-lite over PSI; tree-sitter languages use simplified CFG (may under-count macros).

## Index records

Stored in fjall keyspace `complexity` (proposed):

```json
{
  "symbol_id": "sym://42/src/handler.rs#function:handle_request@100:800",
  "cyclomatic": 12,
  "cognitive": 18,
  "lines_of_code": 85,
  "cfg_nodes": 24,
  "computed_at": "2026-06-06T12:00:00Z",
  "language": "rust",
  "confidence": "computed"
}
```

`confidence` is always `computed` for these metrics (not heuristic). Stale when `symbol_id` tombstoned on re-index.

## Query and MCP surfaces

### Structural query (proposed)

```
complexity:cyclomatic>10 path:src/api/**
complexity:cognitive>15 kind:function lang:rust
complexity:top limit:20
```

### MCP tool: `get_complexity` (proposed)

**Input:**

```json
{
  "symbol_id": "sym://42/src/handler.rs#function:handle_request@100:800"
}
```

**Output:**

```json
{
  "symbol_id": "sym://42/src/handler.rs#function:handle_request@100:800",
  "name": "handle_request",
  "path": "src/handler.rs",
  "cyclomatic": 12,
  "cognitive": 18,
  "lines_of_code": 85,
  "thresholds": {
    "cyclomatic_warning": 10,
    "cognitive_warning": 15
  },
  "over_threshold": true
}
```

### Inspection finding (proposed)

When `cyclomatic > threshold` or `cognitive > threshold`:

```json
{
  "rule_id": "complexity/cognitive_high",
  "severity": "warning",
  "message": "Cognitive complexity 18 exceeds threshold 15",
  "symbol_id": "sym://42/src/handler.rs#function:handle_request@100:800",
  "category": "complexity",
  "evidence": { "cognitive": 18, "cyclomatic": 12, "threshold": 15 }
}
```

Default thresholds: configurable per workspace in `.codesift/config.toml` (TBD).

## Planned structural hints

Separate from cyclomatic/cognitive — pattern findings without asymptotic labels:

| Hint | Trigger |
|------|---------|
| `complexity/nested_loops` | Loop nested ≥2 deep over collections |
| `complexity/deep_nesting` | Block nesting depth ≥4 |
| `complexity/recursion_unbounded` | Direct recursion without obvious base (heuristic) |

These use `severity: info` unless combined with high cognitive score.

## Agent workflow

```
1. get_callers(symbol)           → is this on a hot path?
2. get_complexity(symbol)          → safe to extend?
3. run_inspections(scope, rules) → complexity/* findings
4. search_semantic("similar simple handler") → idiomatic alternative
```

## Coverage

| Capability | Deliverable |
|------------|-------------|
| `cfg-analysis` | CFG builder for Rust functions |
| `complexity-metrics` | `complexity` keyspace, `get_complexity`, query filters |
| `inspections-unified` | Threshold inspections in `run_inspections` |
| `multi-language` | tree-sitter CFG backends (reduced fidelity) |

Promote to [capabilities.md](../capabilities.md) when `spec-accepted`.

## See also

- [big-picture.md](../product/big-picture.md) — Layer 2 quality signals
- [specs/symbol-model.md](../specs/symbol-model.md) — symbol IDs for evidence links
- [specs/index-schema.md](../specs/index-schema.md) — storage patterns
