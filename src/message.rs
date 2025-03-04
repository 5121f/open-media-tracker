/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use derive_more::derive::From;

use crate::gui::screen::{ConfirmScrnMsg, ErrorScrnMsg, LoadingMsg, MainScrnMsg, MediaEditScrnMsg};
use crate::model::{ErrorKind, MaybeError, MediaList};

#[derive(Debug, Clone, From)]
pub enum Msg {
    MainScreen(MainScrnMsg),
    MediaEditScreen(MediaEditScrnMsg),
    ConfirmScreen(ConfirmScrnMsg),
    ErrorScreen(ErrorScrnMsg),
    MediaLoaded(MaybeError<MediaList, ErrorKind>),
    Loading,
}

impl From<LoadingMsg> for Msg {
    #[allow(clippy::match_single_binding)]
    fn from(value: LoadingMsg) -> Self {
        // We want to get a warning if LoadingMsg changes
        match value {
            LoadingMsg {} => {}
        }

        Self::Loading
    }
}
