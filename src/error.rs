use std::{
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

impl ToString for Error {
    fn to_string(&self) -> String {
        self.kind.to_string()
    }
}

impl From<ErrorKind> for Error {
    fn from(value: ErrorKind) -> Self {
        Error::general(value)
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum ErrorKind {
    #[error("{path}: {kind}")]
    FSIO { path: PathBuf, kind: io::ErrorKind },
    #[error("{path}: file parsing error: {source}")]
    Parce { path: PathBuf, source: SpannedError },
    #[error("{serial_name}: Serialize error: ")]
    SerialSerialize {
        serial_name: String,
        source: ron::Error,
    },
    #[error("Could not be found user's state directory")]
    StateDirNotFound,
    #[error("{video_path}: Falied to open video in default program: {kind}")]
    OpenVideo {
        video_path: PathBuf,
        kind: io::ErrorKind,
    },
    #[error("{path}: Failed to find parent directory")]
    FaliedToGetParentDir { path: PathBuf },
    #[error("Uncnown error")]
    Unknown,
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

    pub fn serial_serialize(name: String, source: ron::Error) -> Self {
        Self::SerialSerialize {
            serial_name: name,
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
