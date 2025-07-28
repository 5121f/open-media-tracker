/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use cosmic::iced::{Alignment, Length};
use cosmic::iced_widget::{column, row};
use cosmic::widget::{Space, button, container, scrollable, segmented_button};
use cosmic::{Element, theme};
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;

use crate::gui;
use crate::gui::Page;
use crate::gui::utils::search_bar;
use crate::model::MediaHandler;

#[derive(Debug, Clone)]
pub enum Msg {
    AddMedia,
    MenuButton(segmented_button::Entity),
    SortButton,
    SearchBarChanged(String),
}

enum SortType {
    Alphabet,
}

struct Sorting {
    _type: SortType,
    reverse: bool,
}

type SegButtonModel = segmented_button::Model<segmented_button::SingleSelect>;

#[derive(Default)]
pub struct MainPage {
    media_list_seg_button: SegButtonModel,
    sorting: Option<Sorting>,
    search_bar: String,
}

impl MainPage {
    pub fn new(media_list: &[MediaHandler]) -> Self {
        Self {
            media_list_seg_button: Self::build(media_list),
            sorting: None,
            search_bar: String::new(),
        }
    }

    pub fn update_media(&mut self, media_list: &[MediaHandler]) {
        self.media_list_seg_button = Self::build(media_list);
    }

    pub fn update(&mut self, message: Msg, media_list: &mut [MediaHandler]) {
        match message {
            Msg::SortButton => {
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

                self.media_list_seg_button = Self::build(media_list);
            }
            Msg::SearchBarChanged(value) => {
                self.search_bar = value;

                let matcher = SkimMatcherV2::default();
                let mut search_result: Vec<(String, i64)> = media_list
                    .iter()
                    .map(MediaHandler::name)
                    .map(ToOwned::to_owned)
                    .filter_map(|n| {
                        let scope = matcher.fuzzy_match(&n, &self.search_bar);
                        scope.map(|s| (n, s))
                    })
                    .collect();
                search_result.sort_by(|a, b| a.1.cmp(&b.1));
                search_result.reverse();

                let mut builder = SegButtonModel::builder();
                for (media_name, _) in search_result {
                    builder = builder.insert(|b| b.text(media_name.clone()));
                }
                self.media_list_seg_button = builder.build();
            }
            _ => {}
        }
    }

    pub fn selected(&self, entity: segmented_button::Entity) -> Option<&str> {
        self.media_list_seg_button.text(entity)
    }

    fn build(media_list: &[MediaHandler]) -> SegButtonModel {
        let mut builder = SegButtonModel::builder();
        for media in media_list {
            builder = builder.insert(|b| b.text(media.name().to_owned()));
        }
        builder.build()
    }
}

impl Page for MainPage {
    type Message = Msg;

    fn view(&self) -> Element<Self::Message> {
        let spacing = theme::spacing();

        column![
            container(row![
                container(
                    match &self.sorting {
                        Some(sorting) =>
                            if sorting.reverse {
                                button::icon(gui::icon::sort_descending())
                            } else {
                                button::icon(gui::icon::sort_ascending())
                            },
                        None => button::icon(gui::icon::sort_ascending()),
                    }
                    .on_press(Msg::SortButton)
                )
                .width(Length::Fill),
                button::suggested("Add media").on_press(Msg::AddMedia),
                row![
                    Space::new(Length::Fixed(40.0), Length::Shrink),
                    search_bar(&self.search_bar).on_input(Msg::SearchBarChanged),
                ],
            ])
            .width(Length::Fill)
            .align_x(Alignment::Center),
            scrollable(
                segmented_button::vertical(&self.media_list_seg_button)
                    .on_activate(Msg::MenuButton)
                    .button_padding([spacing.space_s, 0, 0, spacing.space_s])
            )
            .spacing(spacing.space_xxs),
        ]
        .spacing(spacing.space_xs)
        .padding(spacing.space_xxxs)
        .height(Length::Fill)
        .into()
    }
}
