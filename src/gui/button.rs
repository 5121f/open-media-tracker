/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use iced::widget::{Button, button as iced_button, text};
use iced::{Alignment, Background, Border, Color, Element, Theme};

use crate::gui::utils::CYAN;

pub fn square_button<'a, M>(content: impl text::IntoFragment<'a>) -> Button<'a, M> {
    button(
        text(content)
            .align_x(Alignment::Center)
            .line_height(1.0)
            .size(20),
    )
    .height(30)
    .width(30)
}

pub fn link<'a, M>(s: impl text::IntoFragment<'a>) -> Button<'a, M> {
    button(text(s).color(CYAN)).style(link_style)
}

fn link_style(theme: &Theme, status: iced_button::Status) -> iced_button::Style {
    let base = iced_button::text(theme, status);
    match status {
        iced_button::Status::Active
        | iced_button::Status::Disabled
        | iced_button::Status::Pressed => base,
        iced_button::Status::Hovered => {
            let palette = theme.extended_palette();
            let background = palette.background.base.color;
            let background = Color::from_rgb(
                background.r + 0.05,
                background.g + 0.05,
                background.b + 0.05,
            );
            iced_button::Style {
                background: Some(Background::Color(background)),
                border: button_border(),
                ..base
            }
        }
    }
}

pub fn button<'a, Message>(content: impl Into<Element<'a, Message>>) -> Button<'a, Message> {
    button_styled(content, iced_button::primary)
}

pub fn button_styled<'a, Message>(
    content: impl Into<Element<'a, Message>>,
    style: impl Fn(&Theme, iced_button::Status) -> iced_button::Style + 'a,
) -> Button<'a, Message> {
    Button::new(content).style(move |theme, status| iced_button::Style {
        border: button_border(),
        ..style(theme, status)
    })
}

fn button_border() -> Border {
    Border::default().rounded(9.0)
}
