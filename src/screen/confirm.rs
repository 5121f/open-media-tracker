use iced::{
    widget::{button, column, horizontal_space, row, text, vertical_space},
    Element,
};

#[derive(Debug, Clone)]
pub enum Message {
    Confirm,
    Cancel,
}

pub struct ConfirmScreen {
    question: String,
}

impl ConfirmScreen {
    pub fn new(question: String) -> Self {
        Self { question }
    }

    pub fn view(&self) -> Element<Message> {
        row![
            horizontal_space(),
            column![
                vertical_space(),
                text(&self.question),
                row![
                    button("Cancel").on_press(Message::Cancel),
                    horizontal_space(),
                    button("Confirm").on_press(Message::Confirm)
                ],
                vertical_space()
            ],
            horizontal_space()
        ]
        .into()
    }
}
