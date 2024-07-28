/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

pub mod loading;

pub use loading::LoadingDialog;

use std::ops::{Deref, DerefMut};

use iced::Element;

pub trait IDialog {
    type Message;

    fn title(&self) -> String;

    fn view(&self) -> Element<Self::Message>;

    fn view_map<'a, B: 'a>(&'a self, f: impl Fn(Self::Message) -> B + 'a) -> Element<'a, B> {
        self.view().map(f)
    }

    fn view_into<'a, M>(&'a self) -> Element<'a, M>
    where
        M: From<Self::Message> + 'a,
    {
        self.view_map(Into::into)
    }
}

pub trait IHaveKind {
    type Kind;

    fn kind(&self) -> &Self::Kind;
}

pub struct Dialog<T>(Option<T>);

impl<T> Dialog<T> {
    pub fn new(dialog: T) -> Self {
        Self(Some(dialog))
    }

    pub fn closed() -> Self {
        Self(None)
    }

    pub fn close(&mut self) {
        self.0 = None;
    }
}

impl<T: IDialog> Dialog<T> {
    pub fn title(&self) -> Option<String> {
        self.0.as_ref().map(IDialog::title)
    }

    pub fn view(&self) -> Option<Element<T::Message>> {
        self.0.as_ref().map(IDialog::view)
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

impl<T: IHaveKind> Dialog<T> {
    pub fn kind(&self) -> Option<&T::Kind> {
        self.0.as_ref().map(T::kind)
    }
}

impl<T> Deref for Dialog<T> {
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Dialog<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> Default for Dialog<T> {
    fn default() -> Self {
        Self::closed()
    }
}
