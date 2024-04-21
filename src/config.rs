use std::path::PathBuf;

use crate::error::ErrorKind;

#[derive(Default)]
pub struct Config {
    pub data_dir: PathBuf,
}

impl Config {
    pub fn read() -> Result<Self, ErrorKind> {
        let data_dir = dirs::data_dir()
            .ok_or(ErrorKind::UserDataDirNotFound)?
            .join("zcinema");
        Ok(Self { data_dir })
    }
}
