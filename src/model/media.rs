/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::{
    num::NonZeroU8,
    path::{Path, PathBuf},
};

use fs_err as fs;
use ron::ser::PrettyConfig;
use serde::{Deserialize, Serialize};

use crate::{
    model::{ErrorKind, Result},
    read_dir,
};

use super::EpisodeList;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Media {
    pub name: String,
    pub chapter: NonZeroU8,
    pub episode: NonZeroU8,
    pub chapter_path: PathBuf,
}

impl Media {
    pub fn new(name: impl Into<String>) -> Self {
        let one = NonZeroU8::MIN;
        Self {
            name: name.into(),
            chapter: one,
            episode: one,
            chapter_path: PathBuf::default(),
        }
    }

    pub async fn read(path: &Path) -> Result<Self> {
        let file_content = async_fs::read_to_string(&path).await?;
        let media = ron::from_str(&file_content)
            .map_err(|source| ErrorKind::deserialize(path.to_owned(), source))?;
        Ok(media)
    }

    pub fn save(&self, path: impl AsRef<Path>) -> Result<()> {
        let path = path.as_ref();

        let content = self.ser_to_ron()?;
        if !path.parent().unwrap_or_else(|| Path::new("/")).exists() {
            fs::create_dir(path)?;
        }
        fs::write(path, content)?;
        Ok(())
    }

    pub fn chapter_path_is_present(&self) -> bool {
        !self.chapter_path.as_os_str().is_empty()
    }

    pub fn next_chapter_path(&self) -> Result<PathBuf> {
        let chapter_dir_name = self.chapter_path.file_name().unwrap_or_default();
        let parent = self.chapter_path.parent().unwrap_or_else(|| Path::new("/"));
        let mut paths = read_dir::read_dir_with_filter(parent, Path::is_dir)?;
        paths.sort();
        let (current_dir_index, _) = paths
            .iter()
            .filter_map(|path| path.file_name())
            .enumerate()
            .find(|(_, file_name)| *file_name == chapter_dir_name)
            .ok_or(ErrorKind::FindNextChapterPath)?;
        let next_chapter_index = current_dir_index + 1;
        if next_chapter_index >= paths.len() {
            return Err(ErrorKind::FindNextChapterPath);
        }
        let next_dir = paths.into_iter().take(next_chapter_index + 1).collect();
        Ok(next_dir)
    }

    pub fn episode_list(&self) -> Result<EpisodeList> {
        EpisodeList::read(&self.chapter_path)
    }

    fn ser_to_ron(&self) -> Result<String> {
        ron::ser::to_string_pretty(&self, PrettyConfig::new())
            .map_err(|source| ErrorKind::serialize(source, &self.name))
    }

    pub fn set_chapter_to_one(&mut self) {
        self.chapter = NonZeroU8::MIN;
    }

    pub fn set_episode_to_one(&mut self) {
        self.episode = NonZeroU8::MIN;
    }
}
