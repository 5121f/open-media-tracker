/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use cosmic::iced::{Alignment, Length};
use cosmic::iced_widget::column;
use cosmic::widget::{button, container, segmented_button};
use cosmic::{Element, theme};

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
        let spacing = theme::active().cosmic().spacing;

        column![
            container(button::suggested("Add media").on_press(Msg::AddMedia))
                .width(Length::Fill)
                .align_x(Alignment::Center),
            segmented_button::vertical(&self.media_list_seg_button)
                .on_activate(Msg::MenuButton)
                .button_padding([spacing.space_s, 0, 0, spacing.space_s]),
        ]
        .spacing(spacing.space_xs)
        .padding(spacing.space_xs)
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
