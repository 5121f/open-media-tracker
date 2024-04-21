use std::{
    fs,
    num::NonZeroU8,
    path::{Path, PathBuf},
};

use ron::ser::PrettyConfig;
use serde::{Deserialize, Serialize};

use crate::error::ErrorKind;

#[derive(Serialize, Deserialize)]
pub struct Serial {
    pub name: String,
    pub season: NonZeroU8,
    pub seria: NonZeroU8,
    pub season_path: PathBuf,
}

impl Serial {
    pub fn new(name: String, season: NonZeroU8, seria: NonZeroU8, season_path: PathBuf) -> Self {
        Self {
            name,
            season,
            seria,
            season_path,
        }
    }

    pub fn read_from_file(path: impl AsRef<Path>) -> Result<Self, ErrorKind> {
        let path = path.as_ref();
        let file_content =
            fs::read_to_string(path).map_err(|source| ErrorKind::fsio(path, source))?;
        let serail =
            ron::from_str(&file_content).map_err(|source| ErrorKind::parse(path, source))?;
        Ok(serail)
    }

    pub fn file_name(&self) -> String {
        file_name(&self.name)
    }

    pub fn rename(&mut self, dir: impl AsRef<Path>, new_name: String) -> Result<(), ErrorKind> {
        let dir = dir.as_ref();
        if self.name != new_name {
            let current_path = self.path(dir);
            let new_path = dir.join(file_name(&new_name));
            self.name = new_name;
            fs::rename(current_path, new_path)
                .map_err(|source| ErrorKind::fsio(self.name.clone(), source))?;
        }
        self.save(dir)?;
        Ok(())
    }

    pub fn save(&self, dir: impl AsRef<Path>) -> Result<(), ErrorKind> {
        let dir = dir.as_ref();
        let content = ron::ser::to_string_pretty(&self, PrettyConfig::new())
            .map_err(|source| ErrorKind::serial_serialize(self.name.clone(), source))?;
        if !dir.exists() {
            fs::create_dir(&dir).map_err(|source| ErrorKind::fsio(dir, source))?;
        }
        let path = self.path(&dir);
        fs::write(path, content).map_err(|source| ErrorKind::fsio(dir, source))?;
        Ok(())
    }

    pub fn remove_file(&self, dir: impl AsRef<Path>) -> Result<(), ErrorKind> {
        let path = self.path(dir);
        fs::remove_file(&path).map_err(|source| ErrorKind::fsio(path, source))
    }

    pub fn season_path_is_present(&self) -> bool {
        !self.season_path.as_os_str().is_empty()
    }

    fn path(&self, dir: impl AsRef<Path>) -> PathBuf {
        dir.as_ref().join(self.file_name())
    }
}

pub fn file_name(name: &str) -> String {
    format!("{}.ron", name)
}

impl Default for Serial {
    fn default() -> Self {
        let one = NonZeroU8::MIN;
        Self {
            name: Default::default(),
            season: one,
            seria: one,
            season_path: Default::default(),
        }
    }
}
