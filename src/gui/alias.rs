/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use iced::{
    alignment,
    widget::{button, row, text, text_input, Button, Row},
    Color, Theme,
};

pub const INDENT: u16 = 5;
pub const PADDING: u16 = 10;
pub const GRAY: Color = Color::from_rgb(0.6, 0.6, 0.6);

pub fn square_button<M>(content: &str) -> Button<M> {
    button(
        text(content)
            .align_x(alignment::Horizontal::Center)
            .line_height(1.0)
            .size(20),
    )
    .height(30)
    .width(30)
}

pub fn link<M>(s: &str) -> Button<M> {
    const CYAN: Color = Color::from_rgb(0., 1., 1.);
    button(text(s).color(CYAN))
        .padding(0)
        .style(link_button_style)
}

fn link_button_style(_theme: &Theme, _status: button::Status) -> button::Style {
    button::Style {
        background: None,
        ..Default::default()
    }
}

pub fn signed_text_input<'a, M, F>(sign: &'a str, value: &str, on_input: F) -> Row<'a, M>
where
    M: Clone + 'a,
    F: 'a + Fn(String) -> M,
{
    row![text(sign), text_input(sign, value).on_input(on_input)]
        .spacing(INDENT)
        .align_y(alignment::Vertical::Center)
}
