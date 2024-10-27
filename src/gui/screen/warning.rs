/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::fmt::Display;

use iced::{widget::text, Element};
use iced_aw::card;

use crate::gui::{
    dialog::{DialogWithKind, HaveKind},
    Screen,
};

#[derive(Debug, Clone)]
pub enum Message {
    Close,
}

pub struct WarningScrn<T> {
    kind: T,
}

impl<T> WarningScrn<T> {
    pub fn new(kind: T) -> Self {
        Self { kind }
    }
}

impl<T: Display> Screen for WarningScrn<T> {
    type Message = Message;

    fn title(&self) -> String {
        String::from("Warning")
    }

    fn view(&self) -> Element<Message> {
        card(text(self.title()), text(self.kind.to_string()))
            .close_size(25.)
            .style(iced_aw::style::card::warning)
            .on_close(Message::Close)
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
