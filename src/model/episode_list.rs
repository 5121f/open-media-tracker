/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::path::Path;

use derive_more::derive::Deref;

use crate::model::{Episode, ErrorKind, Result};
use crate::read_dir;

#[derive(Deref)]
pub struct EpisodeList(Vec<Episode>);

impl EpisodeList {
    pub fn read(path: impl AsRef<Path>) -> Result<Self> {
        let media_path = path.as_ref();
        let episode_paths = read_dir(media_path)?;
        let mut episodes: Vec<_> = episode_paths
            .into_iter()
            .filter_map(|path| Episode::new(path).ok())
            .collect();
        if episodes.is_empty() {
            return Err(ErrorKind::EpisodeNotFound);
        }
        episodes.sort();
        Ok(Self(episodes))
    }
}
