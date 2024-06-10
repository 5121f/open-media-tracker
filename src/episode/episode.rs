use std::path::{Path, PathBuf};

use crate::utils;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Episode {
    path: PathBuf,
}

impl Episode {
    pub fn new(path: PathBuf) -> Result<Self> {
        if !utils::is_media_file(&path) {
            return Err(Error::MustBeAMediaFile);
        }
        Ok(Self { path })
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn name(&self) -> String {
        self.path
            .file_stem()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string()
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum Error {
    #[error("Episode must be media file")]
    MustBeAMediaFile,
}

type Result<T> = std::result::Result<T, Error>;
