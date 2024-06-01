use std::path::{Path, PathBuf};

use crate::{error::ErrorKind, utils};

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Episode {
    path: PathBuf,
}

impl Episode {
    pub fn new(path: PathBuf) -> Result<Self, ErrorKind> {
        if !utils::is_media_file(&path) {
            return Err(ErrorKind::EpisodeMustBeAMediaFile);
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
