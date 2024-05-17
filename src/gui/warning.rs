use std::fmt::Display;

use iced::{widget::text, Element};
use iced_aw::card;

#[derive(Debug, Clone)]
pub enum Message {
    Close,
}

pub fn warning_view<'a, T: Display>(kind: T) -> Element<'a, Message> {
    card(text("Warning"), text(kind.to_string()))
        .close_size(25.)
        .style(iced_aw::style::CardStyles::Warning)
        .on_close(Message::Close)
        .into()
}
