/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::{ops::Deref, path::Path};

use crate::{
    model::{
        episode::Episode,
        error::{ErrorKind, Result},
    },
    utils,
};

pub struct EpisodeList(Vec<Episode>);

impl EpisodeList {
    pub fn read(path: impl AsRef<Path>) -> Result<Self> {
        let media_path = path.as_ref();
        let episode_paths = utils::read_dir(media_path)?;
        let mut episodes: Vec<_> = episode_paths
            .into_iter()
            .flat_map(|path| Episode::new(path).ok())
            .collect();
        if episodes.is_empty() {
            return Err(ErrorKind::EpisodeNotFound);
        }
        episodes.sort();
        Ok(Self(episodes))
    }
}

impl Deref for EpisodeList {
    type Target = Vec<Episode>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
