/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::fmt::Display;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use derive_more::derive::{Deref, DerefMut};
use fs_err as fs;

use super::{Config, UserPath};
use crate::model::Result;
use crate::model::media::Media;

const DEFAULT_MEDIA_NAME: &str = "New media";

#[derive(Debug, Clone, Deref, DerefMut)]
pub struct MediaHandler {
    #[deref_mut]
    #[deref]
    media: Media,
    config: Arc<Config>,
}

impl MediaHandler {
    pub fn new(media_name: impl Into<String>, config: Arc<Config>) -> Result<Self> {
        let media = Media::new(media_name);
        let handler = Self { media, config };
        handler.save()?;
        Ok(handler)
    }

    pub fn with_default_name(config: Arc<Config>) -> Result<Self> {
        let name = find_available_name(&config.data_dir);
        Self::new(name, config)
    }

    fn save(&self) -> Result<()> {
        self.media.save(self.path())?;
        Ok(())
    }

    fn changed(&mut self) -> Result<()> {
        self.changing_date = chrono::Local::now();
        self.save()
    }

    pub async fn read(path: impl AsRef<Path>, config: Arc<Config>) -> Result<Self> {
        let media = Self {
            media: Media::read(path).await?,
            config,
        };
        Ok(media)
    }

    pub fn rename(&mut self, new_name: impl Into<String>) -> Result<()> {
        let new_name = new_name.into();
        if self.media.name == new_name {
            return Ok(());
        }
        let new_file_name = file_name(&new_name);
        let new_path = self.config.path_to_media(&new_file_name);
        fs::rename(self.path(), &new_path)?;
        self.media.name = new_name;
        self.changed()?;
        Ok(())
    }

    pub fn remove_file(&self) -> Result<()> {
        fs::remove_file(self.path())?;
        Ok(())
    }

    pub fn name(&self) -> &str {
        &self.media.name
    }

    pub const fn chapter(&self) -> u8 {
        self.media.chapter
    }

    pub const fn episode(&self) -> u8 {
        self.media.episode
    }

    pub fn chapter_path(&self) -> &UserPath {
        &self.media.chapter_path
    }

    pub fn next_chapter_path<'a>(&self) -> impl Future<Output = Result<PathBuf>> + 'a {
        self.media.next_chapter_path()
    }

    fn file_name(&self) -> String {
        file_name(&self.media.name)
    }

    pub fn set_chapter(&mut self, value: u8) -> Result<()> {
        self.media.chapter = value;
        self.changed()
    }

    pub fn set_episode(&mut self, value: u8) -> Result<()> {
        self.media.episode = value;
        self.changed()
    }

    pub fn set_chapter_path(&mut self, value: UserPath) -> Result<()> {
        self.media.chapter_path = value;
        self.changed()
    }

    fn path(&self) -> PathBuf {
        self.config.path_to_media(self.file_name())
    }
}

pub fn file_name(name: impl Display) -> String {
    format!("{name}.json")
}

fn find_available_name(path: impl AsRef<Path>) -> String {
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
