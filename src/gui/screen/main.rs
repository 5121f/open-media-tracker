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
    media::Media,
};

#[derive(Debug, Clone)]
pub enum Message {
    AddMedia,
    MenuButton(ListMessage),
}

pub fn main_screen_view(media: &[Media]) -> Element<Message> {
    let mut layout = Column::new().spacing(PADDING).padding(INDENT);

    let add_media_button = container(
        button("Add media")
            .style(theme::Button::Positive)
            .on_press(Message::AddMedia),
    )
    .width(Length::Fill)
    .center_x();

    let media_list = list(media.into_iter().map(Media::name).collect())
        .map(|list| list.map(Message::MenuButton));

    layout = layout.push(add_media_button);
    layout = layout.push_maybe(media_list);

    layout.into()
}
