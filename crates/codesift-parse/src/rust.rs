use std::fs;

use camino::Utf8Path;
use codesift_core::{Language, SymbolKind, Visibility};
use ra_ap_syntax::ast::{self, AstNode, HasDocComments, HasName, HasVisibility};
use ra_ap_syntax::{Edition, SourceFile, SyntaxNode, TextRange, TextSize};

use crate::PsiProvider;
use crate::draft::{CallDraft, ParseOutput, RefDraft, SymbolDraft};

pub struct RustPsiProvider {
    edition: Edition,
}

impl Default for RustPsiProvider {
    fn default() -> Self {
        Self {
            edition: Edition::Edition2024,
        }
    }
}

impl RustPsiProvider {
    pub fn for_path(path: &Utf8Path, workspace_root: &Utf8Path) -> Self {
        Self {
            edition: detect_edition(path, workspace_root),
        }
    }
}

impl PsiProvider for RustPsiProvider {
    fn language(&self) -> Language {
        Language::Rust
    }

    fn parse_file(&self, source: &str, path: &Utf8Path) -> ParseOutput {
        let parsed = SourceFile::parse(source, self.edition);
        let mut output = ParseOutput::default();

        if !parsed.errors().is_empty() {
            output.errors = parsed.errors().iter().map(|e| e.to_string()).collect();
        }

        let tree = parsed.tree();
        let file_node = tree.syntax().clone();
        let module_name = file_stem(path);
        let module_range = range_of(&file_node);

        output.symbols.push(SymbolDraft {
            kind: SymbolKind::Module,
            name: module_name.clone(),
            qualified_name: Some(module_name.clone()),
            start_byte: module_range.start,
            end_byte: module_range.end,
            start_line: module_range.start_line,
            start_column: module_range.start_column,
            end_line: module_range.end_line,
            end_column: module_range.end_column,
            visibility: None,
            signature: None,
            doc_comment: None,
            parent_name: None,
        });

        walk_module(source, &file_node, &module_name, &mut output);
        output
    }
}

fn walk_module(source: &str, node: &SyntaxNode, parent: &str, output: &mut ParseOutput) {
    for child in node.children() {
        if let Some(item) = ast::Item::cast(child.clone()) {
            extract_item(source, &item, parent, output);
        }
    }
}

fn extract_item(source: &str, item: &ast::Item, parent: &str, output: &mut ParseOutput) {
    match item {
        ast::Item::Fn(func) => {
            let name = func
                .name()
                .map(|n| n.text().to_string())
                .unwrap_or_else(|| "<anonymous>".to_string());
            let range = range_of(func.syntax());
            let qualified = format!("{parent}::{name}");
            output.symbols.push(SymbolDraft {
                kind: SymbolKind::Function,
                name: name.clone(),
                qualified_name: Some(qualified.clone()),
                start_byte: range.start,
                end_byte: range.end,
                start_line: range.start_line,
                start_column: range.start_column,
                end_line: range.end_line,
                end_column: range.end_column,
                visibility: visibility_of(func.visibility()),
                signature: first_line(func.syntax().text().to_string()),
                doc_comment: doc_comments_to_text(func.doc_comments()),
                parent_name: Some(parent.to_string()),
            });

            if let Some(body) = func.body() {
                collect_calls(&qualified, body.syntax(), output);
            }
        }
        ast::Item::Struct(strukt) => {
            let name = strukt
                .name()
                .map(|n| n.text().to_string())
                .unwrap_or_default();
            let range = range_of(strukt.syntax());
            let qualified = format!("{parent}::{name}");
            output.symbols.push(SymbolDraft {
                kind: SymbolKind::Type,
                name: name.clone(),
                qualified_name: Some(qualified.clone()),
                start_byte: range.start,
                end_byte: range.end,
                start_line: range.start_line,
                start_column: range.start_column,
                end_line: range.end_line,
                end_column: range.end_column,
                visibility: visibility_of(strukt.visibility()),
                signature: Some(format!("struct {name}")),
                doc_comment: doc_comments_to_text(strukt.doc_comments()),
                parent_name: Some(parent.to_string()),
            });

            if let Some(ast::FieldList::RecordFieldList(field_list)) = strukt.field_list() {
                for field in field_list.fields() {
                    let fname = field
                        .name()
                        .map(|n| n.text().to_string())
                        .unwrap_or_default();
                    let fr = range_of(field.syntax());
                    output.symbols.push(SymbolDraft {
                        kind: SymbolKind::Field,
                        name: format!("{name}.{fname}"),
                        qualified_name: Some(format!("{qualified}.{fname}")),
                        start_byte: fr.start,
                        end_byte: fr.end,
                        start_line: fr.start_line,
                        start_column: fr.start_column,
                        end_line: fr.end_line,
                        end_column: fr.end_column,
                        visibility: visibility_of(field.visibility()),
                        signature: None,
                        doc_comment: doc_comments_to_text(field.doc_comments()),
                        parent_name: Some(qualified.clone()),
                    });
                }
            }
        }
        ast::Item::Enum(en) => {
            let name = en.name().map(|n| n.text().to_string()).unwrap_or_default();
            let range = range_of(en.syntax());
            output.symbols.push(SymbolDraft {
                kind: SymbolKind::Type,
                name: name.clone(),
                qualified_name: Some(format!("{parent}::{name}")),
                start_byte: range.start,
                end_byte: range.end,
                start_line: range.start_line,
                start_column: range.start_column,
                end_line: range.end_line,
                end_column: range.end_column,
                visibility: visibility_of(en.visibility()),
                signature: Some(format!("enum {name}")),
                doc_comment: doc_comments_to_text(en.doc_comments()),
                parent_name: Some(parent.to_string()),
            });
        }
        ast::Item::Trait(tr) => {
            let name = tr.name().map(|n| n.text().to_string()).unwrap_or_default();
            let range = range_of(tr.syntax());
            output.symbols.push(SymbolDraft {
                kind: SymbolKind::Trait,
                name: name.clone(),
                qualified_name: Some(format!("{parent}::{name}")),
                start_byte: range.start,
                end_byte: range.end,
                start_line: range.start_line,
                start_column: range.start_column,
                end_line: range.end_line,
                end_column: range.end_column,
                visibility: visibility_of(tr.visibility()),
                signature: Some(format!("trait {name}")),
                doc_comment: doc_comments_to_text(tr.doc_comments()),
                parent_name: Some(parent.to_string()),
            });
        }
        ast::Item::Impl(imp) => {
            let self_ty = imp
                .self_ty()
                .map(|t| t.syntax().text().to_string())
                .unwrap_or_else(|| "Self".to_string());
            let range = range_of(imp.syntax());
            let name = format!("impl {self_ty}");
            let qualified = format!("{parent}::{name}");
            output.symbols.push(SymbolDraft {
                kind: SymbolKind::Impl,
                name: name.clone(),
                qualified_name: Some(qualified.clone()),
                start_byte: range.start,
                end_byte: range.end,
                start_line: range.start_line,
                start_column: range.start_column,
                end_line: range.end_line,
                end_column: range.end_column,
                visibility: None,
                signature: Some(name),
                doc_comment: None,
                parent_name: Some(parent.to_string()),
            });

            for item in imp
                .assoc_item_list()
                .into_iter()
                .flat_map(|l| l.assoc_items())
            {
                if let ast::AssocItem::Fn(method) = item {
                    let mname = method
                        .name()
                        .map(|n| n.text().to_string())
                        .unwrap_or_default();
                    let mr = range_of(method.syntax());
                    let mqualified = format!("{qualified}::{mname}");
                    output.symbols.push(SymbolDraft {
                        kind: SymbolKind::Function,
                        name: mname.clone(),
                        qualified_name: Some(mqualified.clone()),
                        start_byte: mr.start,
                        end_byte: mr.end,
                        start_line: mr.start_line,
                        start_column: mr.start_column,
                        end_line: mr.end_line,
                        end_column: mr.end_column,
                        visibility: visibility_of(method.visibility()),
                        signature: first_line(method.syntax().text().to_string()),
                        doc_comment: doc_comments_to_text(method.doc_comments()),
                        parent_name: Some(qualified.clone()),
                    });
                    if let Some(body) = method.body() {
                        collect_calls(&mqualified, body.syntax(), output);
                    }
                }
            }
        }
        ast::Item::Use(use_item) => {
            let text = use_item.syntax().text().to_string();
            let range = range_of(use_item.syntax());
            output.symbols.push(SymbolDraft {
                kind: SymbolKind::Import,
                name: text.trim().to_string(),
                qualified_name: None,
                start_byte: range.start,
                end_byte: range.end,
                start_line: range.start_line,
                start_column: range.start_column,
                end_line: range.end_line,
                end_column: range.end_column,
                visibility: visibility_of(use_item.visibility()),
                signature: Some(text.trim().to_string()),
                doc_comment: None,
                parent_name: Some(parent.to_string()),
            });
        }
        ast::Item::Const(konst) => push_const_like(
            output,
            parent,
            konst
                .name()
                .map(|n| n.text().to_string())
                .unwrap_or_default(),
            konst.syntax(),
            konst.visibility(),
            doc_comments_to_text(konst.doc_comments()),
        ),
        ast::Item::Static(stat) => push_const_like(
            output,
            parent,
            stat.name()
                .map(|n| n.text().to_string())
                .unwrap_or_default(),
            stat.syntax(),
            stat.visibility(),
            doc_comments_to_text(stat.doc_comments()),
        ),
        ast::Item::MacroRules(macro_rules) => {
            let name = macro_rules
                .name()
                .map(|n| n.text().to_string())
                .unwrap_or_default();
            let range = range_of(macro_rules.syntax());
            output.symbols.push(SymbolDraft {
                kind: SymbolKind::Macro,
                name: name.clone(),
                qualified_name: Some(format!("{parent}::{name}")),
                start_byte: range.start,
                end_byte: range.end,
                start_line: range.start_line,
                start_column: range.start_column,
                end_line: range.end_line,
                end_column: range.end_column,
                visibility: visibility_of(macro_rules.visibility()),
                signature: Some(format!("macro_rules! {name}")),
                doc_comment: doc_comments_to_text(macro_rules.doc_comments()),
                parent_name: Some(parent.to_string()),
            });
        }
        ast::Item::Module(module) => {
            let name = module
                .name()
                .map(|n| n.text().to_string())
                .unwrap_or_else(|| "<anonymous>".to_string());
            let range = range_of(module.syntax());
            let qualified = format!("{parent}::{name}");
            output.symbols.push(SymbolDraft {
                kind: SymbolKind::Module,
                name: name.clone(),
                qualified_name: Some(qualified.clone()),
                start_byte: range.start,
                end_byte: range.end,
                start_line: range.start_line,
                start_column: range.start_column,
                end_line: range.end_line,
                end_column: range.end_column,
                visibility: visibility_of(module.visibility()),
                signature: None,
                doc_comment: doc_comments_to_text(module.doc_comments()),
                parent_name: Some(parent.to_string()),
            });

            if let Some(item_list) = module.item_list() {
                walk_module(source, item_list.syntax(), &qualified, output);
            }
        }
        _ => {}
    }
}

fn push_const_like(
    output: &mut ParseOutput,
    parent: &str,
    name: String,
    syntax: &SyntaxNode,
    visibility: Option<ast::Visibility>,
    doc_comment: Option<String>,
) {
    let range = range_of(syntax);
    output.symbols.push(SymbolDraft {
        kind: SymbolKind::Const,
        name: name.clone(),
        qualified_name: Some(format!("{parent}::{name}")),
        start_byte: range.start,
        end_byte: range.end,
        start_line: range.start_line,
        start_column: range.start_column,
        end_line: range.end_line,
        end_column: range.end_column,
        visibility: visibility_of(visibility),
        signature: first_line(syntax.text().to_string()),
        doc_comment,
        parent_name: Some(parent.to_string()),
    });
}

fn collect_calls(caller: &str, node: &SyntaxNode, output: &mut ParseOutput) {
    for descendant in node.descendants() {
        if let Some(call_expr) = ast::CallExpr::cast(descendant.clone())
            && let Some(callee) = call_expr
                .expr()
                .and_then(|expr| callee_name_from_expr(&expr))
        {
            let range = range_of(call_expr.syntax());
            output.calls.push(CallDraft {
                caller_name: caller.to_string(),
                callee_name: callee,
                start_byte: range.start,
                end_byte: range.end,
                start_line: range.start_line,
                start_column: range.start_column,
            });
        }

        if let Some(path_type) = ast::PathType::cast(descendant)
            && let Some(path) = path_type.path()
            && let Some(to_name) = path_to_string(&path)
        {
            let range = range_of(path_type.syntax());
            output.refs.push(RefDraft {
                from_name: caller.to_string(),
                to_name,
                start_byte: range.start,
                end_byte: range.end,
                start_line: range.start_line,
                start_column: range.start_column,
            });
        }
    }
}

fn callee_name_from_expr(expr: &ast::Expr) -> Option<String> {
    match expr {
        ast::Expr::PathExpr(path_expr) => path_expr.path().and_then(|p| path_to_string(&p)),
        ast::Expr::MethodCallExpr(method) => {
            let method_name = method.name_ref()?.text().to_string();
            let receiver = method
                .receiver()
                .and_then(|r| callee_name_from_expr(&r))
                .unwrap_or_else(|| "Self".to_string());
            Some(format!("{receiver}.{method_name}"))
        }
        _ => None,
    }
}

fn path_to_string(path: &ast::Path) -> Option<String> {
    let segments: Vec<String> = path
        .segments()
        .map(|segment| segment.syntax().text().to_string().trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    if segments.is_empty() {
        None
    } else {
        Some(segments.join("::"))
    }
}

struct ByteRange {
    start: u32,
    end: u32,
    start_line: u32,
    start_column: u32,
    end_line: u32,
    end_column: u32,
}

fn range_of(node: &SyntaxNode) -> ByteRange {
    let range: TextRange = node.text_range();
    let start: u32 = range.start().into();
    let end: u32 = range.end().into();
    let (start_line, start_column) = offset_to_line_col(node, range.start());
    let (end_line, end_column) = offset_to_line_col(node, range.end());
    ByteRange {
        start,
        end,
        start_line,
        start_column,
        end_line,
        end_column,
    }
}

fn offset_to_line_col(node: &SyntaxNode, offset: TextSize) -> (u32, u32) {
    let root = node.ancestors().last().unwrap_or_else(|| node.clone());
    let text = root.text().to_string();
    let offset: usize = offset.into();
    let mut line = 0u32;
    let mut col = 0u32;
    for (i, ch) in text.char_indices() {
        if i >= offset {
            break;
        }
        if ch == '\n' {
            line += 1;
            col = 0;
        } else {
            col += 1;
        }
    }
    (line, col)
}

fn visibility_of(vis: Option<ast::Visibility>) -> Option<Visibility> {
    let Some(v) = vis else {
        return Some(Visibility::Private);
    };
    let text = v.syntax().text().to_string();
    if text.contains("pub(crate)") {
        Some(Visibility::PubCrate)
    } else if v.pub_token().is_some() {
        Some(Visibility::Pub)
    } else {
        Some(Visibility::Private)
    }
}

fn doc_comments_to_text(comments: impl Iterator<Item = ast::Comment>) -> Option<String> {
    let docs: Vec<String> = comments
        .filter_map(|comment| comment.doc_comment().map(|doc| doc.0.to_string()))
        .collect();
    if docs.is_empty() {
        None
    } else {
        Some(docs.join("\n"))
    }
}

fn first_line(text: String) -> Option<String> {
    text.lines().next().map(|line| line.trim().to_string())
}

fn file_stem(path: &Utf8Path) -> String {
    path.file_stem()
        .map(|s| s.to_string())
        .unwrap_or_else(|| "root".to_string())
}

fn detect_edition(path: &Utf8Path, workspace_root: &Utf8Path) -> Edition {
    let mut dir = path.parent().map(|p| p.to_path_buf());
    while let Some(current) = dir {
        let manifest = current.join("Cargo.toml");
        if manifest.exists() {
            if let Ok(text) = fs::read_to_string(&manifest) {
                if text.contains("edition = \"2024\"") {
                    return Edition::Edition2024;
                }
                if text.contains("edition = \"2021\"") {
                    return Edition::Edition2021;
                }
            }
            break;
        }
        if current == workspace_root {
            break;
        }
        dir = current.parent().map(|p| p.to_path_buf());
    }
    Edition::Edition2024
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn extracts_functions_and_calls() {
        let source = r#"
pub fn greet() {
    hello();
}

fn hello() {}
"#;
        let provider = RustPsiProvider::default();
        let dir = tempdir().unwrap();
        let path = camino::Utf8PathBuf::from_path_buf(dir.path().join("lib.rs")).unwrap();
        let output = provider.parse_file(source, &path);

        let functions: Vec<_> = output
            .symbols
            .iter()
            .filter(|s| s.kind == SymbolKind::Function)
            .map(|s| s.name.as_str())
            .collect();
        assert!(functions.contains(&"greet"));
        assert!(functions.contains(&"hello"));
        assert!(output.calls.iter().any(|c| c.callee_name == "hello"));
    }

    #[test]
    fn extracts_qualified_call_paths() {
        let source = r#"
fn build() {
    Default::default();
}
"#;
        let provider = RustPsiProvider::default();
        let dir = tempdir().unwrap();
        let path = camino::Utf8PathBuf::from_path_buf(dir.path().join("lib.rs")).unwrap();
        let output = provider.parse_file(source, &path);
        assert!(output.calls.iter().any(|c| c.callee_name == "Default::default"));
    }
}
