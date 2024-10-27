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

use ron::ser::PrettyConfig;
use serde::{Deserialize, Serialize};

use crate::{
    model::error::{ErrorKind, FSIOError, Result},
    read_dir,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Media {
    pub name: String,
    pub chapter: NonZeroU8,
    pub episode: NonZeroU8,
    pub chapter_path: PathBuf,
}

impl Media {
    pub fn new(name: String) -> Self {
        let one = NonZeroU8::MIN;
        Self {
            name,
            chapter: one,
            episode: one,
            chapter_path: Default::default(),
        }
    }

    async fn _read(path: &Path) -> Result<Self> {
        let file_content = async_fs::read_to_string(&path)
            .await
            .map_err(|source| FSIOError::new(path, source))?;
        let media =
            ron::from_str(&file_content).map_err(|source| ErrorKind::deserialize(path, source))?;
        Ok(media)
    }
    pub async fn read(path: PathBuf) -> Result<Self> {
        Self::_read(path.as_ref()).await
    }

    fn _save(&self, path: &Path) -> Result<()> {
        let content = self.ser_to_ron()?;
        if !path.parent().unwrap_or_else(|| Path::new("/")).exists() {
            fs::create_dir(path).map_err(|source| FSIOError::new(path, source))?;
        }
        fs::write(path, content).map_err(|source| FSIOError::new(path, source))?;
        Ok(())
    }
    pub fn save(&self, path: impl AsRef<Path>) -> Result<()> {
        self._save(path.as_ref())
    }

    pub fn chapter_path_is_present(&self) -> bool {
        !self.chapter_path.as_os_str().is_empty()
    }

    pub fn next_chapter_path(&self) -> Result<PathBuf> {
        let chapter_dir_name = self
            .chapter_path
            .file_name()
            .ok_or(ErrorKind::FindNextChapterPath)?;
        let parent = self
            .chapter_path
            .parent()
            .ok_or_else(|| ErrorKind::find_parent_dir(&self.chapter_path))?
            .to_owned();
        let mut paths = read_dir::read_dir_for_dirs(parent)?;
        paths.sort();
        let (current_dir_index, _) = paths
            .iter()
            .flat_map(|path| path.file_name())
            .enumerate()
            .find(|(_, file_name)| *file_name == chapter_dir_name)
            .ok_or(ErrorKind::FindNextChapterPath)?;
        let next_chapter_index = current_dir_index + 1;
        if next_chapter_index >= paths.len() {
            return Err(ErrorKind::FindNextChapterPath);
        }
        let next_dir = paths[next_chapter_index].to_path_buf();
        Ok(next_dir)
    }

    fn ser_to_ron(&self) -> Result<String> {
        ron::ser::to_string_pretty(&self, PrettyConfig::new())
            .map_err(|source| ErrorKind::serialize(self.name.clone(), source))
    }

    pub fn set_chapter_to_one(&mut self) {
        self.chapter = NonZeroU8::MIN;
    }

    pub fn set_episode_to_one(&mut self) {
        self.episode = NonZeroU8::MIN;
    }
}
