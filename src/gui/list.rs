/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use iced::widget::{button, column, container, scrollable};
use iced::{Background, Border, Color, Element, Length, Theme};

use crate::gui::utils::INDENT;

#[derive(Debug, Clone)]
pub enum Message {
    Enter(usize),
}

pub fn list(buttons: Vec<&str>) -> Option<Element<Message>> {
    if buttons.is_empty() {
        return None;
    }

    let view = container(
        scrollable(
            column(
                buttons
                    .into_iter()
                    .enumerate()
                    .map(|(id, text)| {
                        button(text)
                            .style(list_button_style)
                            .width(Length::Fill)
                            .on_press(Message::Enter(id))
                    })
                    .map(Into::into),
            )
            .padding(INDENT),
        )
        .height(Length::Fill),
    )
    .width(Length::Fill)
    .style(list_container_style);

    Some(view.into())
}

fn list_button_style(_theme: &Theme, statsus: button::Status) -> button::Style {
    match statsus {
        button::Status::Active => button::Style {
            background: Some(background()),
            text_color: Color::WHITE,
            border: button_border(),
            ..Default::default()
        },
        button::Status::Hovered => {
            let background_color = Color::from_rgb8(40, 42, 46);
            let background = Background::Color(background_color);

            button::Style {
                background: Some(background),
                text_color: Color::WHITE,
                border: button_border(),
                ..Default::default()
            }
        }
        button::Status::Pressed => button::Style {
            text_color: Color::WHITE,
            ..Default::default()
        },
        button::Status::Disabled => button::Style::default(),
    }
}

fn list_container_style(_theme: &Theme) -> container::Style {
    container::Style {
        text_color: None,
        background: Some(background()),
        border: button_border(),
        ..Default::default()
    }
}

fn background() -> Background {
    let background_color = Color::from_rgb8(22, 23, 25);
    Background::Color(background_color)
}

fn button_border() -> Border {
    Border::default().rounded(10.)
}
