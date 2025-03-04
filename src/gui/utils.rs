/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use iced::widget::{Button, Row, button, row, text, text_input};
use iced::{Alignment, Background, Border, Color, Theme};

pub const INDENT: u16 = 5;
pub const LONG_INDENT: u16 = 10;
pub const GRAY: Color = Color::from_rgb(0.6, 0.6, 0.6);
pub const CYAN: Color = Color::from_rgb(0.0, 1.0, 1.0);

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

fn link_style(theme: &Theme, status: button::Status) -> button::Style {
    let base = button::text(theme, status);
    match status {
        button::Status::Active | button::Status::Disabled | button::Status::Pressed => base,
        button::Status::Hovered => {
            let palette = theme.extended_palette();
            let background = palette.background.base.color;
            let background = Color::from_rgb(
                background.r + 0.05,
                background.g + 0.05,
                background.b + 0.05,
            );
            button::Style {
                background: Some(Background::Color(background)),
                border: Border::default().rounded(10.),
                ..base
            }
        }
    }
}

pub fn signed_text_input<'a, M, F>(sign: &'a str, value: &str, on_input: F) -> Row<'a, M>
where
    M: Clone + 'a,
    F: 'a + Fn(String) -> M,
{
    row![text(sign), text_input(sign, value).on_input(on_input)]
        .spacing(INDENT)
        .align_y(Alignment::Center)
}
