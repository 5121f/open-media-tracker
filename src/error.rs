use std::{
    fmt::Display,
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
        Self {
            kind,
            critical: true,
        }
    }

    pub fn general(kind: ErrorKind) -> Self {
        Self {
            kind,
            critical: false,
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.kind.fmt(f)
    }
}

impl From<ErrorKind> for Error {
    fn from(value: ErrorKind) -> Self {
        Error::general(value)
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum ErrorKind {
    #[error("{path}: I/O error: {kind}")]
    FSIO { path: PathBuf, kind: io::ErrorKind },
    #[error("{path}: file parsing error: {source}")]
    Parce { path: PathBuf, source: SpannedError },
    #[error("{series_name}: Serialize error: {source}")]
    SerializeSeries {
        series_name: String,
        source: ron::Error,
    },
    #[error("Failed to found user's data directory")]
    UserDataDirNotFound,
    #[error("{video_path}: Falied to open video in default program: {kind}")]
    OpenVideo {
        video_path: PathBuf,
        kind: io::ErrorKind,
    },
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
        Self::FSIO {
            path: path.as_ref().to_path_buf(),
            kind: source.kind(),
        }
    }

    pub fn parse(path: impl AsRef<Path>, source: SpannedError) -> Self {
        Self::Parce {
            path: path.as_ref().to_path_buf(),
            source,
        }
    }

    pub fn serialize_series(name: String, source: ron::Error) -> Self {
        Self::SerializeSeries {
            series_name: name,
            source,
        }
    }

    pub fn open_vido(path: impl AsRef<Path>, kind: io::ErrorKind) -> Self {
        Self::OpenVideo {
            video_path: path.as_ref().to_path_buf(),
            kind,
        }
    }

    pub fn parent_dir(path: impl AsRef<Path>) -> Self {
        Self::FaliedToGetParentDir {
            path: path.as_ref().to_path_buf(),
        }
    }
}
