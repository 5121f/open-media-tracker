/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::io::Write;
use std::path::{Path, PathBuf};

use chrono::DateTime;
use fs_err as fs;
use serde::{Deserialize, Serialize};

use crate::model::{ErrorKind, Result};
use crate::utils;

use super::Episode;
use super::episode::read_episodes;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Media {
    pub name: String,
    pub chapter: u8,
    pub episode: u8,
    pub chapter_path: PathBuf,
    pub adding_date: DateTime<chrono::Local>,
    pub changing_date: DateTime<chrono::Local>,
}

impl Media {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            chapter: 1,
            episode: 1,
            chapter_path: PathBuf::new(),
            adding_date: chrono::Local::now(),
            changing_date: chrono::Local::now(),
        }
    }

    pub async fn read(path: &Path) -> Result<Self> {
        let file_content = fs_err::tokio::read_to_string(&path).await?;
        let media = serde_json::from_str(&file_content)
            .map_err(|source| ErrorKind::deserialize(path, source))?;
        Ok(media)
    }

    pub fn save(&self, path: impl AsRef<Path>) -> Result<()> {
        let path = path.as_ref();

        let parent = path.parent().ok_or_else(|| ErrorKind::data_dir(path))?;
        if !parent.exists() {
            fs::create_dir(path)?;
        }
        let mut file = fs::File::create(path)?;
        serde_json::to_writer_pretty(&file, &self)
            .map_err(|source| ErrorKind::serialize(source, &self.name))?;
        file.write_all(b"\n")?;
        Ok(())
    }

    pub fn next_chapter_path<'a>(&self) -> impl Future<Output = Result<PathBuf>> + 'a {
        let path = self.chapter_path.clone();
        async { utils::next_dir(path).await }
    }

    pub fn episode_list<'a>(&self) -> impl Future<Output = Result<Vec<Episode>>> + 'a {
        read_episodes(self.chapter_path.clone())
    }
}
