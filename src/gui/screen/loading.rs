use std::{collections::HashSet, hash::Hash};

use iced::{
    widget::{container, text},
    Element, Length,
};

use crate::gui::IDialog;

pub struct LoadingScreen<T> {
    kinds: HashSet<T>,
    max: usize,
}

impl<T> LoadingScreen<T>
where
    T: PartialEq + Eq + Hash,
{
    pub fn new() -> Self {
        Self {
            kinds: HashSet::new(),
            max: 0,
        }
    }

    pub fn insert(&mut self, kind: T) {
        self.kinds.insert(kind);
        self.max += 1;
    }
}

impl<T> IDialog for LoadingScreen<T> {
    type Message = ();

    fn title(&self) -> String {
        String::from("Loading")
    }

    fn view(&self) -> Element<Self::Message> {
        container(text(format!(
            "Loading ({}/{})...",
            self.max - self.kinds.len(),
            self.kinds.len()
        )))
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
    }
}

impl<T> From<HashSet<T>> for LoadingScreen<T> {
    fn from(value: HashSet<T>) -> Self {
        let max = value.len();
        Self { kinds: value, max }
    }
}
