use std::rc::Rc;

use iced::{
    theme,
    widget::{button, column, horizontal_space, row, scrollable, text},
    Alignment, Element,
};

use crate::{
    serial::Serial,
    view_utils::{square_button, DEFAULT_INDENT},
};

#[derive(Debug, Clone)]
pub enum Message {
    AddSerial,
    ChangeSerial(usize),
}

#[derive(Default)]
pub struct MainScreen {
    media: Vec<Rc<Serial>>,
}

impl MainScreen {
    pub fn new(media: Vec<Rc<Serial>>) -> Self {
        Self { media }
    }

    pub fn view(&self) -> Element<Message> {
        column![
            row![
                horizontal_space(),
                button("Add serial")
                    .style(theme::Button::Positive)
                    .on_press(Message::AddSerial),
                horizontal_space(),
            ],
            scrollable(
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
            )
        ]
        .spacing(DEFAULT_INDENT)
        .padding(DEFAULT_INDENT)
        .into()
    }
}
