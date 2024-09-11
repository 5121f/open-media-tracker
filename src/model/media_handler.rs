/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::{
    fs,
    num::NonZeroU8,
    path::{Path, PathBuf},
};

use crate::model::{
    error::{FSIOError, Result},
    media::Media,
};

const DEFAULT_MEDIA_NAME: &str = "New media";

#[derive(Debug, Clone)]
pub struct MediaHandler {
    pub media: Media,
    pub save_folder: PathBuf,
}

impl MediaHandler {
    pub fn new(save_folder: PathBuf, media_name: String) -> Self {
        let media = Media::new(media_name);
        Self { media, save_folder }
    }

    pub fn with_default_name(save_folder: PathBuf) -> Self {
        let name = find_available_name(&save_folder);
        Self::new(save_folder, name)
    }

    fn save(&self) -> Result<()> {
        self.media.save(self.path())?;
        Ok(())
    }

    pub async fn from_file(path: PathBuf) -> Result<MediaHandler> {
        Ok(MediaHandler {
            media: Media::from_file(&path).await?,
            save_folder: path.parent().unwrap_or(Path::new("/")).to_path_buf(),
        })
    }

    pub fn rename(&mut self, new_name: String) -> Result<()> {
        if self.media.name == new_name {
            return Ok(());
        }
        let new_path = self.save_folder.join(file_name(&new_name));
        fs::rename(self.path(), &new_path)
            .map_err(|source| FSIOError::new(&self.media.name, source))?;
        self.media.name = new_name;
        self.save()?;
        Ok(())
    }

    pub fn remove_file(&self) -> Result<()> {
        fs::remove_file(self.path()).map_err(|source| FSIOError::new(self.path(), source).into())
    }

    pub fn name(&self) -> &str {
        &self.media.name
    }

    pub fn chapter(&self) -> NonZeroU8 {
        self.media.chapter
    }

    pub fn episode(&self) -> NonZeroU8 {
        self.media.episode
    }

    pub fn chapter_path(&self) -> &Path {
        &self.media.chapter_path
    }

    pub fn chapter_path_is_present(&self) -> bool {
        self.media.chapter_path_is_present()
    }

    pub fn next_chapter_path(&self) -> Result<PathBuf> {
        self.media.next_chapter_path().map_err(Into::into)
    }

    fn file_name(&self) -> String {
        file_name(&self.media.name)
    }

    pub fn set_chapter(&mut self, value: NonZeroU8) -> Result<()> {
        self.media.chapter = value;
        self.save()
    }

    pub fn set_episode(&mut self, value: NonZeroU8) -> Result<()> {
        self.media.episode = value;
        self.save()
    }

    pub fn set_chapter_path(&mut self, value: PathBuf) -> Result<()> {
        self.media.chapter_path = value;
        self.save()
    }

    fn path(&self) -> PathBuf {
        self.save_folder.join(self.file_name())
    }
}

pub fn file_name(name: &str) -> String {
    format!("{name}.ron")
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
