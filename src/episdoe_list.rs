use std::{ops::Deref, path::Path};

use crate::{episode::Episode, error::ErrorKind, utils};

pub struct EpisodeList(Vec<Episode>);

impl EpisodeList {
    pub fn read(path: impl AsRef<Path>) -> Result<Self, ErrorKind> {
        let media_path = path.as_ref();
        let episode_paths = utils::read_dir(media_path)?;
        let mut episodes: Vec<_> = episode_paths
            .into_iter()
            .flat_map(|path| Episode::new(path).ok())
            .collect();
        if episodes.is_empty() {
            return Err(ErrorKind::EpisodesDidNotFound);
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
