/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use iced::{
    theme,
    widget::{button, column, container, scrollable, Button},
    Background, Border, Color, Element, Length, Theme,
};

use crate::gui::utils::INDENT;

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
            .padding(INDENT),
        )
        .style(list_container_style()),
    )
    .height(Length::Fill);

    Some(view.into())
}

pub fn list_button(text: &str) -> Button<Message> {
    button(text).style(list_button_style()).width(Length::Fill)
}

fn background() -> Background {
    let background_color = Color::from_rgb8(22, 23, 25);
    Background::Color(background_color)
}

const BUTTON_BORDER_RADIUS: f32 = 10.;

fn button_border() -> Border {
    Border::with_radius(BUTTON_BORDER_RADIUS)
}

struct ListButton;

impl button::StyleSheet for ListButton {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            shadow_offset: Default::default(),
            background: Some(background()),
            text_color: Color::WHITE,
            border: button_border(),
            shadow: Default::default(),
        }
    }

    fn hovered(&self, _style: &Self::Style) -> button::Appearance {
        let background_color = Color::from_rgb8(40, 42, 46);
        let background = Background::Color(background_color);

        button::Appearance {
            shadow_offset: Default::default(),
            background: Some(background),
            text_color: Color::WHITE,
            border: button_border(),
            shadow: Default::default(),
        }
    }
}

fn list_button_style() -> theme::Button {
    theme::Button::Custom(Box::new(ListButton))
}

struct List;

impl container::StyleSheet for List {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            text_color: None,
            background: Some(background()),
            border: button_border(),
            shadow: Default::default(),
        }
    }
}

fn list_container_style() -> theme::Container {
    theme::Container::Custom(Box::new(List))
}
