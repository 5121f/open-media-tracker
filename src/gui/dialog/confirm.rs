/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::ops::{Deref, DerefMut};

use crate::gui::screen::ConfirmScrn;

use super::Dialog;

pub struct ConfirmDlg<T>(Dialog<ConfirmScrn<T>>);

impl<T> ConfirmDlg<T> {
    pub fn from_kind(kind: T) -> Self {
        let screen = ConfirmScrn::new(kind);
        let closable = Dialog::new(screen);
        Self(closable)
    }

    pub fn closed() -> Self {
        Self(Dialog::closed())
    }
}

impl<T> Deref for ConfirmDlg<T> {
    type Target = Dialog<ConfirmScrn<T>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for ConfirmDlg<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
