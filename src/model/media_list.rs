/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::sync::Arc;

use derive_more::derive::{Deref, DerefMut, From};

use crate::{
    model::{
        error::{ErrorKind, Result},
        MediaHandler,
    },
    read_dir,
};

use super::{Config, Placeholder};

#[derive(Deref, DerefMut, Debug, Clone, From)]
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

    pub async fn read(config: Arc<Config>) -> Result<Self> {
        let dir_content = read_dir(&config.data_dir)?;
        let mut media_list = Vec::with_capacity(dir_content.len());
        for entry in dir_content {
            let media = MediaHandler::read(entry, config.clone()).await?;
            media_list.push(media);
        }
        Ok(media_list.into())
    }

    /// Rename media with check on unique
    pub fn rename_media(&mut self, media_id: usize, new_name: String) -> Result<()> {
        if self.name_is_used(&new_name) {
            return Err(ErrorKind::media_name_is_used(&new_name));
        }
        self.0[media_id].rename(new_name)?;
        Ok(())
    }

    /// Insert media to the `MediaList` and return his index
    pub fn insert(&mut self, media: MediaHandler) -> usize {
        let index = self.0.len();
        self.0.insert(index, media);
        index
    }

    fn name_is_used(&self, name: &str) -> bool {
        self.0.iter().any(|s| s.name == name)
    }
}

impl Placeholder for MediaList {
    fn placeholder() -> Self {
        Self::new()
    }
}
