use std::{ops::Deref, path::Path};

use crate::{episode::Episode, error::ErrorKind, utils};

pub struct Episodes(Vec<Episode>);

impl Episodes {
    pub fn read(path: impl AsRef<Path>) -> Result<Self, ErrorKind> {
        let series_path = path.as_ref();
        let episode_paths = utils::read_dir(series_path)?;
        let mut episodes: Vec<_> = episode_paths
            .into_iter()
            .flat_map(|path| Episode::new(path).ok())
            .collect();
        episodes.sort();
        Ok(Self(episodes))
    }
}

impl Deref for Episodes {
    type Target = Vec<Episode>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
