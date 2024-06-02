use iced::{
    theme,
    widget::{button, column, container, scrollable, Button},
    Border, Color, Element, Length, Theme,
};

use super::utils::DEFAULT_INDENT;

#[derive(Debug, Clone)]
pub enum Message {
    Enter(usize),
}

pub fn list(buttons: Vec<&str>) -> Option<Element<Message>> {
    if buttons.is_empty() {
        return None;
    }

    let view = scrollable(
        container(
            column(
                buttons
                    .into_iter()
                    .enumerate()
                    .map(|(id, text)| list_button(text).on_press(Message::Enter(id)))
                    .map(Into::into)
                    .collect::<Vec<_>>(),
            )
            .padding(DEFAULT_INDENT),
        )
        .style(theme::Container::Custom(Box::new(List))),
    );

    Some(view.into())
}

pub fn list_button(text: &str) -> Button<Message> {
    button(text)
        .style(theme::Button::Custom(Box::new(ListButton)))
        .width(Length::Fill)
}

fn background_color() -> Color {
    Color::from_rgb8(26, 27, 30)
}

struct ListButton;

impl button::StyleSheet for ListButton {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            shadow_offset: Default::default(),
            background: Some(iced::Background::Color(background_color())),
            text_color: Color::WHITE,
            border: Border::with_radius(10.),
            shadow: Default::default(),
        }
    }

    fn hovered(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            shadow_offset: Default::default(),
            background: Some(iced::Background::Color(Color::from_rgb8(40, 42, 46))),
            text_color: Color::WHITE,
            border: Border::with_radius(10.),
            shadow: Default::default(),
        }
    }
}

struct List;

impl container::StyleSheet for List {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            text_color: None,
            background: Some(iced::Background::Color(background_color())),
            border: Border::with_radius(10.),
            shadow: Default::default(),
        }
    }
}
