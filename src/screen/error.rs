use iced::{
    theme,
    widget::{button, column, horizontal_space, row, text, vertical_space},
    Element,
};

use crate::error::Error;

#[derive(Debug, Clone)]
pub enum Message {
    Ok { critical: bool },
}

pub struct ErrorScreen {
    message: String,
    critical: bool,
}

impl ErrorScreen {
    pub fn view(&self) -> Element<Message> {
        let ok_button_style = if self.critical {
            theme::Button::Destructive
        } else {
            theme::Button::Primary
        };
        row![
            horizontal_space(),
            column![
                vertical_space(),
                text(format!("Error: {}", &self.message)),
                row![
                    horizontal_space(),
                    button("Ok").style(ok_button_style).on_press(Message::Ok {
                        critical: self.critical
                    })
                ],
                vertical_space()
            ],
            horizontal_space()
        ]
        .into()
    }
}

impl From<Error> for ErrorScreen {
    fn from(value: Error) -> Self {
        Self {
            message: value.to_string(),
            critical: value.critical,
        }
    }
}
