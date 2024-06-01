use std::{
    fs,
    num::NonZeroU8,
    path::{Path, PathBuf},
    sync::Arc,
};

use ron::{de::SpannedError, ser::PrettyConfig};
use serde::{Deserialize, Serialize};

use crate::{config::Config, error::FSIOError};

const DEFAULT_SERIES_NAME: &str = "New series";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Series {
    name: String,
    season: NonZeroU8,
    episode: NonZeroU8,
    season_path: PathBuf,
    #[serde(skip)]
    config: Arc<Config>,
}

impl Series {
    pub fn new(config: Arc<Config>) -> Result<Self, SeriesError> {
        let one = NonZeroU8::MIN;
        let series = Self {
            name: find_availible_name(&config.data_dir),
            season: one,
            episode: one,
            season_path: Default::default(),
            config,
        };
        series.save()?;
        Ok(series)
    }

    pub async fn read_from_file(
        path: impl AsRef<Path>,
        config: Arc<Config>,
    ) -> Result<Self, SeriesError> {
        let path = path.as_ref();
        let file_content = async_fs::read_to_string(path)
            .await
            .map_err(|source| FSIOError::new(path, source))?;
        let series = ron::from_str(&file_content)
            .map_err(|source| SeriesError::deserialize(path, source))?;
        let series = Series { config, ..series };
        Ok(series)
    }

    pub fn file_name(&self) -> String {
        file_name(&self.name)
    }

    pub fn rename(&mut self, new_name: String) -> Result<(), SeriesError> {
        if self.name != new_name {
            let new_path = self.config.data_dir.join(file_name(&new_name));
            fs::rename(self.path(), new_path)
                .map_err(|source| FSIOError::new(self.name.clone(), source))?;
            self.name = new_name;
        }
        self.save()?;
        Ok(())
    }

    pub fn save(&self) -> Result<(), SeriesError> {
        let dir = &self.config.data_dir;
        let content = self.ser_to_ron()?;
        if !dir.exists() {
            fs::create_dir(&dir).map_err(|source| FSIOError::new(dir, source))?;
        }
        let path = self.path();
        fs::write(path, content).map_err(|source| FSIOError::new(dir, source))?;
        Ok(())
    }

    pub fn remove_file(&self) -> Result<(), FSIOError> {
        let path = self.path();
        fs::remove_file(&path).map_err(|source| FSIOError::new(path, source))
    }

    pub fn season_path_is_present(&self) -> bool {
        !self.season_path.as_os_str().is_empty()
    }

    pub fn set_season(&mut self, value: NonZeroU8) -> Result<(), SeriesError> {
        self.season = value;
        self.save()
    }

    pub fn set_episode(&mut self, value: NonZeroU8) -> Result<(), SeriesError> {
        self.episode = value;
        self.save()
    }

    pub fn set_season_path(&mut self, value: PathBuf) -> Result<(), SeriesError> {
        self.season_path = value;
        self.save()
    }

    pub fn season(&self) -> NonZeroU8 {
        self.season
    }

    pub fn episode(&self) -> NonZeroU8 {
        self.episode
    }

    pub fn season_path(&self) -> &Path {
        &self.season_path
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    fn ser_to_ron(&self) -> Result<String, SeriesError> {
        ron::ser::to_string_pretty(&self, PrettyConfig::new())
            .map_err(|source| SeriesError::serialize(self.name.clone(), source))
    }

    fn path(&self) -> PathBuf {
        self.config.data_dir.join(self.file_name())
    }
}

pub fn file_name(name: &str) -> String {
    format!("{name}.ron")
}

fn find_availible_name(path: impl AsRef<Path>) -> String {
    let path = path.as_ref();
    let mut i = 1;
    let mut potential_name = DEFAULT_SERIES_NAME.to_string();
    loop {
        let potential_file_name = file_name(&potential_name);
        let potential_name_used = path.join(potential_file_name).exists();
        if !potential_name_used {
            return potential_name;
        }
        potential_name = format!("{DEFAULT_SERIES_NAME} {i}");
        i += 1;
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum SeriesError {
    #[error("{name}: Serialize error: {source}")]
    Serialize { name: String, source: ron::Error },
    #[error("{path}: file parsing error: {source}")]
    Deserialize { path: PathBuf, source: SpannedError },
    #[error(transparent)]
    FSIO(#[from] FSIOError),
}

impl SeriesError {
    fn serialize(name: String, source: ron::Error) -> Self {
        Self::Serialize { name, source }
    }

    fn deserialize(path: impl AsRef<Path>, source: SpannedError) -> Self {
        let path = path.as_ref().to_path_buf();
        Self::Deserialize { path, source }
    }
}
