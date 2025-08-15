/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

mod kind;
mod message;

use std::path::{Path, PathBuf};
use std::sync::Arc;

use cosmic::dialog::file_chooser;
use cosmic::iced::font::Weight;
use cosmic::iced::{Alignment, Length};
use cosmic::iced_core::text::Wrapping;
use cosmic::iced_widget::{column, row};
use cosmic::widget::{
    Column, Space, button, container, divider, horizontal_space, popover, text, tooltip,
};
use cosmic::{Element, Task, font, style, theme};
use derive_more::From;
use expand_tilde::ExpandTilde;

use crate::gui;
use crate::gui::page::{ConfirmDlg, ConfirmPageMsg, WarningDlg, WarningPageMsg};
use crate::gui::utils::signed_text_input;
use crate::model::{Episode, ErrorKind, LoadedData, MediaHandler, MediaList, Result};
use crate::utils;
use kind::{ConfirmKind, WarningKind};
pub use message::Msg;

pub struct MediaEditPage {
    confirm: ConfirmDlg<ConfirmKind>,
    warning: WarningDlg<WarningKind>,
    editable_media_id: usize,
    episodes: Episodes,
    buffer_name: String,
    chapter: u8,
    episode: u8,
}

impl MediaEditPage {
    pub fn new(media_list: &[MediaHandler], editable_media_id: usize) -> (Self, Task<Msg>) {
        let editable_media = &media_list[editable_media_id];
        let task = load_episodes(editable_media);
        (
            Self {
                confirm: ConfirmDlg::closed(),
                warning: WarningDlg::closed(),
                editable_media_id,
                episodes: LoadedData::Loading.into(),
                buffer_name: editable_media.name().to_string(),
                chapter: editable_media.chapter(),
                episode: editable_media.episode(),
            },
            task,
        )
    }

    pub fn view<'a>(&'a self, media_list: &'a [MediaHandler]) -> Element<'a, Msg> {
        let spacing = theme::spacing();

        let media = self.editable_media(media_list);
        let top = row![
            container(
                button::text("Back")
                    .leading_icon(gui::icon::back())
                    .on_press(Msg::Back)
            )
            .width(Length::Fill),
            text::title4(media.name()),
            container(button::destructive("Delete").on_press(Msg::Delete(self.editable_media_id)))
                .width(Length::Fill)
                .align_x(Alignment::End),
        ]
        .align_y(Alignment::Center);
        let watch = container(
            button::suggested("Watch").on_press_maybe(
                self.episode(media_list)
                    .and_then(|res| res.ok().map(Episode::path).map(Msg::watch)),
            ),
        )
        .width(Length::Fill)
        .align_x(Alignment::Center);
        let watch_sign = self.watch_sign(media_list).map(|watch_sign| {
            text(watch_sign)
                .font(font::light())
                .size(13)
                .wrapping(Wrapping::WordOrGlyph)
                .align_x(Alignment::Center)
                .width(Length::Fill)
        });
        let edit_view = self.edit_view(media.chapter_path());

        let layout = Column::new()
            .push(top)
            .push(Space::with_height(Length::Fixed(spacing.space_xxs.into())))
            .push(watch)
            .push_maybe(watch_sign)
            .push_maybe(self.warning.view_into())
            .push(edit_view)
            .padding(spacing.space_xs)
            .spacing(spacing.space_xs)
            .height(Length::Fill);

        if let Some(confirm_screen_view) = self.confirm.view_into() {
            return popover::Popover::new(layout)
                .popup(confirm_screen_view)
                .into();
        }

        layout.into()
    }

    fn edit_view<'a>(&'a self, chapter_path: &'a Path) -> Element<'a, Msg> {
        let spacing = theme::spacing();

        container(
            column![
                signed_text_input("Name", &self.buffer_name, Msg::NameChanged),
                divider::horizontal::default(),
                row![
                    "Chapter",
                    horizontal_space(),
                    gui::utils::spin_button(self.chapter, Msg::ChapterChanged),
                ]
                .spacing(spacing.space_xs)
                .align_y(Alignment::Center),
                divider::horizontal::default(),
                row![
                    "Episode",
                    horizontal_space(),
                    gui::utils::spin_button(self.episode, Msg::EpisodeChanged),
                ]
                .spacing(spacing.space_xxs)
                .align_y(Alignment::Center),
                divider::horizontal::default(),
                row![
                    signed_text_input(
                        "Chapter path",
                        chapter_path.to_string_lossy(),
                        Msg::ChapterPathChanged
                    ),
                    tooltip(
                        button::standard("...")
                            .height(30)
                            .font_size(18)
                            .font_weight(Weight::Bold)
                            .on_press(Msg::ChapterPathSelect),
                        text("Select folder"),
                        tooltip::Position::Top
                    ),
                    button::standard("")
                        .leading_icon(gui::icon::folder())
                        .height(30)
                        .tooltip("Open folder")
                        .on_press_maybe(
                            (!chapter_path.as_os_str().is_empty())
                                .then(|| Msg::OpenChapterDirectory)
                        ),
                ]
                .align_y(Alignment::Center)
                .spacing(spacing.space_xs)
            ]
            .spacing(spacing.space_xs),
        )
        .padding(spacing.space_xs)
        .class(style::Container::Card)
        .into()
    }

    pub fn update(&mut self, media_list: &mut MediaList, message: Msg) -> Result<Task<Msg>> {
        match message {
            Msg::NameChanged(value) => {
                self.buffer_name.clone_from(&value);
                let rename_res = media_list.rename_media(self.editable_media_id, value);
                match rename_res {
                    Ok(()) => {}
                    Err(ErrorKind::MediaNameIsUsed { .. }) => {
                        self.warning(WarningKind::NameUsed);
                        return Ok(Task::none());
                    }
                    Err(err) => return Err(err),
                }
                if matches!(self.warning.kind(), Some(WarningKind::NameUsed)) {
                    self.warning.close();
                }
            }
            Msg::ChapterChanged(value) => {
                self.chapter = value;
                self.editable_media_mut(media_list).set_chapter(value)?;
            }
            Msg::EpisodeChanged(value) => {
                self.episode = value;
                return self.set_episode(media_list, value);
            }
            Msg::ChapterPathChanged(value) => {
                return self.set_chapter_path(media_list, value);
            }
            Msg::ConfirmScreen(message) => return self.confirm_screen_update(media_list, &message),
            Msg::ChapterPathSelect => {
                return Ok(cosmic::task::future(async {
                    let dialog = file_chooser::open::Dialog::new().title("Select chapter path");
                    match dialog.open_folder().await {
                        Ok(responce) => Msg::ChapterPathSelected(responce.url().to_owned()),
                        Err(file_chooser::Error::Cancelled) => Msg::OpenDialogCanceled,
                        Err(err) => Msg::OpenDialogError(Arc::new(err)),
                    }
                }));
            }
            Msg::Warning(WarningPageMsg::Close) => self.warning.close(),
            Msg::OpenChapterDirectory => {
                let chapter_path = self
                    .editable_media(media_list)
                    .chapter_path()
                    .expand_tilde()?;
                if !chapter_path.is_dir() {
                    self.warning(WarningKind::WrongChapterPath);
                    return Ok(Task::none());
                }
                utils::open(&*chapter_path)?;
            }
            Msg::ChapterPathSelected(url) => {
                if let Ok(path) = url.to_file_path() {
                    return self.set_chapter_path(media_list, path);
                }
                self.warning(WarningKind::WrongChapterPath);
            }
            Msg::OpenDialogError(err) => return Err(ErrorKind::open_dialog(err)),
            Msg::NextChapterPath(path) => self.confirm_switch_to_next_chapter(path?),
            Msg::EpisodeListLoaded(res) => self.episodes = Episodes(res.map(Arc::new).into()),
            Msg::CheckOverflow(res) => {
                let res = res.map(Arc::new);
                self.episodes = Episodes(res.into());
                if !self.is_episode_overflow(self.episode) {
                    return Ok(Task::none());
                }
                let Some(episodes_count) = self.episodes.len() else {
                    return Ok(Task::none());
                };
                self.confirm_episode_overflow(episodes_count);
            }
            Msg::Watch { path } => utils::open(path)?,
            _ => {}
        }
        Ok(Task::none())
    }

    fn watch_sign(&self, media_list: &[MediaHandler]) -> Option<String> {
        if self
            .editable_media(media_list)
            .chapter_path()
            .as_os_str()
            .is_empty()
        {
            return None;
        }
        if matches!(self.episodes.0, LoadedData::Loading) {
            return Some(String::from("Loading..."));
        }
        let watch_sign = match self.episodes.get(self.episode_id(media_list))? {
            Ok(episode) => episode.name(),
            Err(ErrorKind::Io(err)) => {
                format!("Chapter path is incorrect: {err}")
            }
            Err(err) => format!("Chapter path is incorrect: {err}"),
        };
        Some(watch_sign)
    }

    fn confirm_screen_update(
        &mut self,
        media_list: &mut [MediaHandler],
        message: &ConfirmPageMsg,
    ) -> Result<Task<Msg>> {
        match message {
            ConfirmPageMsg::Confirm => {
                if let Some(kind) = self.confirm.kind() {
                    return self.confirm_kind_update(media_list, kind.clone());
                }
            }
            ConfirmPageMsg::Cancel => self.confirm.close(),
        }
        Ok(Task::none())
    }

    fn confirm_kind_update(
        &mut self,
        media_lost: &mut [MediaHandler],
        kind: ConfirmKind,
    ) -> Result<Task<Msg>> {
        match kind {
            ConfirmKind::SwitchToNextChapter { path } => {
                self.confirm.close();
                self.set_chapter_path(media_lost, path)
            }
            ConfirmKind::EpisodesOverflow { .. } => {
                self.confirm.close();
                self.increase_chapter(media_lost)
            }
        }
    }

    const fn editable_media<'a>(&self, media_list: &'a [MediaHandler]) -> &'a MediaHandler {
        &media_list[self.editable_media_id]
    }

    fn editable_media_mut<'a>(&self, media_list: &'a mut [MediaHandler]) -> &'a mut MediaHandler {
        &mut media_list[self.editable_media_id]
    }

    fn episode(
        &self,
        media_list: &[MediaHandler],
    ) -> Option<std::result::Result<&Episode, &ErrorKind>> {
        self.episodes.get(self.episode_id(media_list))
    }

    const fn episode_id(&self, media_list: &[MediaHandler]) -> usize {
        (self.editable_media(media_list).episode() - 1) as usize
    }

    fn set_chapter_path(
        &mut self,
        media_list: &mut [MediaHandler],
        chapter_path: impl Into<PathBuf>,
    ) -> Result<Task<Msg>> {
        let editable_media = self.editable_media_mut(media_list);
        editable_media.set_chapter_path(chapter_path)?;
        Ok(load_episodes(editable_media))
    }

    fn warning(&mut self, kind: WarningKind) {
        self.warning = WarningDlg::from_kind(kind);
    }

    fn is_episode_overflow(&self, value: u8) -> bool {
        self.episodes.len().is_some_and(|ec| ec < value as usize)
    }

    fn set_episode(&mut self, media_list: &mut [MediaHandler], value: u8) -> Result<Task<Msg>> {
        let media = self.editable_media_mut(media_list);

        match self.episodes.len() {
            Some(episodes_count) if value as usize <= episodes_count => {
                self.episode = value;
                media.set_episode(value)?;
            }
            None => {
                self.episode = value;
                media.set_episode(value)?;
            }
            Some(_) => {
                if self.is_episode_overflow(value) {
                    return Ok(self.test_overflow(media_list));
                }
            }
        }

        Ok(Task::none())
    }

    fn test_overflow(&self, media_list: &[MediaHandler]) -> Task<Msg> {
        let media = self.editable_media(media_list);
        let future = media.episode_list();
        cosmic::task::future(async { Msg::CheckOverflow(future.await) })
    }

    fn increase_chapter(&mut self, media_list: &mut [MediaHandler]) -> Result<Task<Msg>> {
        if self.chapter == 0 {
            self.chapter = 1;
            return Ok(cosmic::task::none());
        }

        self.episode = 1;
        let media = self.editable_media_mut(media_list);
        media.set_episode(1)?;
        let next_chapter = media.chapter().saturating_add(1);
        self.chapter = next_chapter;
        media.set_chapter(next_chapter)?;
        if media.chapter_path().as_os_str().is_empty() {
            return Ok(cosmic::task::none());
        }
        let next_chapter_path = media.next_chapter_path();
        Ok(cosmic::task::future(async {
            Msg::NextChapterPath(next_chapter_path.await)
        }))
    }

    fn confirm(&mut self, kind: ConfirmKind) {
        self.confirm = ConfirmDlg::from_kind(kind);
    }

    fn confirm_switch_to_next_chapter(&mut self, next_chapter_path: impl Into<PathBuf>) {
        let kind = ConfirmKind::switch_to_next_chapter(next_chapter_path);
        self.confirm(kind);
    }

    fn confirm_episode_overflow(&mut self, episodes_count: usize) {
        let kind = ConfirmKind::episode_overflow(episodes_count);
        self.confirm(kind);
    }
}

fn load_episodes(media: &MediaHandler) -> Task<Msg> {
    let future = media.episode_list();
    cosmic::task::future(async { Msg::EpisodeListLoaded(future.await) })
}

#[derive(Debug, Clone, From)]
struct Episodes(LoadedData<Arc<Vec<Episode>>, ErrorKind>);

impl Episodes {
    fn len(&self) -> Option<usize> {
        if let LoadedData::Some(episodes) = &self.0 {
            Some(episodes.len())
        } else {
            None
        }
    }

    fn get(&self, id: usize) -> Option<std::result::Result<&Episode, &ErrorKind>> {
        self.0
            .get()
            .map(|res| res.and_then(|episodes| episodes.get(id).ok_or(&ErrorKind::EpisodeNotFound)))
    }
}
