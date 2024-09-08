/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::{
    gui::screen::{
        ConfirmScreenMessage, ErrorScreenMessage, LoadingMessage, MainScreenMessage,
        MediaEditScreenMessage,
    },
    model::{error::ErrorKind, media::MediaList},
};

#[derive(Debug, Clone)]
pub enum Message {
    MainScreen(MainScreenMessage),
    MediaEditScreen(MediaEditScreenMessage),
    ConfirmScreen(ConfirmScreenMessage),
    ErrorScreen(ErrorScreenMessage),
    FontLoaded(Result<(), iced::font::Error>),
    MediaLoaded(Result<MediaList, ErrorKind>),
    Loading,
}

impl From<ConfirmScreenMessage> for Message {
    fn from(value: ConfirmScreenMessage) -> Self {
        Self::ConfirmScreen(value)
    }
}

impl From<ErrorScreenMessage> for Message {
    fn from(value: ErrorScreenMessage) -> Self {
        Self::ErrorScreen(value)
    }
}

impl From<MediaEditScreenMessage> for Message {
    fn from(value: MediaEditScreenMessage) -> Self {
        Self::MediaEditScreen(value)
    }
}

impl From<MainScreenMessage> for Message {
    fn from(value: MainScreenMessage) -> Self {
        Self::MainScreen(value)
    }
}

impl From<LoadingMessage> for Message {
    fn from(_value: LoadingMessage) -> Self {
        Self::Loading
    }
}
