use std::{collections::HashSet, hash::Hash};

use iced::{
    widget::{container, text},
    Element, Length,
};

pub struct LoadingScreen<T> {
    kinds: HashSet<T>,
    kind_count: usize,
}

impl<T> LoadingScreen<T>
where
    T: PartialEq + Eq + Hash,
{
    pub fn new() -> Self {
        Self {
            kinds: HashSet::new(),
            kind_count: 0,
        }
    }

    pub fn insert(&mut self, kind: T) {
        self.kinds.insert(kind);
        self.kind_count += 1;
    }

    pub fn complete(&mut self, kind: T) {
        self.kinds.remove(&kind);
    }

    pub fn all_complete(&self) -> bool {
        self.kinds.len() == 0
    }

    pub fn title(&self) -> String {
        String::from("Loading")
    }

    pub fn view<'a, M: 'a>(&'a self) -> Element<M> {
        container(text(format!(
            "Loading ({}/{})...",
            self.kind_count - self.kinds.len(),
            self.kinds.len()
        )))
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
    }
}
