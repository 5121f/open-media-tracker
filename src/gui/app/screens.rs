/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use cosmic::Element;
use derive_more::derive::From;

use crate::gui::Screen;
use crate::gui::app::Msg;
use crate::gui::screen::{ErrorScrn, MainScrn, MediaEditScrn};
use crate::model::{Error, MediaHandler, MediaList};

#[derive(From)]
pub enum Screens {
    Main(MainScrn),
    MediaChange(MediaEditScrn),
    Error(ErrorScrn),
}

impl Screens {
    pub fn view<'a>(&'a self, media: &'a MediaList) -> Element<'a, Msg> {
        match self {
            Self::Main(screen) => screen.view().map(Into::into),
            Self::MediaChange(screen) => screen.view(media).map(Into::into),
            Self::Error(screen) => screen.view_into(),
        }
    }

    pub fn change_media(media: &[MediaHandler], id: usize) -> Self {
        MediaEditScrn::new(media, id).into()
    }

    pub fn error(error: impl Into<Error>) -> Self {
        Self::Error(ErrorScrn::from(error.into()))
    }
}

impl Default for Screens {
    fn default() -> Self {
        Self::Main(MainScrn::default())
    }
}
