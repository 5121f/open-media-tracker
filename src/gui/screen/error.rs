/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use iced::{
    theme,
    widget::{button, column, container, horizontal_space, row, text, Space},
    Element, Length,
};
use iced_aw::card;

use crate::{
    error::Error,
    gui::{utils::INDENT, IDialog},
};

#[derive(Debug, Clone)]
pub enum Message {
    Ok { critical: bool },
}

pub struct ErrorScreen {
    error: Error,
}

impl ErrorScreen {
    pub fn new(error: Error) -> Self {
        Self { error }
    }
}

impl IDialog for ErrorScreen {
    type Message = Message;

    fn view(&self) -> Element<Message> {
        let ok_button_style = if self.error.critical {
            theme::Button::Destructive
        } else {
            theme::Button::Primary
        };

        container(row![
            Space::with_width(Length::FillPortion(1)),
            card(
                text(self.title()),
                column![
                    text(&self.error),
                    row![
                        horizontal_space(),
                        button("Ok").style(ok_button_style).on_press(Message::Ok {
                            critical: self.error.critical
                        })
                    ],
                ]
                .spacing(INDENT)
            )
            .style(iced_aw::style::card::CardStyles::Danger)
            .width(Length::FillPortion(15)),
            Space::with_width(Length::FillPortion(1))
        ])
        .center_y()
        .into()
    }

    fn title(&self) -> String {
        String::from("Error")
    }
}
