use std::{
    fs,
    num::NonZeroU8,
    path::{Path, PathBuf},
    rc::Rc,
};

use ron::ser::PrettyConfig;
use serde::{Deserialize, Serialize};

use crate::{config::Config, error::ErrorKind, utils};

const DEFAULT_SERIAL_NAME: &str = "New serial";

#[derive(Serialize, Deserialize)]
pub struct Serial {
    name: String,
    season: NonZeroU8,
    seria: NonZeroU8,
    season_path: PathBuf,
    #[serde(skip)]
    config: Rc<Config>,
}

impl Serial {
    pub fn new(config: Rc<Config>) -> Result<Self, ErrorKind> {
        let one = NonZeroU8::MIN;
        Ok(Self {
            name: find_availible_new_name(&config.data_dir)?,
            season: one,
            seria: one,
            season_path: Default::default(),
            config,
        })
    }

    pub fn read_from_file(path: impl AsRef<Path>, config: Rc<Config>) -> Result<Self, ErrorKind> {
        let path = path.as_ref();
        let file_content =
            fs::read_to_string(path).map_err(|source| ErrorKind::fsio(path, source))?;
        let mut serail: Serial =
            ron::from_str(&file_content).map_err(|source| ErrorKind::parse(path, source))?;
        serail.config = config;
        Ok(serail)
    }

    pub fn file_name(&self) -> String {
        file_name(&self.name)
    }

    pub fn rename(&mut self, new_name: String) -> Result<(), ErrorKind> {
        let data_dir = &self.config.data_dir;
        if self.name != new_name {
            let current_path = self.path(data_dir);
            let new_path = data_dir.join(file_name(&new_name));
            self.name = new_name;
            fs::rename(current_path, new_path)
                .map_err(|source| ErrorKind::fsio(self.name.clone(), source))?;
        }
        self.save()?;
        Ok(())
    }

    pub fn save(&self) -> Result<(), ErrorKind> {
        let dir = &self.config.data_dir;
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

    pub fn set_season(&mut self, value: NonZeroU8) -> Result<(), ErrorKind> {
        self.season = value;
        self.save()
    }

    pub fn set_seria(&mut self, value: NonZeroU8) -> Result<(), ErrorKind> {
        self.seria = value;
        self.save()
    }

    pub fn set_season_path(&mut self, value: PathBuf) -> Result<(), ErrorKind> {
        self.season_path = value;
        self.save()
    }

    pub fn season(&self) -> NonZeroU8 {
        self.season
    }

    pub fn seria(&self) -> NonZeroU8 {
        self.seria
    }

    pub fn season_path(&self) -> &Path {
        &self.season_path
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    fn path(&self, dir: impl AsRef<Path>) -> PathBuf {
        dir.as_ref().join(self.file_name())
    }
}

pub fn file_name(name: &str) -> String {
    format!("{}.ron", name)
}

fn default_serial_file_name() -> String {
    file_name(DEFAULT_SERIAL_NAME)
}

fn find_availible_new_name(path: impl AsRef<Path>) -> Result<String, ErrorKind> {
    let dir_entrys = utils::read_dir(path)?;
    let file_names: Vec<_> = dir_entrys
        .iter()
        .flat_map(|e| e.file_name())
        .map(|n| n.to_string_lossy())
        .collect();
    let default_serial_name_availible =
        !file_names.iter().any(|n| n == &default_serial_file_name());
    dbg!(&file_names);
    println!("{}", default_serial_name_availible);
    if default_serial_name_availible {
        return Ok(DEFAULT_SERIAL_NAME.to_string());
    }
    let mut i = 1;
    loop {
        let potential_name = format!("{} {}", DEFAULT_SERIAL_NAME, i);
        let potential_file_name = file_name(&potential_name);
        let potential_name_availible = !file_names
            .iter()
            .any(|n: &std::borrow::Cow<'_, str>| n == &potential_file_name);
        if potential_name_availible {
            return Ok(potential_name);
        }
        i += 1;
    }
}
