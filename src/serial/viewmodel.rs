use std::{
    fs,
    num::NonZeroU8,
    ops::{Deref, DerefMut},
    path::{Path, PathBuf},
    rc::Rc,
};

use ron::ser::PrettyConfig;

use crate::{error::ErrorKind, serial::model};

pub struct Serial(Rc<model::Serial>);

impl Serial {
    pub fn new(name: String, season: NonZeroU8, seria: NonZeroU8) -> Self {
        let model = model::Serial {
            name,
            current_season: season,
            current_seria: seria,
        };
        Self(Rc::new(model))
    }

    pub fn read_from_file(path: &Path) -> Result<Self, ErrorKind> {
        let file_content =
            fs::read_to_string(path).map_err(|source| ErrorKind::fsio(path, source))?;
        let model =
            ron::from_str(&file_content).map_err(|source| ErrorKind::parse(path, source))?;
        Ok(Self(Rc::new(model)))
    }

    pub fn file_name(&self) -> String {
        file_name(&self.0.name)
    }

    pub fn rc_clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }

    pub fn rename<P: AsRef<Path>>(&mut self, dir: P, new_name: String) -> Result<(), ErrorKind> {
        let dir = dir.as_ref();
        if self.0.name != new_name {
            let current_path = self.path(dir);
            let new_path = dir.join(file_name(&new_name));
            let serial = Rc::get_mut(&mut self.0).ok_or(ErrorKind::Uncnown)?;
            serial.name = new_name;
            fs::rename(current_path, new_path)
                .map_err(|source| ErrorKind::fsio(serial.name.clone(), source))?;
        }
        Ok(())
    }

    pub fn save<P: AsRef<Path>>(&self, dir: P) -> Result<(), ErrorKind> {
        let content = ron::ser::to_string_pretty(self.0.as_ref(), PrettyConfig::new())
            .map_err(|source| ErrorKind::serial_serialize(self.name.clone(), source))?;
        if !dir.as_ref().exists() {
            fs::create_dir(&dir).map_err(|source| ErrorKind::fsio(dir.as_ref(), source))?;
        }
        let path = self.path(&dir);
        fs::write(path, content).map_err(|source| ErrorKind::fsio(dir.as_ref(), source))?;
        Ok(())
    }

    pub fn remove_file<P: AsRef<Path>>(&self, dir: P) -> Result<(), ErrorKind> {
        let path = self.path(dir);
        fs::remove_file(&path).map_err(|source| ErrorKind::fsio(path, source))
    }

    pub fn change_season(&mut self, new_value: NonZeroU8) -> Result<(), ErrorKind> {
        let model = Rc::get_mut(&mut self.0).ok_or(ErrorKind::Uncnown)?;
        model.current_season = new_value;
        Ok(())
    }

    pub fn change_seria(&mut self, new_value: NonZeroU8) -> Result<(), ErrorKind> {
        let model = Rc::get_mut(&mut self.0).ok_or(ErrorKind::Uncnown)?;
        model.current_seria = new_value;
        Ok(())
    }

    // pub fn get_mut(&mut self) -> Result<&mut model::Serial> {
    //     Rc::get_mut(&mut self.0).ok_or(Error::Uncnown)
    // }

    fn path<P: AsRef<Path>>(&self, dir: P) -> PathBuf {
        dir.as_ref().join(self.file_name())
    }
}

impl Deref for Serial {
    type Target = Rc<model::Serial>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Serial {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<model::Serial> for Serial {
    fn from(value: model::Serial) -> Self {
        Self(Rc::new(value))
    }
}

pub fn file_name(name: &str) -> String {
    format!("{}.ron", name)
}
