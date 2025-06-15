/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use cosmic::Element;
use cosmic::iced::{Alignment, Length};
use cosmic::widget::{Column, button, container, segmented_button};

use crate::gui::utils::LONG_INDENT;
use crate::model::MediaHandler;

#[derive(Debug, Clone)]
pub enum Msg {
    AddMedia,
    MenuButton(segmented_button::Entity),
}

#[derive(Default)]
pub struct MainScrn {
    media_list_seg_button: segmented_button::Model<segmented_button::SingleSelect>,
}

impl MainScrn {
    pub fn new(media_list: &[MediaHandler]) -> Self {
        Self {
            media_list_seg_button: Self::build(media_list),
        }
    }

    pub fn update(&mut self, media_list: &[MediaHandler]) {
        self.media_list_seg_button = Self::build(media_list);
    }

    pub fn view(&self) -> Element<Msg> {
        Column::new()
            .push(
                container(button::suggested("Add media").on_press(Msg::AddMedia))
                    .width(Length::Fill)
                    .align_x(Alignment::Center),
            )
            .push(
                segmented_button::vertical(&self.media_list_seg_button)
                    .on_activate(Msg::MenuButton),
            )
            .spacing(LONG_INDENT)
            .padding(LONG_INDENT)
            .height(Length::Fill)
            .into()
    }

    pub fn selected(&self, entity: segmented_button::Entity) -> Option<&str> {
        self.media_list_seg_button.text(entity)
    }

    fn build(
        media_list: &[MediaHandler],
    ) -> segmented_button::Model<segmented_button::SingleSelect> {
        let mut builder = segmented_button::Model::builder();
        for media in media_list {
            builder = builder.insert(|b| b.text(media.name().to_owned()));
        }
        builder.build()
    }
}
