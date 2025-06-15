/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use cosmic::Element;
use derive_more::derive::From;

use crate::gui::screen::{MainScrn, MediaEditScrn};
use crate::message::Msg;
use crate::model::{MediaHandler, MediaList};

#[derive(From)]
pub enum Screens {
    Main(MainScrn),
    MediaChange(MediaEditScrn),
}

impl Screens {
    pub fn view<'a>(&'a self, media: &'a MediaList) -> Element<'a, Msg> {
        match self {
            Self::Main(screen) => screen.view().map(Into::into),
            Self::MediaChange(screen) => screen.view(media).map(Into::into),
        }
    }

    pub fn change_media(media: &[MediaHandler], id: usize) -> Self {
        MediaEditScrn::new(media, id).into()
    }
}

impl Default for Screens {
    fn default() -> Self {
        Self::Main(MainScrn::default())
    }
}
