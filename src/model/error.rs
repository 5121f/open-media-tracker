/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::{
    fmt::{self, Display},
    io,
    path::PathBuf,
    sync::Arc,
};

use ron::de::SpannedError;

use crate::open::OpenError;

pub struct Error {
    pub kind: ErrorKind,
    pub critical: bool,
}

impl Error {
    pub const fn critical(kind: ErrorKind) -> Self {
        let critical = true;
        Self { kind, critical }
    }

    pub const fn common(kind: ErrorKind) -> Self {
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
        Self::common(value)
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
    #[error("{path}: Failed to find parent dir")]
    FindParentDir { path: PathBuf },
    #[error(transparent)]
    Open(#[from] OpenError),
    #[error(transparent)]
    Io(#[from] Arc<io::Error>),
}

impl ErrorKind {
    pub const fn serialize(name: String, source: ron::Error) -> Self {
        Self::Serialize { name, source }
    }

    pub fn deserialize(path: impl Into<PathBuf>, source: SpannedError) -> Self {
        let path = path.into();
        Self::Deserialize { path, source }
    }

    pub fn media_name_is_used(name: impl Into<String>) -> Self {
        Self::MediaNameIsUsed { name: name.into() }
    }

    pub fn find_parent_dir(path: impl Into<PathBuf>) -> Self {
        Self::FindParentDir { path: path.into() }
    }
}

impl From<io::Error> for ErrorKind {
    fn from(value: io::Error) -> Self {
        Self::Io(Arc::new(value))
    }
}

pub type Result<T> = std::result::Result<T, ErrorKind>;
