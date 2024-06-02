use std::{
    ops::{Deref, DerefMut},
    sync::Arc,
};

use crate::{
    config::Config,
    error::ErrorKind,
    media::{Media, MediaError},
    utils,
};

#[derive(Debug, Clone, Default)]
pub struct MediaList(Vec<Media>);

impl MediaList {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn remove(&mut self, id: usize) -> Result<(), ErrorKind> {
        let media = &self.0[id];
        media.remove_file()?;
        self.0.remove(id);
        Ok(())
    }

    pub async fn read(config: Arc<Config>) -> Result<Self, ErrorKind> {
        let dir_content = utils::read_dir(&config.data_dir)?;
        let mut media_list = Vec::with_capacity(dir_content.len());
        for entry in dir_content {
            let config = Arc::clone(&config);
            let media = Media::read_from_file(entry, config).await?;
            media_list.push(media);
        }
        Ok(media_list.into())
    }

    /// Rename media with check on unique
    pub fn rename_media(&mut self, media_id: usize, new_name: String) -> Result<(), MediaErrror> {
        if self.name_is_used(&new_name) {
            return Err(MediaErrror::NameIsUsed);
        }
        self.0[media_id].rename(new_name)?;
        Ok(())
    }

    fn name_is_used(&self, name: &str) -> bool {
        self.0.iter().any(|s| s.name() == name)
    }
}

impl Deref for MediaList {
    type Target = Vec<Media>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for MediaList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Vec<Media>> for MediaList {
    fn from(value: Vec<Media>) -> Self {
        Self(value)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum MediaErrror {
    #[error("Name is used")]
    NameIsUsed,
    #[error(transparent)]
    MediaError(#[from] MediaError),
    #[error(transparent)]
    ErrorKind(#[from] ErrorKind),
}
