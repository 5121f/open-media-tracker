/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use cosmic::Element;
use cosmic::iced::{Alignment, Length};
use cosmic::iced_widget::{column, row};
use cosmic::widget::{Space, button, container, horizontal_space, text};

use crate::gui::Screen;
use crate::gui::utils::INDENT;
use crate::model::Error;

#[derive(Debug, Clone)]
pub enum Msg {
    Ok { critical: bool },
}

impl Msg {
    const fn ok(critical: bool) -> Self {
        Self::Ok { critical }
    }
}

pub struct ErrorScrn {
    error: Error,
}

// impl ErrorScrn {
//     pub const fn new(error: Error) -> Self {
//         Self { error }
//     }
// }

impl Screen for ErrorScrn {
    type Message = Msg;

    fn view(&self) -> Element<Msg> {
        container(row![
            Space::with_width(Length::FillPortion(1)),
            container(
                column![
                    text(self.error.to_string()),
                    row![
                        horizontal_space(),
                        if self.error.critical {
                            button::destructive("Ok")
                        } else {
                            button::suggested("Ok")
                        }
                        .on_press(Msg::ok(self.error.critical))
                    ],
                ]
                .spacing(INDENT)
            )
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

impl From<Error> for ErrorScrn {
    fn from(value: Error) -> Self {
        Self { error: value }
    }
}
