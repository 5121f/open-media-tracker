/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::{
    ops::{Deref, DerefMut},
    path::Path,
};

use crate::{
    model::{
        error::FSIOError,
        media::{MediaHandler, MediaHandlerError},
    },
    utils,
};

#[derive(Debug, Clone, Default)]
pub struct MediaList(Vec<MediaHandler>);

impl MediaList {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn remove(&mut self, id: usize) -> Result<()> {
        let media = &self.0[id];
        media.remove_file()?;
        self.0.remove(id);
        Ok(())
    }

    pub async fn read(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let dir_content = utils::read_dir(path)?;
        let mut media_list = Vec::with_capacity(dir_content.len());
        for entry in dir_content {
            let media = MediaHandler::from_file(entry).await?;
            media_list.push(media);
        }
        Ok(media_list.into())
    }

    /// Rename media with check on unique
    pub fn rename_media(&mut self, media_id: usize, new_name: String) -> Result<()> {
        if self.name_is_used(&new_name) {
            return Err(Error::NameIsUsed);
        }
        self.0[media_id].rename(new_name)?;
        Ok(())
    }

    fn name_is_used(&self, name: &str) -> bool {
        self.0.iter().any(|s| s.media.name == name)
    }
}

impl Deref for MediaList {
    type Target = Vec<MediaHandler>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for MediaList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Vec<MediaHandler>> for MediaList {
    fn from(value: Vec<MediaHandler>) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum Error {
    #[error("Name is used")]
    NameIsUsed,
    #[error(transparent)]
    Media(#[from] MediaHandlerError),
    #[error(transparent)]
    FSIO(#[from] FSIOError),
}

type Result<T> = std::result::Result<T, Error>;
