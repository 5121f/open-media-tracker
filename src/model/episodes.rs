/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

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
