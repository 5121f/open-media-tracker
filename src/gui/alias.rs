/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use iced::{
    widget::{button, row, text, text::IntoFragment, text_input, Button, Row},
    Alignment, Color,
};

pub const INDENT: u16 = 5;
pub const LONG_INDENT: u16 = 10;
pub const GRAY: Color = Color::from_rgb(0.6, 0.6, 0.6);
pub const CYAN: Color = Color::from_rgb(0., 1., 1.);

pub fn square_button<'a, M>(content: impl IntoFragment<'a>) -> Button<'a, M> {
    button(
        text(content)
            .align_x(Alignment::Center)
            .line_height(1.0)
            .size(20),
    )
    .height(30)
    .width(30)
}

pub fn link<'a, M>(s: impl IntoFragment<'a>) -> Button<'a, M> {
    button(text(s).color(CYAN)).padding(0).style(button::text)
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
