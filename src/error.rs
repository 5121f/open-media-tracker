use std::{io, path::Path};

use ron::de::SpannedError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{path}: {source}")]
    FSIO { path: String, source: io::Error },
    #[error("{path}: file parsing error: {source}")]
    ParceError { path: String, source: SpannedError },
}

impl Error {
    pub fn fsio<P: AsRef<Path>>(path: P, source: io::Error) -> Self {
        Self::FSIO {
            path: path.as_ref().display().to_string(),
            source,
        }
    }

    pub fn parse<P: AsRef<Path>>(path: P, source: SpannedError) -> Self {
        Self::ParceError {
            path: path.as_ref().display().to_string(),
            source,
        }
    }
}
