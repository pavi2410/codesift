use std::path::Path;

use camino::Utf8PathBuf;

use crate::{Error, Result};

/// Workspace root and index location.
#[derive(Debug, Clone)]
pub struct Workspace {
    pub root: Utf8PathBuf,
    pub index_dir: Utf8PathBuf,
}

impl Workspace {
    pub fn discover(start: impl AsRef<Path>) -> Result<Self> {
        let start = start.as_ref();
        let root = if start.is_file() {
            start
                .parent()
                .ok_or_else(|| Error::message("cannot determine workspace from file path"))?
        } else {
            start
        };

        let root = root.canonicalize().map_err(|source| Error::Io {
            path: root.display().to_string(),
            source,
        })?;

        let root = Utf8PathBuf::from_path_buf(root)
            .map_err(|_| Error::message("workspace path is not valid UTF-8"))?;

        Ok(Self {
            index_dir: root.join(".codesift"),
            root,
        })
    }

    pub fn with_index_dir(mut self, index_dir: impl Into<Utf8PathBuf>) -> Self {
        self.index_dir = index_dir.into();
        self
    }

    pub fn index_path(&self, relative: &str) -> Utf8PathBuf {
        self.index_dir.join(relative)
    }

    pub fn relative_path(&self, absolute: &Path) -> Result<String> {
        let absolute = absolute.canonicalize().map_err(|source| Error::Io {
            path: absolute.display().to_string(),
            source,
        })?;

        let absolute = Utf8PathBuf::from_path_buf(absolute)
            .map_err(|_| Error::message("path is not valid UTF-8"))?;

        absolute
            .strip_prefix(&self.root)
            .map(|p| p.as_str().replace('\\', "/"))
            .map_err(|_| Error::message(format!("path {absolute} is outside workspace")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn workspace_has_codesift_index_dir() {
        let ws = Workspace::discover(".").unwrap();
        assert!(ws.index_dir.ends_with(".codesift"));
    }
}
