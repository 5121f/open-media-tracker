use iced::{
    widget::{column, horizontal_space, row, text},
    Alignment, Element,
};

use crate::{
    serial::viewmodel::Serial,
    view_utils::{square_button, DEFAULT_INDENT},
};

#[derive(Debug, Clone)]
pub enum Message {
    AddSerial,
    ChangeSerial(usize),
}

#[derive(Default)]
pub struct MainScreen {
    media: Vec<Serial>,
}

impl MainScreen {
    pub fn new(media: Vec<Serial>) -> Self {
        Self { media }
    }

    pub fn view(&self) -> Element<Message> {
        column![
            square_button("+").on_press(Message::AddSerial),
            column(
                self.media
                    .iter()
                    .enumerate()
                    .map(|(id, m)| row![
                        text(&m.name),
                        horizontal_space(),
                        square_button("...").on_press(Message::ChangeSerial(id))
                    ]
                    .align_items(Alignment::Center))
                    .map(Into::into)
                    .collect::<Vec<_>>()
            )
            .spacing(DEFAULT_INDENT)
        ]
        .spacing(DEFAULT_INDENT)
        .padding(DEFAULT_INDENT)
        .into()
    }
}
