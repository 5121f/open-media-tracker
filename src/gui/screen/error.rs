/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use iced::{
    widget::{button, column, container, horizontal_space, row, text, Space},
    Alignment, Element, Length,
};
use iced_aw::card;

use crate::{
    gui::{alias::INDENT, Dialog},
    model::Error,
};

#[derive(Debug, Clone)]
pub enum Msg {
    Ok { critical: bool },
}

pub struct ErrorScrn {
    error: Error,
}

impl ErrorScrn {
    pub fn new(error: Error) -> Self {
        Self { error }
    }
}

impl Dialog for ErrorScrn {
    type Message = Msg;

    fn view(&self) -> Element<Msg> {
        let ok_button_style = if self.error.critical {
            button::danger
        } else {
            button::success
        };

        container(row![
            Space::with_width(Length::FillPortion(1)),
            card(
                text(self.title()),
                column![
                    text(self.error.to_string()),
                    row![
                        horizontal_space(),
                        button("Ok").style(ok_button_style).on_press(Msg::Ok {
                            critical: self.error.critical
                        })
                    ],
                ]
                .spacing(INDENT)
            )
            .style(iced_aw::style::card::danger)
            .width(Length::FillPortion(15)),
            Space::with_width(Length::FillPortion(1))
        ])
        .height(Length::Fill)
        .align_y(Alignment::Center)
        .into()
    }

    fn title(&self) -> String {
        String::from("Error")
    }
}
