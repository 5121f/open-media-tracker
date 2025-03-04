/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use iced::{
    Alignment, Element, Length,
    widget::{Column, button, container},
};

use crate::gui::ListMsg;
use crate::gui::list;
use crate::gui::utils::{INDENT, LONG_INDENT};
use crate::model::MediaHandler;

#[derive(Debug, Clone)]
pub enum Msg {
    AddMedia,
    MenuButton(ListMsg),
}

pub fn main_screen_view(media: &[MediaHandler]) -> Element<Msg> {
    Column::new()
        .push(
            container(
                button("Add media")
                    .style(button::success)
                    .on_press(Msg::AddMedia),
            )
            .width(Length::Fill)
            .align_x(Alignment::Center),
        )
        .push_maybe(
            list(media.iter().map(MediaHandler::name).collect()).map(|v| v.map(Msg::MenuButton)),
        )
        .spacing(LONG_INDENT)
        .padding(INDENT)
        .into()
}
