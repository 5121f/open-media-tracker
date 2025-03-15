/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use iced::widget::{Column, button as iced_button, container};
use iced::{Alignment, Element, Length};

use crate::gui::button::button_styled;
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
            container(button_styled("Add media", iced_button::success).on_press(Msg::AddMedia))
                .width(Length::Fill)
                .align_x(Alignment::Center),
        )
        .push_maybe(
            list(media.iter().map(MediaHandler::name).collect()).map(|v| v.map(Msg::MenuButton)),
        )
        .spacing(LONG_INDENT)
        .padding(LONG_INDENT)
        .into()
}
