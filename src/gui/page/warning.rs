/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::fmt::Display;

use cosmic::iced_widget::{column, row};
use cosmic::widget::{button, container, horizontal_space, icon, text};
use cosmic::{Element, style, theme};

use crate::gui::dialog::{DialogWithKind, HaveKind};
use crate::gui::{self, Page};

#[derive(Debug, Clone)]
pub enum Msg {
    Close,
}

pub struct WarningPage<T> {
    kind: T,
}

impl<T> WarningPage<T> {
    pub const fn new(kind: T) -> Self {
        Self { kind }
    }
}

impl<T: Display> Page for WarningPage<T> {
    type Message = Msg;

    fn view(&self) -> Element<'_, Msg> {
        let spacing = theme::spacing();

        container(
            row![
                icon(crate::gui::icon::warning()).size(25),
                column![
                    row![
                        text::title4("Warning"),
                        horizontal_space(),
                        button::icon(gui::icon::close()).on_press(Msg::Close)
                    ],
                    text(self.kind.to_string())
                ]
                .spacing(spacing.space_s),
            ]
            .spacing(spacing.space_s),
        )
        .class(style::Container::Dialog)
        .padding(spacing.space_m)
        .into()
    }
}

impl<T> HaveKind for WarningPage<T> {
    type Kind = T;

    fn kind(&self) -> &Self::Kind {
        &self.kind
    }
}

impl<T> From<T> for WarningPage<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

pub type WarningDlg<T> = DialogWithKind<WarningPage<T>>;
