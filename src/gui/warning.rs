use std::fmt::Display;

use iced::{widget::text, Element};
use iced_aw::card;

use crate::gui::IDialig;

#[derive(Debug, Clone)]
pub enum Message {
    Close,
}

pub struct WarningPopUp<T> {
    kind: T,
}

impl<T> WarningPopUp<T> {
    pub fn new(kind: T) -> Self {
        Self { kind }
    }

    pub fn kind(&self) -> &T {
        &self.kind
    }
}

impl<T: Display> IDialig for WarningPopUp<T> {
    type Message = Message;

    fn view(&self) -> Element<Message> {
        card("Warning", text(self.kind.to_string()))
            .close_size(25.)
            .style(iced_aw::style::CardStyles::Warning)
            .on_close(Message::Close)
            .into()
    }

    fn title(&self) -> String {
        todo!()
    }
}
