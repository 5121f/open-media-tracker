use std::ops::{Deref, DerefMut};

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
