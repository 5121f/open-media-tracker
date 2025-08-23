// SPDX-License-Identifier: MPL-2.0 OR GPL-2.0-or-later

use cosmic::{Element, Task};
use derive_more::derive::From;

use crate::gui::Page;
use crate::gui::app::Msg;
use crate::gui::page::{ErrorPage, MainPage, MediaEditPage};
use crate::model::{Error, MediaList, MediaListRef};

#[derive(From)]
pub enum Screens {
    Main(MainPage),
    MediaChange(MediaEditPage),
    Error(ErrorPage),
}

impl Screens {
    pub fn view<'a>(&'a self, media: &'a MediaList) -> Element<'a, Msg> {
        match self {
            Self::Main(screen) => screen.view_into(),
            Self::MediaChange(screen) => screen.view(media).map(Into::into),
            Self::Error(screen) => screen.view_into(),
        }
    }

    pub fn change_media(media: MediaListRef, id: usize) -> (Self, Task<Msg>) {
        let (screen, task) = MediaEditPage::new(media, id);
        (Self::MediaChange(screen), task.map(Msg::MediaEditScreen))
    }

    pub fn error(error: impl Into<Error>) -> Self {
        Self::Error(ErrorPage::from(error.into()))
    }
}

impl Default for Screens {
    fn default() -> Self {
        Self::Main(MainPage::default())
    }
}
