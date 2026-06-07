# Multi-Language Strategy

Phased language support and per-language capability matrix.

## Phases

| Phase | Languages | Parser |
|-------|-----------|--------|
| 1 (MVP) | Rust | `ra_ap_syntax` |
| 4 | TypeScript, Python, Go, Java | tree-sitter grammars |
| 5+ | C/C++, Ruby, Kotlin, ... | tree-sitter as community demand |

Markdown and plain docs indexed with `semantic-search`.

## Capability matrix (target)

| Capability | Rust MVP | tree-sitter langs |
|------------|----------|-------------------|
| Symbol definitions | yes | yes |
| Syntax-level refs | partial | partial |
| Call graph | yes | best-effort |
| Import graph | yes | yes |
| Type resolve | no | no |
| Macro expansion | no | no |
| Incremental reparse | yes | yes |
| Semantic chunks | `semantic-search` | `multi-language` |

## Per-language PSI providers

```
codesift-parse/
├── rust/          # ra_ap_syntax backend
├── typescript/    # tree-sitter-typescript + queries
├── python/
└── ...
```

Each implements `PsiProvider` with language-specific query `.scm` files.

## tree-sitter grammar management

- Pin grammar crate versions in workspace `Cargo.toml`
- Vendor or submodule `.scm` queries in repo
- CI: parse smoke tests on sample files per language

## Language detection

1. File extension map (`.rs` → rust)
2. Shebang / modeline for scripts
3. `lang` override in `.codesift/config.toml` (future)

## See also

- [parsing-and-psi.md](parsing-and-psi.md)
- [../adr/0003-tree-sitter-for-parsing.md](../adr/0003-tree-sitter-for-parsing.md)
- [../capabilities.md](../capabilities.md)
