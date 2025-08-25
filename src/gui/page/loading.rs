/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::hash::Hash;

use cosmic::Element;
use cosmic::iced::Length;
use cosmic::iced_widget::center;
use cosmic::widget::text;

use crate::gui::Page;
use crate::model::LoadingQueue;

pub struct Msg;

pub type LoadingPage<T> = LoadingQueue<T>;

impl<T> Page for LoadingPage<T>
where
    T: PartialEq + Eq + Hash,
{
    type Message = Msg;

    fn view(&self) -> Element<'_, Self::Message> {
        center(text("Loading..."))
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
