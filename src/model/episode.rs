/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::borrow::Cow;
use std::path::{Path, PathBuf};

use mime_guess::mime;

use crate::model::ErrorKind;
use crate::utils::read_dir;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Episode {
    path: PathBuf,
}

impl Episode {
    pub fn new(path: impl Into<PathBuf>) -> Result<Self, NotAMediaFileError> {
        let path = path.into();

        if !is_media_file(&path) {
            return Err(NotAMediaFileError);
        }

        Ok(Self { path })
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn name(&self) -> Cow<'_, str> {
        self.path.file_stem().unwrap_or_default().to_string_lossy()
    }
}

fn is_media_file(path: impl AsRef<Path>) -> bool {
    let mime = mime_guess::from_path(path);
    let Some(mime) = mime.first() else {
        return false;
    };
    let mtype = mime.type_();
    mtype == mime::VIDEO || mtype == mime::AUDIO
}

pub async fn read_episodes(path: impl AsRef<Path>) -> Result<Vec<Episode>, ErrorKind> {
    let media_path = path.as_ref();
    let episode_paths = read_dir(media_path).await?;
    let mut episodes: Vec<_> = episode_paths
        .into_iter()
        .filter_map(|path| Episode::new(path).ok())
        .collect();
    if episodes.is_empty() {
        return Err(ErrorKind::EpisodeNotFound);
    }
    episodes.sort();
    Ok(episodes)
}

#[derive(Debug, Clone, thiserror::Error)]
#[error("Episode must be media file")]
pub struct NotAMediaFileError;
