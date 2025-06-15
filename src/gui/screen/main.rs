/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use cosmic::Element;
use cosmic::iced::{Alignment, Length};
use cosmic::widget::{Column, button, container};

use crate::gui::utils::LONG_INDENT;
use crate::gui::{ListMsg, list};
use crate::model::MediaHandler;

#[derive(Debug, Clone)]
pub enum Msg {
    AddMedia,
    MenuButton(ListMsg),
}

pub fn main_screen_view(media: &[MediaHandler]) -> Element<Msg> {
    Column::new()
        .push(
            container(button::suggested("Add media").on_press(Msg::AddMedia))
                .width(Length::Fill)
                .align_x(Alignment::Center),
        )
        .push_maybe(
            list(media.iter().map(MediaHandler::name).collect()).map(|v| v.map(Msg::MenuButton)),
        )
        .spacing(LONG_INDENT)
        .padding(LONG_INDENT)
        .height(Length::Fill)
        .into()
}
