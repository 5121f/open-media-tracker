/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::{
    fmt::{self, Display},
    io,
    path::{Path, PathBuf},
};

use ron::de::SpannedError;

use crate::open::OpenError;

pub struct Error {
    pub kind: ErrorKind,
    pub critical: bool,
}

impl Error {
    pub fn critical(kind: ErrorKind) -> Self {
        let critical = true;
        Self { kind, critical }
    }

    pub fn common(kind: ErrorKind) -> Self {
        let critical = false;
        Self { kind, critical }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.kind.fmt(f)
    }
}

impl From<ErrorKind> for Error {
    fn from(value: ErrorKind) -> Self {
        Error::common(value)
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum ErrorKind {
    #[error("Failed to found user's data directory")]
    UserDataDirNotFound,
    #[error("{name}: Serialize error: {source}")]
    Serialize { name: String, source: ron::Error },
    #[error("{path}: file parsing error: {source}")]
    Deserialize { path: PathBuf, source: SpannedError },
    #[error("Failed to find next chapter path")]
    FindNextChapterPath,
    #[error("Name \"{name}\" is used")]
    MediaNameIsUsed { name: String },
    #[error("Eisode not found")]
    EpisodeNotFound,
    #[error(transparent)]
    Open(#[from] OpenError),
    #[error(transparent)]
    Fsio(#[from] FSIOError),
}

impl ErrorKind {
    pub fn serialize(name: String, source: ron::Error) -> Self {
        Self::Serialize { name, source }
    }

    pub fn deserialize(path: impl AsRef<Path>, source: SpannedError) -> Self {
        let path = path.as_ref().to_path_buf();
        Self::Deserialize { path, source }
    }

    pub fn media_name_is_used(name: &str) -> Self {
        Self::MediaNameIsUsed {
            name: name.to_string(),
        }
    }
}

#[derive(Debug, Clone, thiserror::Error)]
#[error("{path}: I/O error: {kind}")]
pub struct FSIOError {
    pub path: PathBuf,
    pub kind: io::ErrorKind,
}

impl FSIOError {
    pub fn new(path: impl Into<PathBuf>, source: io::Error) -> Self {
        let path = path.into();
        let kind = source.kind();
        Self { path, kind }
    }
}

pub type Result<T> = std::result::Result<T, ErrorKind>;
