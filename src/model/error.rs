/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::io;
use std::path::PathBuf;
use std::sync::Arc;

use cosmic::dialog::file_chooser;
use derive_more::Display;

use crate::model::config::UserDataDirNotFoundError;
use crate::utils::OpenError;

#[derive(Display)]
#[display("{}", self.kind)]
pub struct Error {
    pub kind: ErrorKind,
    pub fatal: bool,
}

impl Error {
    pub const fn fatal(kind: ErrorKind) -> Self {
        Self { kind, fatal: true }
    }

    pub const fn common(kind: ErrorKind) -> Self {
        Self { kind, fatal: false }
    }
}

impl From<ErrorKind> for Error {
    fn from(value: ErrorKind) -> Self {
        Self::common(value)
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum ErrorKind {
    #[error("{name}: Serialize error: {source}")]
    Serialize {
        name: String,
        source: Arc<serde_json::Error>,
    },
    #[error("{path}: file parsing error: {source}")]
    Deserialize {
        path: PathBuf,
        source: Arc<serde_json::Error>,
    },
    #[error("{path}: Failed to find next chapter path")]
    FindNextChapterPath { path: PathBuf },
    #[error("Name \"{name}\" is used")]
    MediaNameIsUsed { name: String },
    #[error("Eisode not found")]
    EpisodeNotFound,
    #[error("Failed to determinate data directory: {path}")]
    DataDir { path: PathBuf },
    #[error(transparent)]
    Open(#[from] OpenError),
    #[error(transparent)]
    Io(#[from] Arc<io::Error>),
    #[error(transparent)]
    UserDataDirNotFound(#[from] UserDataDirNotFoundError),
    #[error("Open dialog error: {source}")]
    OpenDialog { source: Arc<file_chooser::Error> },
    #[error("{path}: Falied to find parent directory")]
    FindParent { path: PathBuf },
    #[error("Failed to expand tilde {0}")]
    PathTildeExpand(Arc<expand_tilde::Error>),
}

impl ErrorKind {
    pub fn serialize(source: serde_json::Error, name: impl Into<String>) -> Self {
        let name = name.into();
        let source = source.into();
        Self::Serialize { name, source }
    }

    pub fn deserialize(path: impl Into<PathBuf>, source: serde_json::Error) -> Self {
        let path = path.into();
        let source = source.into();
        Self::Deserialize { path, source }
    }

    pub fn media_name_is_used(name: impl Into<String>) -> Self {
        Self::MediaNameIsUsed { name: name.into() }
    }

    pub fn data_dir(path: impl Into<PathBuf>) -> Self {
        Self::DataDir { path: path.into() }
    }

    pub fn open_dialog(source: impl Into<Arc<file_chooser::Error>>) -> Self {
        let source = source.into();
        Self::OpenDialog { source }
    }

    pub fn find_next_chapter(path: impl Into<PathBuf>) -> Self {
        Self::FindNextChapterPath { path: path.into() }
    }

    pub fn find_parent(path: impl Into<PathBuf>) -> Self {
        Self::FindParent { path: path.into() }
    }
}

impl From<io::Error> for ErrorKind {
    fn from(value: io::Error) -> Self {
        Self::Io(Arc::new(value))
    }
}

impl From<expand_tilde::Error> for ErrorKind {
    fn from(value: expand_tilde::Error) -> Self {
        Self::PathTildeExpand(value.into())
    }
}

pub type Result<T> = std::result::Result<T, ErrorKind>;
