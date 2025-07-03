/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use cosmic::iced::Alignment;
use cosmic::iced_widget::row;
use cosmic::theme;
use cosmic::widget::{Row, text, text_input};

pub fn signed_text_input<'a, M, F>(sign: &'a str, value: &'a str, on_input: F) -> Row<'a, M>
where
    M: Clone + 'static,
    F: Fn(String) -> M + 'a,
{
    row![text(sign), text_input(sign, value).on_input(on_input)]
        .spacing(theme::spacing().space_xs)
        .align_y(Alignment::Center)
}
