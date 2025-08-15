/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use cosmic::iced::{Alignment, Length};
use cosmic::iced_widget::{column, row};
use cosmic::widget::{Space, button, container, scrollable, segmented_button};
use cosmic::{Element, Task, theme};
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;

use crate::gui::utils::search_bar;
use crate::gui::{self, Page, app};
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

    pub fn update(&mut self, message: Msg, media_list: &mut [MediaHandler]) -> Task<app::Msg> {
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
                let mut search_result: Vec<(&str, i64)> = media_list
                    .iter()
                    .map(MediaHandler::name)
                    .filter_map(|name| {
                        let scope = matcher.fuzzy_match(name, &self.search_bar);
                        scope.map(|scope| (name, scope))
                    })
                    .collect();
                search_result
                    .sort_by(|(_name_a, scope_a), (_name_b, scope_b)| scope_a.cmp(scope_b));
                search_result.reverse();

                let mut builder = SegButtonModel::builder();
                for (media_name, _scope) in search_result {
                    builder = builder.insert(move |b| b.text(media_name.to_owned()));
                }
                self.media_list_seg_button = builder.build();
            }
            Msg::AddMedia => return Task::done(app::Msg::CreateMedia),
            Msg::MenuButton(entity) => {
                let Some(selected_media_name) = self.media_list_seg_button.text(entity) else {
                    return Task::none();
                };
                return Task::done(app::Msg::SelectMedia(selected_media_name.to_owned()));
            }
        }
        Task::none()
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

    fn view(&self) -> Element<'_, Self::Message> {
        let spacing = theme::spacing();

        column![
            container(row![
                container(
                    match &self.sorting {
                        Some(sorting) if sorting.reverse =>
                            button::icon(gui::icon::sort_descending()),
                        Some(_) | None => button::icon(gui::icon::sort_ascending()),
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
            .spacing(spacing.space_xxs)
            .height(Length::Fill),
        ]
        .spacing(spacing.space_xs)
        .padding(spacing.space_xxxs)
        .height(Length::Fill)
        .into()
    }
}
