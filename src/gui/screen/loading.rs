use std::fmt::Display;

use iced::{
    widget::{container, text},
    Element, Length,
};

pub struct LoadingScreen<T> {
    message: T,
}

impl<T: Display> LoadingScreen<T> {
    pub fn new(message: T) -> Self {
        Self { message }
    }

    pub fn view<'a, M: 'a>(&'a self) -> Element<M> {
        container(text(&self.message))
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    pub fn title(&self) -> String {
        String::from("Loading")
    }
}
