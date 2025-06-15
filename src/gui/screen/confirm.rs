/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::fmt::Display;

use cosmic::Element;
use cosmic::iced::{Alignment, Length};
use cosmic::iced_widget::{column, row};
use cosmic::widget::{Space, button, container, horizontal_space, text};

use crate::gui::Screen;
use crate::gui::dialog::{DialogWithKind, HaveKind};
use crate::gui::utils::INDENT;

#[derive(Debug, Clone)]
pub enum Msg {
    Confirm,
    Cancel,
}

pub struct ConfirmScrn<T> {
    kind: T,
}

impl<T> ConfirmScrn<T> {
    pub const fn new(kind: T) -> Self {
        Self { kind }
    }
}

impl<T: Display> Screen for ConfirmScrn<T> {
    type Message = Msg;

    fn title(&self) -> String {
        String::from("Confirm")
    }

    fn view(&self) -> Element<Msg> {
        container(row![
            Space::with_width(Length::FillPortion(1)),
            container(
                column![
                    text(self.kind.to_string()),
                    row![
                        button::destructive("Cancel").on_press(Msg::Cancel),
                        horizontal_space(),
                        button::suggested("Confirm").on_press(Msg::Confirm)
                    ]
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
}

impl<T> HaveKind for ConfirmScrn<T> {
    type Kind = T;

    fn kind(&self) -> &Self::Kind {
        &self.kind
    }
}

impl<T> From<T> for ConfirmScrn<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

pub type ConfirmDlg<T> = DialogWithKind<ConfirmScrn<T>>;
