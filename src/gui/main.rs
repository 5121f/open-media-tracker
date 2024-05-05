use std::{cell::RefCell, rc::Rc};

use iced::{
    theme,
    widget::{button, column, container, horizontal_space, row, scrollable, text},
    Element, Length,
};

use crate::{
    gui::utils::{square_button, DEFAULT_INDENT},
    series::Series,
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
            container(
                button("Add series")
                    .style(theme::Button::Positive)
                    .on_press(Message::AddSeries)
            )
            .width(Length::Fill)
            .center_x(),
            scrollable(
                column(
                    self.media
                        .iter()
                        .enumerate()
                        .map(|(id, m)| row![
                            text(&m.borrow().name()),
                            horizontal_space(),
                            square_button("...").on_press(Message::ChangeSeries(id))
                        ])
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
