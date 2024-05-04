use std::fmt::Display;

use iced::{
    theme,
    widget::{button, column, horizontal_space, row, text, vertical_space, Space},
    Element,
};
use iced_aw::card;

use crate::{
    error::Error,
    gui::{utils::DEFAULT_INDENT, IDialog},
};

#[derive(Debug, Clone)]
pub enum Message {
    Ok { critical: bool },
}

pub struct ErrorScreen<T> {
    kind: T,
    critical: bool,
}

impl<T: Display> IDialog for ErrorScreen<T> {
    type Message = Message;

    fn title(&self) -> String {
        String::from("Error")
    }

    fn view(&self) -> Element<Message> {
        let ok_button_style = if self.critical {
            theme::Button::Destructive
        } else {
            theme::Button::Primary
        };
        row![
            Space::with_width(80),
            column![
                vertical_space(),
                card(
                    text(self.title()),
                    column![
                        text(&self.kind),
                        row![
                            horizontal_space(),
                            button("Ok").style(ok_button_style).on_press(Message::Ok {
                                critical: self.critical
                            })
                        ],
                    ]
                    .spacing(DEFAULT_INDENT)
                )
                .style(iced_aw::style::card::CardStyles::Danger),
                vertical_space()
            ],
            Space::with_width(80)
        ]
        .into()
    }
}

impl From<Error> for ErrorScreen<Error> {
    fn from(value: Error) -> Self {
        let critical = value.critical;
        Self {
            kind: value,
            critical,
        }
    }
}
