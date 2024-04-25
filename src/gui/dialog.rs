use std::ops::{Deref, DerefMut};

use iced::Element;

pub trait IDialig {
    type Message;

    fn title(&self) -> String;
    fn view(&self) -> Element<Self::Message>;
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

impl<T: IDialig> Dialog<T> {
    pub fn title(&self) -> Option<String> {
        Some(self.0.as_ref()?.title())
    }

    pub fn view(&self) -> Option<Element<T::Message>> {
        Some(self.0.as_ref()?.view())
    }

    pub fn view_map<'a, B: 'a>(
        &'a self,
        f: impl Fn(T::Message) -> B + 'a,
    ) -> Option<Element<'a, B>> {
        Some(self.view()?.map(f))
    }

    pub fn view_into<'a, M>(&'a self) -> Option<Element<'a, M>>
    where
        M: From<T::Message> + 'a,
    {
        Some(self.view_map(Into::into)?)
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
