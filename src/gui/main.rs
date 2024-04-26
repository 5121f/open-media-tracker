use std::{cell::RefCell, rc::Rc};

use iced::{
    theme,
    widget::{button, column, horizontal_space, row, scrollable, text},
    Alignment, Element,
};

use crate::{
    series::Series,
    view_utils::{square_button, DEFAULT_INDENT},
};

#[derive(Debug, Clone)]
pub enum Message {
    AddSeries,
    ChangeSeries(usize),
}

#[derive(Default)]
pub struct MainScreen {
    media: Vec<Rc<RefCell<Series>>>,
}

impl MainScreen {
    pub fn new(media: Vec<Rc<RefCell<Series>>>) -> Self {
        Self { media }
    }

    pub fn view(&self) -> Element<Message> {
        column![
            row![
                horizontal_space(),
                button("Add series")
                    .style(theme::Button::Positive)
                    .on_press(Message::AddSeries),
                horizontal_space(),
            ],
            scrollable(
                column(
                    self.media
                        .iter()
                        .enumerate()
                        .map(|(id, m)| row![
                            text(&m.borrow().name()),
                            horizontal_space(),
                            square_button("...").on_press(Message::ChangeSeries(id))
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
