use std::{
    fmt::{self, Display},
    io,
    path::{Path, PathBuf},
};

use crate::{episode::EpisodeError, media::MediaError, media_list::MediaListError};

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
    #[error("{path}: I/O error: {kind}")]
    FSIO { path: PathBuf, kind: io::ErrorKind },
    #[error("Failed to found user's data directory")]
    UserDataDirNotFound,
    #[error("{path}: Falied to open default program: {kind}")]
    Open { path: PathBuf, kind: io::ErrorKind },
    #[error("Failed to find next chapter path")]
    FailedToFindNextChapterPath,
    #[error("Filed to load font")]
    FontLoad,
    #[error("Episodes didn't found")]
    EpisodesDidNotFound,
    #[error(transparent)]
    Media(#[from] MediaError),
    #[error(transparent)]
    MediaList(#[from] MediaListError),
    #[error(transparent)]
    Episode(#[from] EpisodeError),
    #[error(transparent)]
    FSIOError(#[from] FSIOError),
}

impl ErrorKind {
    pub fn fsio(path: impl AsRef<Path>, source: io::Error) -> Self {
        let path = path.as_ref().to_path_buf();
        let kind = source.kind();
        Self::FSIO { path, kind }
    }

    pub fn open(path: impl AsRef<Path>, kind: io::ErrorKind) -> Self {
        let path = path.as_ref().to_path_buf();
        Self::Open { path, kind }
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub struct FSIOError {
    path: PathBuf,
    kind: io::ErrorKind,
}

impl FSIOError {
    pub fn new(path: impl AsRef<Path>, source: io::Error) -> Self {
        let path = path.as_ref().to_path_buf();
        let kind = source.kind();
        Self { path, kind }
    }
}

impl Display for FSIOError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: I/O error: {}", self.path.display(), self.kind)
    }
}
