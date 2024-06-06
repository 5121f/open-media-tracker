use std::{
    fs,
    num::NonZeroU8,
    path::{Path, PathBuf},
};

use ron::{de::SpannedError, ser::PrettyConfig};
use serde::{Deserialize, Serialize};

use crate::error::FSIOError;

const DEFAULT_MEDIA_NAME: &str = "New media";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Media {
    name: String,
    chapter: NonZeroU8,
    episode: NonZeroU8,
    chapter_path: PathBuf,
    #[serde(skip)]
    dest_path: PathBuf,
}

impl Media {
    pub fn new(dest_path: PathBuf) -> Result<Self> {
        let one = NonZeroU8::MIN;
        let media = Self {
            name: find_availible_name(&dest_path),
            chapter: one,
            episode: one,
            chapter_path: Default::default(),
            dest_path,
        };
        media.save()?;
        Ok(media)
    }

    pub async fn read_from_file(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let file_content = async_fs::read_to_string(path)
            .await
            .map_err(|source| FSIOError::new(path, source))?;
        let media =
            ron::from_str(&file_content).map_err(|source| MediaError::deserialize(path, source))?;
        let media = Media {
            dest_path: path.to_owned(),
            ..media
        };
        Ok(media)
    }

    pub fn file_name(&self) -> String {
        file_name(&self.name)
    }

    pub fn rename(&mut self, new_name: String) -> Result<()> {
        if self.name != new_name {
            let new_path = self.parent().join(file_name(&new_name));
            fs::rename(&self.dest_path, &new_path)
                .map_err(|source| FSIOError::new(self.name.clone(), source))?;
            self.name = new_name;
            self.dest_path = new_path;
        }
        self.save()?;
        Ok(())
    }

    pub fn save(&self) -> Result<()> {
        let content = self.ser_to_ron()?;
        let parent = self.parent();
        if !parent.exists() {
            fs::create_dir(&self.dest_path)
                .map_err(|source| FSIOError::new(&self.dest_path, source))?;
        }
        fs::write(&self.dest_path, content)
            .map_err(|source| FSIOError::new(&self.dest_path, source))?;
        Ok(())
    }

    pub fn remove_file(&self) -> Result<()> {
        fs::remove_file(&self.dest_path)
            .map_err(|source| FSIOError::new(&self.dest_path, source).into())
    }

    pub fn chapter_path_is_present(&self) -> bool {
        !self.chapter_path.as_os_str().is_empty()
    }

    pub fn set_chapter(&mut self, value: NonZeroU8) -> Result<()> {
        self.chapter = value;
        self.save()
    }

    pub fn set_episode(&mut self, value: NonZeroU8) -> Result<()> {
        self.episode = value;
        self.save()
    }

    pub fn set_chapter_path(&mut self, value: PathBuf) -> Result<()> {
        self.chapter_path = value;
        self.save()
    }

    pub fn chapter(&self) -> NonZeroU8 {
        self.chapter
    }

    pub fn episode(&self) -> NonZeroU8 {
        self.episode
    }

    pub fn chapter_path(&self) -> &Path {
        &self.chapter_path
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    fn parent(&self) -> &Path {
        self.dest_path.parent().unwrap_or(Path::new("/"))
    }

    fn ser_to_ron(&self) -> Result<String> {
        ron::ser::to_string_pretty(&self, PrettyConfig::new())
            .map_err(|source| MediaError::serialize(self.name.clone(), source))
    }
}

pub fn file_name(name: &str) -> String {
    format!("{name}.ron")
}

fn find_availible_name(path: impl AsRef<Path>) -> String {
    let path = path.as_ref();
    let mut i = 1;
    let mut potential_name = DEFAULT_MEDIA_NAME.to_string();
    loop {
        let potential_file_name = file_name(&potential_name);
        let potential_name_used = path.join(potential_file_name).exists();
        if !potential_name_used {
            return potential_name;
        }
        potential_name = format!("{DEFAULT_MEDIA_NAME} {i}");
        i += 1;
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum MediaError {
    #[error("{name}: Serialize error: {source}")]
    Serialize { name: String, source: ron::Error },
    #[error("{path}: file parsing error: {source}")]
    Deserialize { path: PathBuf, source: SpannedError },
    #[error(transparent)]
    FSIO(#[from] FSIOError),
}

impl MediaError {
    fn serialize(name: String, source: ron::Error) -> Self {
        Self::Serialize { name, source }
    }

    fn deserialize(path: impl AsRef<Path>, source: SpannedError) -> Self {
        let path = path.as_ref().to_path_buf();
        Self::Deserialize { path, source }
    }
}

type Result<T> = std::result::Result<T, MediaError>;
