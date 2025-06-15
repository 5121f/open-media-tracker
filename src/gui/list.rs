/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use cosmic::Element;
use cosmic::iced::Length;
use cosmic::widget::{button, column, container, scrollable};

use crate::gui::utils::INDENT;

#[derive(Debug, Clone)]
pub enum Message {
    Enter(usize),
}

pub fn list(buttons: Vec<&str>) -> Option<Element<Message>> {
    if buttons.is_empty() {
        return None;
    }

    let view = container(
        scrollable(
            column::Column::with_children(
                buttons
                    .into_iter()
                    .enumerate()
                    .map(|(id, text)| {
                        button::standard(text)
                            .width(Length::Fill)
                            .on_press(Message::Enter(id))
                    })
                    .map(Into::into),
            )
            .padding(INDENT),
        )
        .height(Length::Fill),
    )
    .width(Length::Fill);

    Some(view.into())
}
