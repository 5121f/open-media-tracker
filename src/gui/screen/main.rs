use iced::{
    theme,
    widget::{button, column, container},
    Element, Length,
};

use crate::{
    gui::{list::list, utils::DEFAULT_INDENT, ListMessage},
    series::Series,
};

#[derive(Debug, Clone)]
pub enum Message {
    AddSeries,
    ChangeSeries(usize),
    MenuButton(ListMessage),
}

#[derive(Default)]
pub struct MainScreen;

impl MainScreen {
    pub fn new() -> Self {
        Self
    }

    pub fn view<'a>(&'a self, media: &'a [Series]) -> Element<Message> {
        column![
            container(
                button("Add series")
                    .style(theme::Button::Positive)
                    .on_press(Message::AddSeries)
            )
            .width(Length::Fill)
            .center_x(),
            list(media.into_iter().map(Series::name).collect()).map(Message::MenuButton)
        ]
        .spacing(DEFAULT_INDENT)
        .padding(DEFAULT_INDENT)
        .into()
    }
}
