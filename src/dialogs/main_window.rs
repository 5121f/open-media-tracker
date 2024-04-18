use std::rc::Rc;

use iced::{
    widget::{button, column, horizontal_space, row, text},
    Element,
};

use crate::Serial;

#[derive(Debug, Clone)]
pub enum Message {
    AddSerial,
    ChangeSerial(usize),
}

pub struct MainWindow {
    media: Vec<Rc<Serial>>,
}

impl MainWindow {
    pub fn new(media: Vec<Rc<Serial>>) -> Self {
        Self { media }
    }

    pub fn view(&self) -> Element<Message> {
        column![
            button("+").on_press(Message::AddSerial),
            column(
                self.media
                    .iter()
                    .enumerate()
                    .map(|(id, m)| row![
                        text(&m.name),
                        horizontal_space(),
                        button("...").on_press(Message::ChangeSerial(id))
                    ])
                    .map(Into::into)
                    .collect::<Vec<_>>()
            )
        ]
        .into()
    }
}
