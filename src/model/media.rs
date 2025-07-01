/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::path::{Path, PathBuf};

use chrono::DateTime;
use fs_err as fs;
use serde::{Deserialize, Serialize};

use crate::model::{ErrorKind, Result};
use crate::utils;

use super::{EpisodeList, UserPath};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Media {
    pub name: String,
    pub chapter: u8,
    pub episode: u8,
    pub chapter_path: UserPath,
    pub adding_date: DateTime<chrono::Local>,
    pub changing_date: DateTime<chrono::Local>,
}

impl Media {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            chapter: 1,
            episode: 1,
            chapter_path: UserPath::default(),
            adding_date: chrono::Local::now(),
            changing_date: chrono::Local::now(),
        }
    }

    pub async fn read(path: impl AsRef<Path>) -> Result<Self> {
        let file_content = fs_err::tokio::read_to_string(&path).await?;
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

    pub fn next_chapter_path<'a>(&self) -> impl Future<Output = Result<PathBuf>> + 'a {
        let path = self.chapter_path.clone().into_path_buf();
        async { utils::next_dir(path).await.map_err(Into::into) }
    }

    pub fn episode_list(&self) -> Result<EpisodeList> {
        EpisodeList::read(self.chapter_path.clone().into_path_buf())
    }
}
