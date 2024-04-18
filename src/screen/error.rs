use iced::{
    widget::{button, column, horizontal_space, row, text, vertical_space},
    Element,
};

#[derive(Debug, Clone)]
pub enum Message {
    Ok { critical: bool },
}

pub struct ErrorScreen {
    message: String,
    critical: bool,
}

impl ErrorScreen {
    pub fn new(message: impl ToString, critical: bool) -> Self {
        Self {
            message: message.to_string(),
            critical,
        }
    }

    pub fn view(&self) -> Element<Message> {
        row![
            horizontal_space(),
            column![
                vertical_space(),
                text(format!("Error: {}", &self.message)),
                row![
                    horizontal_space(),
                    button("Ok").on_press(Message::Ok {
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
