/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use iced::{
    theme,
    widget::{button, container, Column},
    Element, Length,
};

use crate::{
    gui::{
        list::list,
        utils::{INDENT, PADDING},
        ListMessage,
    },
    model::media::Media,
};

#[derive(Debug, Clone)]
pub enum Message {
    AddMedia,
    MenuButton(ListMessage),
}

pub fn main_screen_view(media: &[Media]) -> Element<Message> {
    Column::new()
        .push(
            container(
                button("Add media")
                    .style(theme::Button::Positive)
                    .on_press(Message::AddMedia),
            )
            .width(Length::Fill)
            .center_x(),
        )
        .push_maybe(
            list(media.into_iter().map(Media::name).collect()).map(|v| v.map(Message::MenuButton)),
        )
        .spacing(PADDING)
        .padding(INDENT)
        .into()
}
