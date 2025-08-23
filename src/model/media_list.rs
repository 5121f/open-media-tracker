// SPDX-License-Identifier: MPL-2.0 OR GPL-2.0-or-later

use std::sync::Arc;

use derive_more::derive::{Deref, DerefMut, From};

use crate::model::error::{ErrorKind, Result};
use crate::model::{Config, MaybeError, MediaHandler};
use crate::utils::read_dir;

pub type MediaListRef<'a> = &'a [MediaHandler];
pub type MediaListRefMut<'a> = &'a mut [MediaHandler];

#[derive(Deref, DerefMut, Debug, Clone, From, Default)]
pub struct MediaList(Vec<MediaHandler>);

impl MediaList {
    pub const fn new() -> Self {
        Self(Vec::new())
    }

    pub fn remove(&mut self, id: usize) -> Result<()> {
        let media = &self.0[id];
        media.remove_file()?;
        self.0.remove(id);
        Ok(())
    }

    pub async fn read(config: Arc<Config>) -> MaybeError<Self, ErrorKind> {
        let dir_content = match read_dir(&config.data_dir).await {
            Ok(dir_content) => dir_content,
            Err(err) => return MaybeError::error(err),
        };
        let mut error = None;
        let mut media_list = Vec::with_capacity(dir_content.len());
        for entry in dir_content {
            match MediaHandler::read(&entry, config.clone()).await {
                Ok(media) => media_list.push(media),
                Err(err) => error = Some(err),
            }
        }
        MaybeError {
            value: media_list.into(),
            error,
        }
    }

    /// Rename media with check on unique
    pub fn rename_media(&mut self, media_id: usize, new_name: impl Into<String>) -> Result<()> {
        let new_name = new_name.into();
        if self.name_is_used(&new_name) {
            return Err(ErrorKind::media_name_is_used(new_name));
        }
        self.0[media_id].rename(new_name)?;
        Ok(())
    }

    /// Insert media to the `MediaList` and return its index
    pub fn insert(&mut self, media: MediaHandler) -> usize {
        let index = self.0.len();
        self.0.insert(index, media);
        index
    }

    fn name_is_used(&self, name: &str) -> bool {
        self.0.iter().any(|s| s.name == name)
    }
}
