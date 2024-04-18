use std::{io, path::Path};

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

#[derive(Debug, Clone, thiserror::Error)]
pub enum ErrorKind {
    #[error("{path}: {kind}")]
    FSIO { path: String, kind: io::ErrorKind },
    #[error("{path}: file parsing error: {source}")]
    Parce { path: String, source: SpannedError },
    #[error("{serial_name}: Serialize error: ")]
    SerialSerialize {
        serial_name: String,
        source: ron::Error,
    },
    #[error("Could not be found user's state directory")]
    StateDirNotFound,
    #[error("{video_path}: Falied to open video in default program: {kind}")]
    OpenVideo {
        video_path: String,
        kind: io::ErrorKind,
    },
    #[error("Uncnown error")]
    Unknown,
}

impl ErrorKind {
    pub fn fsio<P: AsRef<Path>>(path: P, source: io::Error) -> Self {
        Self::FSIO {
            path: path.as_ref().display().to_string(),
            kind: source.kind(),
        }
    }

    pub fn parse<P: AsRef<Path>>(path: P, source: SpannedError) -> Self {
        Self::Parce {
            path: path.as_ref().display().to_string(),
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
            video_path: path.as_ref().display().to_string(),
            kind,
        }
    }
}
