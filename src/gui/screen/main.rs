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
    MenuButton(ListMessage),
}

pub fn main_screen_view(media: &[Series]) -> Element<Message> {
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
