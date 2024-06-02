use std::{fs, path::PathBuf};

use crate::error::ErrorKind;

#[derive(Debug, Default)]
pub struct Config {
    pub data_dir: PathBuf,
}

impl Config {
    pub fn read() -> Result<Self, ErrorKind> {
        let data_dir = dirs::data_dir()
            .ok_or(ErrorKind::UserDataDirNotFound)?
            .join("open media tracker");
        if !data_dir.exists() {
            fs::create_dir(&data_dir).map_err(|source| ErrorKind::fsio(&data_dir, source))?;
        }
        Ok(Self { data_dir })
    }
}
