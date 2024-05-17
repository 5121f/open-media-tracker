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
pub struct MainScreen;

impl MainScreen {
    pub fn new() -> Self {
        Self
    }

    pub fn view(&self, media: &[Series]) -> Element<Message> {
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
                    media
                        .iter()
                        .enumerate()
                        .map(|(id, m)| row![
                            text(&m.name()),
                            horizontal_space(),
                            square_button("...").on_press(Message::ChangeSeries(id))
                        ]
                        .spacing(DEFAULT_INDENT))
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
