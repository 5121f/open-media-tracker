/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use iced::{
    widget::{container, text},
    Element, Length,
};

use crate::{
    gui::IDialog,
    model::{LoadingKind, LoadingQueue},
};

pub struct Message;

pub type LoadingScreen<T> = LoadingQueue<T>;

impl<T> IDialog for LoadingScreen<T>
where
    T: LoadingKind,
{
    type Message = Message;

    fn title(&self) -> String {
        String::from("Loading")
    }

    fn view(&self) -> Element<Self::Message> {
        container(text("Loading..."))
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
