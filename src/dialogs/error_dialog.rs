use iced::{
    widget::{button, column, horizontal_space, row, text, vertical_space},
    Element,
};

#[derive(Debug, Clone)]
pub enum Message {
    Ok,
}

pub struct ErrorDialog {
    message: String,
}

impl ErrorDialog {
    pub fn new(message: impl ToString) -> Self {
        Self {
            message: message.to_string(),
        }
    }

    pub fn view(&self) -> Element<Message> {
        row![
            horizontal_space(),
            column![
                vertical_space(),
                text(format!("Error: {}", &self.message)),
                row![horizontal_space(), button("Ok").on_press(Message::Ok)],
                vertical_space()
            ],
            horizontal_space()
        ]
        .into()
    }
}
