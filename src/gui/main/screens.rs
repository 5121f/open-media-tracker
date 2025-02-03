/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use derive_more::derive::From;
use iced::Element;

use crate::{
    gui::screen::{main_screen_view, MediaEditScrn},
    message::Msg,
    model::{MediaHandler, MediaList},
};

#[derive(Default, From)]
pub enum Screens {
    #[default]
    Main,
    MediaChange(MediaEditScrn),
}

impl Screens {
    pub fn view<'a>(&'a self, media: &'a MediaList) -> Element<'a, Msg> {
        match self {
            Self::Main => main_screen_view(media).map(Into::into),
            Self::MediaChange(screen) => screen.view(media).map(Into::into),
        }
    }

    pub fn change_media(media: &[MediaHandler], id: usize) -> Self {
        MediaEditScrn::new(media, id).into()
    }

    pub fn title(&self, media: &[MediaHandler]) -> Option<String> {
        let title = match self {
            Self::Main => return None,
            Self::MediaChange(media_edit_scrn) => media_edit_scrn.title(media),
        };
        Some(title)
    }
}
