/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::fmt::Display;

use cosmic::iced_widget::{column, row};
use cosmic::widget::{button, container, horizontal_space, text};
use cosmic::{Element, font, style, theme};

use crate::gui::dialog::{DialogWithKind, HaveKind};
use crate::gui::{self, Screen};

#[derive(Debug, Clone)]
pub enum Message {
    Close,
}

pub struct WarningScrn<T> {
    kind: T,
}

impl<T> WarningScrn<T> {
    pub const fn new(kind: T) -> Self {
        Self { kind }
    }
}

impl<T: Display> Screen for WarningScrn<T> {
    type Message = Message;

    fn view(&self) -> Element<Message> {
        let spacing = theme::spacing();

        container(
            column![
                row![
                    text::text("Warning").size(17).font(font::bold()),
                    horizontal_space(),
                    button::icon(gui::icon::close()).on_press(Message::Close)
                ],
                text(self.kind.to_string())
            ]
            .spacing(spacing.space_s),
        )
        .class(style::Container::Dialog)
        .padding(spacing.space_m)
        .into()
    }
}

impl<T> HaveKind for WarningScrn<T> {
    type Kind = T;

    fn kind(&self) -> &Self::Kind {
        &self.kind
    }
}

impl<T> From<T> for WarningScrn<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

pub type WarningDlg<T> = DialogWithKind<WarningScrn<T>>;
