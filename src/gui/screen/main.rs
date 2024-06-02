use iced::{
    theme,
    widget::{button, column, container},
    Element, Length,
};

use crate::{
    gui::{list::list, utils::DEFAULT_INDENT, ListMessage},
    media::Media,
};

#[derive(Debug, Clone)]
pub enum Message {
    AddMedia,
    MenuButton(ListMessage),
}

pub fn main_screen_view(media: &[Media]) -> Element<Message> {
    column![
        container(
            button("Add media")
                .style(theme::Button::Positive)
                .on_press(Message::AddMedia)
        )
        .width(Length::Fill)
        .center_x(),
        list(media.into_iter().map(Media::name).collect()).map(Message::MenuButton)
    ]
    .spacing(DEFAULT_INDENT)
    .padding(DEFAULT_INDENT)
    .into()
}
