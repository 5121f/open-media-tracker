// SPDX-License-Identifier: MPL-2.0 OR GPL-2.0-or-later

use std::sync::Arc;

use derive_more::{Deref, From};

use crate::model::{Episode, ErrorKind, LoadedData};

#[derive(Debug, Clone, From, Deref)]
pub struct Episodes(pub LoadedData<Arc<Vec<Episode>>, ErrorKind>);

impl Episodes {
    pub fn len(&self) -> Option<usize> {
        self.0.as_option().map(|episodes| episodes.len())
    }

    pub fn get(&self, id: usize) -> Option<std::result::Result<&Episode, &ErrorKind>> {
        let res = self
            .0
            .as_opt_res()?
            .and_then(|episodes| episodes.get(id).ok_or(&ErrorKind::EpisodeNotFound));
        Some(res)
    }
}
