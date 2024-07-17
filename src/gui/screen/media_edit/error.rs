/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::{episode::EpisodeListError, media::MediaError, utils::OpenError};

#[derive(Debug, Clone, thiserror::Error)]
pub enum Error {
    #[error("Eisode not found")]
    EpisodeNotFound,
    #[error(transparent)]
    EpisodeList(#[from] EpisodeListError),
    #[error(transparent)]
    Media(#[from] MediaError),
    #[error(transparent)]
    Open(#[from] OpenError),
}

pub(crate) type Result<T> = std::result::Result<T, Error>;
