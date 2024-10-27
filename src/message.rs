/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::{
    gui::screen::{ConfirmScrnMsg, ErrorScrnMsg, LoadingMsg, MainScrnMsg, MediaEditScrnMsg},
    model::{ErrorKind, MediaList},
};

#[derive(Debug, Clone)]
pub enum Msg {
    MainScreen(MainScrnMsg),
    MediaEditScreen(MediaEditScrnMsg),
    ConfirmScreen(ConfirmScrnMsg),
    ErrorScreen(ErrorScrnMsg),
    MediaLoaded(Result<MediaList, ErrorKind>),
    Loading,
}

impl From<ConfirmScrnMsg> for Msg {
    fn from(value: ConfirmScrnMsg) -> Self {
        Self::ConfirmScreen(value)
    }
}

impl From<ErrorScrnMsg> for Msg {
    fn from(value: ErrorScrnMsg) -> Self {
        Self::ErrorScreen(value)
    }
}

impl From<MediaEditScrnMsg> for Msg {
    fn from(value: MediaEditScrnMsg) -> Self {
        Self::MediaEditScreen(value)
    }
}

impl From<MainScrnMsg> for Msg {
    fn from(value: MainScrnMsg) -> Self {
        Self::MainScreen(value)
    }
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
