use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct FilePath {
    base_directory: PathBuf,
    relative_path: PathBuf,
}

impl FilePath {
    pub fn new(base_directory: &Path, relative_path: &Path) -> Self {
        FilePath {
            base_directory: base_directory.to_path_buf(),
            relative_path: relative_path.to_path_buf(),
        }
    }

    pub fn real_path(&self) -> String {
        self.base_directory
            .join(&self.relative_path)
            .to_str()
            .expect("Invalid UTF-8 in path")
            .to_string()
    }

    pub fn relative_path(&self) -> String {
        self.relative_path.to_str().unwrap().to_string()
    }
}
