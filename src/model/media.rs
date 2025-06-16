/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::num::NonZeroU8;
use std::path::{Path, PathBuf};

use fs_err as fs;
use serde::{Deserialize, Serialize};

use crate::model::{ErrorKind, Result};
use crate::read_dir;

use super::{EpisodeList, UserPath};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Media {
    pub name: String,
    pub chapter: NonZeroU8,
    pub episode: NonZeroU8,
    pub chapter_path: UserPath,
}

impl Media {
    pub fn new(name: impl Into<String>) -> Self {
        let one = NonZeroU8::MIN;
        Self {
            name: name.into(),
            chapter: one,
            episode: one,
            chapter_path: UserPath::default(),
        }
    }

    pub async fn read(path: impl AsRef<Path>) -> Result<Self> {
        let file_content = async_fs::read_to_string(&path).await?;
        let media = serde_json::from_str(&file_content)
            .map_err(|source| ErrorKind::deserialize(path.as_ref(), source))?;
        Ok(media)
    }

    pub fn save(&self, path: impl AsRef<Path>) -> Result<()> {
        let path = path.as_ref();

        let parent = path.parent().ok_or_else(|| ErrorKind::data_dir(path))?;
        if !parent.exists() {
            fs::create_dir(path)?;
        }
        let content = serde_json::to_string_pretty(&self)
            .map_err(|source| ErrorKind::serialize(source, &self.name))?;
        fs::write(path, content)?;
        Ok(())
    }

    pub fn next_chapter_path(&self) -> Result<PathBuf> {
        let chapter_path = self.chapter_path.clone().into_path_buf();
        let chapter_dir_name = chapter_path.file_name().unwrap_or_default();
        let parent = chapter_path.parent().unwrap_or_else(|| Path::new("/"));
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
        EpisodeList::read(self.chapter_path.clone().into_path_buf())
    }

    pub fn set_episode_to_one(&mut self) {
        self.episode = NonZeroU8::MIN;
    }
}
