# IntelliJ and IDE Indexing

Reference links for IntelliJ Platform indexing concepts that inspire codesift.

## Official documentation

| Topic | URL |
|-------|-----|
| IntelliJ Platform SDK | https://plugins.jetbrains.com/docs/intellij/welcome.html |
| Indexing and PSI | https://plugins.jetbrains.com/docs/intellij/indexing-and-psi-stubs.html |
| PSI elements | https://plugins.jetbrains.com/docs/intellij/psi-elements.html |
| Stub indexes | https://plugins.jetbrains.com/docs/intellij/stub-indexes.html |
| File-based indexes | https://plugins.jetbrains.com/docs/intellij/file-based-indexes.html |

## Key concepts

### PSI (Program Structure Interface)

Tree of elements representing source structure. codesift's PSI layer is documented in [parsing-and-psi.md](../techniques/parsing-and-psi.md).

### Stub indexes

Compact persistent data structures keyed by names/flags, independent of PSI tree lifetime. Maps to codesift fjall keyspaces — see [intellij-inspiration.md](../techniques/intellij-inspiration.md).

### ReferencesSearch

API to find all references to a PSI element. Maps to `refs:` and `callers:` queries.

## Other IDE references

| IDE / tool | Relevant idea |
|------------|---------------|
| VS Code + LSP | Separation of editor and language server |
| rust-analyzer | Incremental syntax + salsa (in-memory) |
| Eclipse JDT | Early stub index designs |

## See also

- [intellij-inspiration.md](../techniques/intellij-inspiration.md)
- [glossary.md](../glossary.md)
