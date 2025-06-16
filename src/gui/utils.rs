/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use cosmic::iced::Alignment;
use cosmic::iced_widget::{row, text_input};
use cosmic::theme;
use cosmic::widget::{Row, button, text};

pub fn signed_text_input<'a, M, F>(sign: &'a str, value: &str, on_input: F) -> Row<'a, M>
where
    M: Clone + 'static,
    F: Fn(String) -> M + 'a,
{
    let spacing = theme::active().cosmic().spacing;

    row![text(sign), text_input(sign, value).on_input(on_input)]
        .spacing(spacing.space_xxs)
        .align_y(Alignment::Center)
}

pub fn square_button<M>(sign: &str) -> button::TextButton<M> {
    button::standard(sign).height(30).font_size(30)
}
