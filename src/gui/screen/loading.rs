use std::{collections::HashSet, hash::Hash};

use iced::{
    widget::{container, text},
    Element, Length,
};

use crate::gui::IDialog;

pub struct Message;

pub struct LoadingScreen<T> {
    kinds: HashSet<T>,
}

impl<T> LoadingScreen<T>
where
    T: PartialEq + Eq + Hash,
{
    pub fn new() -> Self {
        Self {
            kinds: HashSet::new(),
        }
    }

    pub fn insert(&mut self, kind: T) {
        self.kinds.insert(kind);
    }

    pub fn complete(&mut self, kind: T) {
        self.kinds.remove(&kind);
    }

    pub fn all_complete(&self) -> bool {
        self.kinds.len() == 0
    }
}

impl<T> IDialog for LoadingScreen<T> {
    type Message = Message;

    fn title(&self) -> String {
        String::from("Loading")
    }

    fn view(&self) -> Element<Self::Message> {
        container(text("Loading..."))
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
