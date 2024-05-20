use iced::font;

use crate::{
    error::ErrorKind,
    gui::screen::{
        ConfirmScreenMessage, ErrorScreenMessage, MainScreenMessage, SeriesEditScreenMessage,
    },
    media::Media,
};

#[derive(Debug, Clone)]
pub enum Message {
    MainScreen(MainScreenMessage),
    SeriesEditScreen(SeriesEditScreenMessage),
    ConfirmScreen(ConfirmScreenMessage),
    ErrorScreen(ErrorScreenMessage),
    FontLoaded(Result<(), font::Error>),
    MediaLoaded(Result<Media, ErrorKind>),
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

impl From<SeriesEditScreenMessage> for Message {
    fn from(value: SeriesEditScreenMessage) -> Self {
        Self::SeriesEditScreen(value)
    }
}

impl From<MainScreenMessage> for Message {
    fn from(value: MainScreenMessage) -> Self {
        Self::MainScreen(value)
    }
}
