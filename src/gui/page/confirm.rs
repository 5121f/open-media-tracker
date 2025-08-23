// SPDX-License-Identifier: MPL-2.0 OR GPL-2.0-or-later

use std::fmt::Display;

use cosmic::Element;
use cosmic::widget::{Dialog, button};

use crate::gui::Page;
use crate::gui::dialog::{DialogWithKind, HaveKind};

#[derive(Debug, Clone)]
pub enum Msg {
    Confirm,
    Cancel,
}

pub struct ConfirmPage<T> {
    kind: T,
}

impl<T> ConfirmPage<T> {
    pub const fn new(kind: T) -> Self {
        Self { kind }
    }
}

impl<T: Display> Page for ConfirmPage<T> {
    type Message = Msg;

    fn view(&self) -> Element<'_, Msg> {
        Dialog::new()
            .title("Delte media")
            .body(self.kind.to_string())
            .primary_action(button::suggested("Confirm").on_press(Msg::Confirm))
            .secondary_action(button::destructive("Cancel").on_press(Msg::Cancel))
            .into()
    }
}

impl<T> HaveKind for ConfirmPage<T> {
    type Kind = T;

    fn kind(&self) -> &Self::Kind {
        &self.kind
    }
}

impl<T> From<T> for ConfirmPage<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

pub type ConfirmDlg<T> = DialogWithKind<ConfirmPage<T>>;
