use iced::{
    theme,
    widget::{button, container, Column},
    Element, Length,
};

use crate::{
    gui::{list::list, utils::PADDING, ListMessage},
    media::Media,
};

#[derive(Debug, Clone)]
pub enum Message {
    AddMedia,
    MenuButton(ListMessage),
}

pub fn main_screen_view(media: &[Media]) -> Element<Message> {
    let mut layout = Column::new().spacing(PADDING).padding(PADDING);

    let add_media_button = container(
        button("Add media")
            .style(theme::Button::Positive)
            .on_press(Message::AddMedia),
    )
    .width(Length::Fill)
    .center_x();

    let media_list = list(media.into_iter().map(Media::name).collect())
        .map(|list| list.map(Message::MenuButton));

    layout = layout.push(add_media_button);
    layout = layout.push_maybe(media_list);

    layout.into()
}
