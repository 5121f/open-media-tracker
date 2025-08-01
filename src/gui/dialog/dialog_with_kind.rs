/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::ops::{Deref, DerefMut};

use crate::gui::dialog::HaveKind;
use crate::gui::{Dialog, Page};

pub struct DialogWithKind<S>(Dialog<S>)
where
    S: Page + HaveKind;

impl<S> DialogWithKind<S>
where
    S: Page + HaveKind,
{
    pub const fn closed() -> Self {
        Self(Dialog::closed())
    }
}

impl<S> DialogWithKind<S>
where
    S: Page + HaveKind + From<S::Kind>,
{
    pub fn from_kind(kind: S::Kind) -> Self {
        let screen: S = kind.into();
        screen.into()
    }
}

impl<S> Default for DialogWithKind<S>
where
    S: Page + HaveKind,
{
    fn default() -> Self {
        Self(Dialog::default())
    }
}

impl<S> From<S> for Dialog<S>
where
    S: Page + HaveKind,
{
    fn from(value: S) -> Self {
        Self::new(value)
    }
}

impl<S> From<Dialog<S>> for DialogWithKind<S>
where
    S: Page + HaveKind,
{
    fn from(value: Dialog<S>) -> Self {
        Self(value)
    }
}

impl<S> From<S> for DialogWithKind<S>
where
    S: Page + HaveKind,
{
    fn from(value: S) -> Self {
        let dialog: Dialog<_> = value.into();
        dialog.into()
    }
}

impl<S> Deref for DialogWithKind<S>
where
    S: Page + HaveKind,
{
    type Target = Dialog<S>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<S> DerefMut for DialogWithKind<S>
where
    S: Page + HaveKind,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
