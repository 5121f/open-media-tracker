use std::{
    ops::{Deref, DerefMut},
    sync::Arc,
};

use crate::{config::Config, error::ErrorKind, series::Series, utils::read_dir};

#[derive(Debug, Clone, Default)]
pub struct Media(Vec<Series>);

impl Media {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn remove(&mut self, id: usize) -> Result<(), ErrorKind> {
        let series = &self.0[id];
        series.remove_file()?;
        self.0.remove(id);
        Ok(())
    }

    pub async fn read(config: Arc<Config>) -> Result<Self, ErrorKind> {
        let dir_content = read_dir(&config.data_dir)?;
        let mut media = Vec::with_capacity(dir_content.len());
        for entry in dir_content {
            let config = Arc::clone(&config);
            let series = Series::read_from_file(entry, config).await?;
            media.push(series);
        }
        Ok(media.into())
    }

    pub fn rename_series(&mut self, series_id: usize, new_name: String) -> Result<(), MediaErrror> {
        if self.name_is_used(&new_name) {
            return Err(MediaErrror::NameIsUsed);
        }
        self.0[series_id].rename(new_name)?;
        Ok(())
    }

    fn name_is_used(&self, name: &str) -> bool {
        self.0.iter().any(|s| s.name() == name)
    }
}

impl Deref for Media {
    type Target = Vec<Series>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Media {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Vec<Series>> for Media {
    fn from(value: Vec<Series>) -> Self {
        Self(value)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum MediaErrror {
    #[error("Name is used")]
    NameIsUsed,
    #[error(transparent)]
    ErrorKind(#[from] ErrorKind),
}
