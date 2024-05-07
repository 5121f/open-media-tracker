use std::{
    fmt::{self, Display},
    io,
    path::{Path, PathBuf},
};

use ron::de::SpannedError;

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
    #[error("{path}: file parsing error: {source}")]
    Parce { path: PathBuf, source: SpannedError },
    #[error("{name}: Serialize error: {source}")]
    SerializeSeries { name: String, source: ron::Error },
    #[error("Failed to found user's data directory")]
    UserDataDirNotFound,
    #[error("{path}: Falied to open video in default program: {kind}")]
    OpenVideo { path: PathBuf, kind: io::ErrorKind },
    #[error("{path}: Failed to find parent directory")]
    FaliedToGetParentDir { path: PathBuf },
    #[error("Failed to find next season path")]
    FailedToFindNextSeasonPath,
    #[error("Filed to load font")]
    FontLoad,
    #[error("{season_path}: Season path did not exists")]
    SeasonPathDidNotExists { season_path: PathBuf },
    #[error("Episodes didn't found")]
    EpisodesDidNotFound,
}

impl ErrorKind {
    pub fn fsio(path: impl AsRef<Path>, source: io::Error) -> Self {
        let path = path.as_ref().to_path_buf();
        let kind = source.kind();
        Self::FSIO { path, kind }
    }

    pub fn parse(path: impl AsRef<Path>, source: SpannedError) -> Self {
        let path = path.as_ref().to_path_buf();
        Self::Parce { path, source }
    }

    pub fn serialize_series(name: String, source: ron::Error) -> Self {
        Self::SerializeSeries { name, source }
    }

    pub fn open_vido(path: impl AsRef<Path>, kind: io::ErrorKind) -> Self {
        let path = path.as_ref().to_path_buf();
        Self::OpenVideo { path, kind }
    }

    pub fn parent_dir(path: impl AsRef<Path>) -> Self {
        let path = path.as_ref().to_path_buf();
        Self::FaliedToGetParentDir { path }
    }
}
