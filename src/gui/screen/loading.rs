/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use iced::{
    widget::{center, text},
    Element, Length,
};

use crate::{
    gui::Screen,
    model::{LoadingKind, LoadingQueue},
};

pub struct Msg;

pub type LoadingScrn<T> = LoadingQueue<T>;

impl<T> Screen for LoadingScrn<T>
where
    T: LoadingKind,
{
    type Message = Msg;

    fn title(&self) -> String {
        String::from("Loading")
    }

    fn view(&self) -> Element<Self::Message> {
        center(text("Loading..."))
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
