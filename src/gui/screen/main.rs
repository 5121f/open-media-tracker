/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use cosmic::iced::{Alignment, Length};
use cosmic::iced_widget::{column, row};
use cosmic::widget::{button, container, horizontal_space, icon, segmented_button};
use cosmic::{Element, theme};

use crate::model::MediaHandler;

#[derive(Debug, Clone)]
pub enum Msg {
    AddMedia,
    MenuButton(segmented_button::Entity),
    SortButton,
}

enum SortType {
    Alphabet,
}

struct Sorting {
    _type: SortType,
    reverse: bool,
}

#[derive(Default)]
pub struct MainScrn {
    media_list_seg_button: segmented_button::Model<segmented_button::SingleSelect>,
    sorting: Option<Sorting>,
}

impl MainScrn {
    pub fn new(media_list: &[MediaHandler]) -> Self {
        Self {
            media_list_seg_button: Self::build(media_list),
            sorting: None,
        }
    }

    pub fn update_media(&mut self, media_list: &[MediaHandler]) {
        self.media_list_seg_button = Self::build(media_list);
    }

    pub fn view(&self) -> Element<Msg> {
        let spacing = theme::active().cosmic().spacing;

        column![
            container(row![
                horizontal_space(),
                button::suggested("Add media").on_press(Msg::AddMedia),
                row![
                    horizontal_space(),
                    button::icon(match &self.sorting {
                        Some(sorting) =>
                            if sorting.reverse {
                                icon::from_name("view-sort-descending-symbolic")
                            } else {
                                icon::from_name("view-sort-ascending-symbolic")
                            },
                        None => icon::from_name("view-sort-ascending-symbolic"),
                    })
                    .on_press(Msg::SortButton)
                ],
            ])
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

    pub fn sort(&mut self, media_list: &mut Vec<MediaHandler>) {
        if let Some(sorting) = &mut self.sorting {
            sorting.reverse = !sorting.reverse;
            media_list.sort_by(|a, b| a.name().cmp(b.name()));
            if sorting.reverse {
                media_list.reverse();
            }
        } else {
            self.sorting = Some(Sorting {
                _type: SortType::Alphabet,
                reverse: true,
            });
            media_list.sort_by(|a, b| a.name().cmp(b.name()));
            media_list.reverse();
        }

        let mut builder = segmented_button::Model::builder();
        for media in media_list {
            builder = builder.insert(|b| b.text(media.name().to_owned()));
        }
        self.media_list_seg_button = builder.build();
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
