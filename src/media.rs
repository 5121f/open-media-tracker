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
            let series = Series::read_from_file(entry, Arc::clone(&config)).await?;
            media.push(series);
        }
        Ok(media.into())
    }

    pub fn name_is_used(&self, name: &str) -> bool {
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
