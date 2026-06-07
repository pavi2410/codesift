# Parsing and PSI

**Status:** draft

How codesift parses source into a PSI-like layer. See ADR-0003.

## Parser strategy

| Language phase | Parser | Rationale |
|----------------|--------|-----------|
| Rust (MVP) | `ra_ap_syntax` | Lossless CST, incremental reparse, edition-aware |
| Multi-lang (`multi-language`) | tree-sitter | Broad grammar coverage, incremental |
| Markdown / docs | tree-sitter-markdown or custom | Section extraction |

**Not used:** `syn` — not incremental; unsuitable for watch-mode indexing.

## tree-sitter baseline

- **Incremental reparse:** `tree.edit()` + `parser.parse(source, Some(&old_tree))`
- **Changed ranges:** `tree.get_changed_ranges(&old_tree)` drives symbol diff
- **Queries:** `.scm` query files extract nodes (functions, classes, imports)
- **Error tolerance:** partial trees on syntax errors; index what's parseable

### tree-sitter query example (Rust-like, illustrative)

```scheme
(function_item
  name: (identifier) @name
  body: (block) @body) @function
```

## ra_ap_syntax for Rust

```rust
// Pseudocode — TBD at implementation
let parse = SourceFile::parse(source, edition);
let tree = parse.tree();
// Walk syntax nodes → extract SymbolDraft records
```

Benefits over tree-sitter-rust alone:

- Accurate Rust syntax including macros and editions
- Same CST model as rust-analyzer
- Incremental `reparse()` on edit

## PSI trait (sketch)

```rust
/// Language-agnostic symbol extraction from a parsed file.
trait PsiProvider {
    fn language(&self) -> Language;
    fn parse(&self, source: &str, path: &Path) -> ParseResult;
    fn extract_symbols(&self, parse: &ParseResult) -> Vec<SymbolDraft>;
    fn extract_refs(&self, parse: &ParseResult) -> Vec<RefDraft>;
    fn extract_calls(&self, parse: &ParseResult) -> Vec<CallDraft>;
}
```

`SymbolDraft` / `RefDraft` are converted to persisted records by the structural indexer.

## Incremental reparse flow

1. File change detected (content hash differs)
2. Load previous parse from cache (optional) or reparse full file
3. Incremental reparse with edit range
4. Diff symbol sets: added, removed, modified (span/name/kind)
5. Emit tombstones + new records

## Error handling

| Situation | Behavior |
|-----------|----------|
| Total parse failure | Record error in `file_state`; skip symbol extraction |
| Partial parse | Index valid subtrees |
| Macro expansion | Syntax-level only in MVP; no expansion |

## See also

- [../adr/0003-tree-sitter-for-parsing.md](../adr/0003-tree-sitter-for-parsing.md)
- [multi-language-strategy.md](multi-language-strategy.md)
- [incremental-invalidation.md](incremental-invalidation.md)
