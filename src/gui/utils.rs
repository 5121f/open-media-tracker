/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::borrow::Cow;

use cosmic::iced::Alignment;
use cosmic::iced_widget::row;
use cosmic::theme;
use cosmic::widget::{Row, SpinButton, TextInput, container, icon, text, text_input};

use crate::gui;

pub fn signed_text_input<'a, M, F>(
    sign: &'a str,
    value: impl Into<Cow<'a, str>>,
    on_input: F,
) -> Row<'a, M>
where
    M: Clone + 'static,
    F: Fn(String) -> M + 'a,
{
    row![text(sign), text_input(sign, value).on_input(on_input)]
        .spacing(theme::spacing().space_xs)
        .align_y(Alignment::Center)
}

pub fn spin_button<'a, M>(
    value: u8,
    on_press: impl Fn(u8) -> M + 'static,
) -> SpinButton<'a, u8, M> {
    cosmic::widget::spin_button(value.to_string(), value, 1, 1, u8::MAX, on_press)
}

pub fn search_bar<'a, M>(value: impl Into<Cow<'a, str>>) -> TextInput<'a, M>
where
    M: Clone + 'static,
{
    text_input("Search", value)
        .style(theme::TextInput::Search)
        .leading_icon(
            container(icon::icon(gui::icon::search()).size(16))
                .padding([0, 0, 0, 3])
                .into(),
        )
}
