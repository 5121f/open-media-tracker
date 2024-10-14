/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::fmt::Display;

use iced::{
    alignment::Vertical,
    widget::{button, column, container, horizontal_space, row, text, Space},
    Element, Length,
};
use iced_aw::card;

use crate::gui::{alias::INDENT, dialog::HaveKind, Dialog};

#[derive(Debug, Clone)]
pub enum Msg {
    Confirm,
    Cancel,
}

pub struct ConfirmScrn<T> {
    kind: T,
}

impl<T> ConfirmScrn<T> {
    pub fn new(kind: T) -> Self {
        Self { kind }
    }
}

impl<T: Display> Dialog for ConfirmScrn<T> {
    type Message = Msg;

    fn title(&self) -> String {
        String::from("Confirm")
    }

    fn view(&self) -> Element<Msg> {
        container(row![
            Space::with_width(Length::FillPortion(1)),
            card(
                text(self.title()),
                column![
                    text(self.kind.to_string()),
                    row![
                        button("Cancel").style(button::danger).on_press(Msg::Cancel),
                        horizontal_space(),
                        button("Confirm")
                            .style(button::success)
                            .on_press(Msg::Confirm)
                    ]
                ]
                .spacing(INDENT)
            )
            .close_size(25.)
            .width(Length::FillPortion(15))
            .on_close(Msg::Cancel),
            Space::with_width(Length::FillPortion(1))
        ])
        .height(Length::Fill)
        .align_y(Vertical::Center)
        .into()
    }
}

impl<T> HaveKind for ConfirmScrn<T> {
    type Kind = T;

    fn kind(&self) -> &Self::Kind {
        &self.kind
    }
}
