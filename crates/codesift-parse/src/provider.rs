use camino::Utf8Path;

use crate::ParseOutput;

pub trait PsiProvider {
    fn language(&self) -> codesift_core::Language;
    fn parse_file(&self, source: &str, path: &Utf8Path) -> ParseOutput;
}
