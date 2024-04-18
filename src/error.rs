use std::{io, path::Path};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{path}: {source}")]
    FSIO { path: String, source: io::Error },
}

impl Error {
    pub fn fsio<P: AsRef<Path>>(path: P, source: io::Error) -> Self {
        Self::FSIO {
            path: path.as_ref().display().to_string(),
            source,
        }
    }
}
