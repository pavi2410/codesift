use std::fs;
use std::path::Path;

use camino::Utf8Path;
use ignore::WalkBuilder;

use crate::types::Language;
use crate::{Error, Result, Workspace};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileEntry {
    pub path: String,
    pub absolute: camino::Utf8PathBuf,
    pub content_hash: String,
    pub language: Language,
}

pub fn hash_bytes(bytes: &[u8]) -> String {
    format!("blake3:{}", blake3::hash(bytes).to_hex())
}

pub fn hash_file(path: &Utf8Path) -> Result<String> {
    let bytes = fs::read(path).map_err(|source| Error::Io {
        path: path.to_string(),
        source,
    })?;
    Ok(hash_bytes(&bytes))
}

pub fn discover_files(workspace: &Workspace, rust_only: bool) -> Result<Vec<FileEntry>> {
    let mut builder = WalkBuilder::new(&workspace.root);
    builder
        .hidden(false)
        .git_ignore(true)
        .git_global(true)
        .git_exclude(true)
        .ignore(true)
        .filter_entry(|entry| {
            let name = entry.file_name().to_string_lossy();
            name != ".codesift" && name != ".git"
        });

    if let Some(codesiftignore) = workspace
        .root
        .join(".codesiftignore")
        .to_path_buf()
        .exists()
        .then(|| workspace.root.join(".codesiftignore"))
    {
        let _ = codesiftignore;
        builder.add_custom_ignore_filename(".codesiftignore");
    }

    let mut files = Vec::new();

    for entry in builder.build().flatten() {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }

        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or_default();

        let language = match Language::from_extension(ext) {
            Some(lang) if !rust_only || lang == Language::Rust => lang,
            _ => continue,
        };

        if contains_nul_byte(path)? {
            continue;
        }

        let absolute = path.canonicalize().map_err(|source| Error::Io {
            path: path.display().to_string(),
            source,
        })?;
        let absolute = camino::Utf8PathBuf::from_path_buf(absolute)
            .map_err(|_| Error::message("path is not valid UTF-8"))?;

        let rel = workspace.relative_path(path)?;
        let content_hash = hash_file(&absolute)?;

        files.push(FileEntry {
            path: rel,
            absolute,
            content_hash,
            language,
        });
    }

    files.sort_by(|a, b| a.path.cmp(&b.path));
    Ok(files)
}

fn contains_nul_byte(path: &Path) -> Result<bool> {
    use std::io::Read;

    let mut file = fs::File::open(path).map_err(|source| Error::Io {
        path: path.display().to_string(),
        source,
    })?;
    let mut buf = [0u8; 8192];
    let n = file.read(&mut buf).map_err(|source| Error::Io {
        path: path.display().to_string(),
        source,
    })?;
    Ok(buf[..n].contains(&0))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn discovers_rust_files_with_stable_hashes() {
        let dir = tempdir().unwrap();
        let root = dir.path();
        std::fs::write(root.join("lib.rs"), "pub fn hello() {}").unwrap();
        std::fs::write(root.join("readme.md"), "# hi").unwrap();

        let ws = Workspace::discover(root).unwrap();
        let files = discover_files(&ws, true).unwrap();
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].path, "lib.rs");
        assert!(files[0].content_hash.starts_with("blake3:"));

        let again = discover_files(&ws, true).unwrap();
        assert_eq!(files[0].content_hash, again[0].content_hash);
    }

    #[test]
    fn discovers_rs_files_in_repo() {
        let ws = Workspace::discover(env!("CARGO_MANIFEST_DIR")).unwrap();
        let files = discover_files(&ws, true).unwrap();
        assert!(!files.is_empty());
        assert!(files.iter().all(|f| f.path.ends_with(".rs")));
    }
}
