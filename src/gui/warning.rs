use std::fmt::Display;

use iced::{widget::text, Element};
use iced_aw::card;

use super::{dialog::IHaveKind, IDialog};

#[derive(Debug, Clone)]
pub enum Message {
    Close,
}

pub struct WarningScreen<T> {
    kind: T,
}

impl<T> WarningScreen<T> {
    pub fn new(kind: T) -> Self {
        Self { kind }
    }
}

impl<T: Display> IDialog for WarningScreen<T> {
    type Message = Message;

    fn title(&self) -> String {
        String::from("Warning")
    }

    fn view(&self) -> Element<Self::Message> {
        card(text(self.title()), text(&self.kind))
            .close_size(25.)
            .style(iced_aw::style::CardStyles::Warning)
            .on_close(Message::Close)
            .into()
    }
}

impl<T> IHaveKind for WarningScreen<T> {
    type Kind = T;

    fn kind(&self) -> &Self::Kind {
        &self.kind
    }
}
