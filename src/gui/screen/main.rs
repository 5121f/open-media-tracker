/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use iced::{
    alignment,
    widget::{button, container, Column},
    Element, Length,
};

use crate::{
    gui::{
        alias::{INDENT, PADDING},
        list::list,
        ListMsg,
    },
    model::MediaHandler,
};

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
            .align_x(alignment::Horizontal::Center),
        )
        .push_maybe(
            list(media.iter().map(MediaHandler::name).collect()).map(|v| v.map(Msg::MenuButton)),
        )
        .spacing(PADDING)
        .padding(INDENT)
        .into()
}
