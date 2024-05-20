use std::fmt::Display;

use iced::{
    theme,
    widget::{button, column, container, horizontal_space, row, text, Space},
    Element, Length,
};
use iced_aw::card;

use crate::gui::{utils::DEFAULT_INDENT, IDialog};

#[derive(Debug, Clone)]
pub enum Message {
    Confirm,
    Cancel,
}

pub struct ConfirmScreen<T> {
    kind: T,
}

impl<T> ConfirmScreen<T> {
    pub fn new(kind: T) -> Self {
        Self { kind }
    }

    pub fn kind(self) -> T {
        self.kind
    }
}

impl<T: Display> IDialog for ConfirmScreen<T> {
    type Message = Message;

    fn title(&self) -> String {
        String::from("Confirm")
    }

    fn view(&self) -> Element<Message> {
        container(row![
            Space::with_width(Length::FillPortion(1)),
            card(
                text(self.title()),
                column![
                    text(&self.kind),
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
                .spacing(DEFAULT_INDENT)
            )
            .close_size(25.)
            .width(Length::FillPortion(15))
            .on_close(Message::Cancel),
            Space::with_width(Length::FillPortion(1))
        ])
        .center_y()
        .into()
    }
}
