use std::{io, path::Path};

use ron::de::SpannedError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, thiserror::Error)]
pub enum Error {
    #[error("{path}: {kind}")]
    FSIO { path: String, kind: io::ErrorKind },
    #[error("{path}: file parsing error: {source}")]
    ParceError { path: String, source: SpannedError },
    #[error("Could not be found user's state directory")]
    StateDirNotFound,
    #[error("Uncnown error")]
    Uncnown,
}

impl Error {
    pub fn fsio<P: AsRef<Path>>(path: P, source: io::Error) -> Self {
        Self::FSIO {
            path: path.as_ref().display().to_string(),
            kind: source.kind(),
        }
    }

    pub fn parse<P: AsRef<Path>>(path: P, source: SpannedError) -> Self {
        Self::ParceError {
            path: path.as_ref().display().to_string(),
            source,
        }
    }
}
