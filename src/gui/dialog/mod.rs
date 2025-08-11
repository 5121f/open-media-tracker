/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

mod dialog_with_kind;
mod have_kind;

use derive_more::{Deref, DerefMut};
pub use dialog_with_kind::DialogWithKind;
pub use have_kind::HaveKind;

use cosmic::Element;

use crate::gui::Page;

#[derive(Deref, DerefMut)]
pub struct Dialog<T>(Option<T>);

impl<T> Dialog<T> {
    pub const fn new(dialog: T) -> Self {
        Self(Some(dialog))
    }

    pub const fn closed() -> Self {
        Self(None)
    }

    pub fn close(&mut self) {
        self.0 = None;
    }
}

impl<T: Page> Dialog<T> {
    pub fn view(&self) -> Option<Element<T::Message>> {
        self.0.as_ref().map(Page::view)
    }

    pub fn view_map<'a, B: 'a>(
        &'a self,
        f: impl Fn(T::Message) -> B + 'a,
    ) -> Option<Element<'a, B>> {
        self.view().map(|d| d.map(f))
    }

    pub fn view_into<'a, M>(&'a self) -> Option<Element<'a, M>>
    where
        M: From<T::Message> + 'a,
    {
        self.view_map(Into::into)
    }
}

impl<T: HaveKind> Dialog<T> {
    pub fn kind(&self) -> Option<&T::Kind> {
        self.0.as_ref().map(T::kind)
    }
}

impl<T> Default for Dialog<T> {
    fn default() -> Self {
        Self::closed()
    }
}
