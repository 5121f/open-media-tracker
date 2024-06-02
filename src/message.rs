use iced::font;

use crate::{
    gui::screen::{
        ConfirmScreenMessage, ErrorScreenMessage, LoadingMessage, MainScreenMessage,
        MediaEditScreenMessage,
    },
    media_list::{MediaList, MediaListError},
};

#[derive(Debug, Clone)]
pub enum Message {
    MainScreen(MainScreenMessage),
    MediaEditScreen(MediaEditScreenMessage),
    ConfirmScreen(ConfirmScreenMessage),
    ErrorScreen(ErrorScreenMessage),
    FontLoaded(Result<(), font::Error>),
    MediaLoaded(Result<MediaList, MediaListError>),
    LoadingMessage,
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
    fn from(_: LoadingMessage) -> Self {
        Self::LoadingMessage
    }
}
