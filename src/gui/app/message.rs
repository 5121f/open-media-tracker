/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use derive_more::derive::From;

use crate::gui::page::{
    ConfirmPageMsg, ErrorPageMsg, LoadingPageMsg, MainPageMsg, MediaEditPageMsg,
};
use crate::model::{ErrorKind, MaybeError, MediaList};

#[derive(Debug, Clone, From)]
pub enum Msg {
    MainScreen(MainPageMsg),
    MediaEditScreen(MediaEditPageMsg),
    ConfirmScreen(ConfirmPageMsg),
    ErrorScreen(ErrorPageMsg),
    MediaLoaded(MaybeError<MediaList, ErrorKind>),
    SelectMedia(String),
    CreateMedia,
    Loading,
}

impl From<LoadingPageMsg> for Msg {
    #[allow(clippy::match_single_binding)]
    fn from(value: LoadingPageMsg) -> Self {
        // We want to get a warning if LoadingMsg changes
        match value {
            LoadingPageMsg {} => {}
        }

        Self::Loading
    }
}
