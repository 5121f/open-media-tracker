use std::fmt::Display;

use iced::{
    theme,
    widget::{button, column, horizontal_space, row, text, vertical_space, Space},
    Element,
};
use iced_aw::card;

#[derive(Debug, Clone)]
pub enum Message {
    Confirm,
    Cancel,
}

pub struct ConfirmScreen<T> {
    kind: T,
}

impl<T: Display> ConfirmScreen<T> {
    pub fn new(kind: T) -> Self {
        Self { kind }
    }

    pub fn view(&self) -> Element<Message> {
        row![
            Space::with_width(100),
            column![
                vertical_space(),
                card(
                    "Confirm dialog",
                    column![
                        row![horizontal_space(), text(&self.kind), horizontal_space()],
                        row![
                            button("Cancel")
                                .style(theme::Button::Destructive)
                                .on_press(Message::Cancel),
                            horizontal_space(),
                            button("Confirm")
                                .style(theme::Button::Positive)
                                .on_press(Message::Confirm)
                        ]
                    ]
                ),
                vertical_space()
            ],
            Space::with_width(100)
        ]
        .into()
    }

    pub fn kind(&self) -> &T {
        &self.kind
    }
}
