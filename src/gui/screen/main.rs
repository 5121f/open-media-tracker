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
        list::list,
        utils::{INDENT, PADDING},
        ListMessage,
    },
    model::MediaHandler,
};

#[derive(Debug, Clone)]
pub enum Message {
    AddMedia,
    MenuButton(ListMessage),
}

pub fn main_screen_view(media: &[MediaHandler]) -> Element<Message> {
    Column::new()
        .push(
            container(
                button("Add media")
                    .style(button::success)
                    .on_press(Message::AddMedia),
            )
            .width(Length::Fill)
            .align_x(alignment::Horizontal::Center),
        )
        .push_maybe(
            list(media.iter().map(MediaHandler::name).collect())
                .map(|v| v.map(Message::MenuButton)),
        )
        .spacing(PADDING)
        .padding(INDENT)
        .into()
}
