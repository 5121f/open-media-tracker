/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::num::NonZeroU8;
use std::path::PathBuf;

use cosmic::iced::font::Weight;
use cosmic::iced::{Alignment, Length};
use cosmic::iced_core::text::Wrapping;
use cosmic::iced_widget::{column, row};
use cosmic::widget::{
    Column, button, container, divider, horizontal_space, icon, popover, spin_button, text,
};
use cosmic::{Element, style, theme};

use super::kind::{ConfirmKind, WarningKind};
use super::message::Msg;
use crate::gui::screen::{ConfirmDlg, ConfirmScrnMsg, WarningDlg, WarningMsg};
use crate::gui::utils::signed_text_input;
use crate::model::{Episode, EpisodeList, ErrorKind, MediaHandler, MediaList, Result};
use crate::open;

pub struct MediaEditScrn {
    confirm: ConfirmDlg<ConfirmKind>,
    warning: WarningDlg<WarningKind>,
    editable_media_id: usize,
    episodes: Result<EpisodeList>,
    buffer_name: String,
    chapter: u8,
    episode: u8,
}

impl MediaEditScrn {
    pub fn new(media: &[MediaHandler], editable_media_id: usize) -> Self {
        let editable_media = &media[editable_media_id];

        Self {
            confirm: ConfirmDlg::closed(),
            warning: WarningDlg::closed(),
            editable_media_id,
            episodes: editable_media.episode_list(),
            buffer_name: editable_media.name().to_string(),
            chapter: editable_media.chapter().get(),
            episode: editable_media.episode().get(),
        }
    }

    pub fn view<'a>(&'a self, media_list: &'a [MediaHandler]) -> Element<'a, Msg> {
        let spacing = theme::active().cosmic().spacing;

        let media = self.editable_media(media_list);
        let top = row![
            container(
                button::text("Back")
                    .leading_icon(icon::from_name("go-previous-symbolic"))
                    .on_press(Msg::Back)
            )
            .width(Length::Fill),
            text(media.name()),
            container(button::destructive("Delete").on_press(Msg::Delete(self.editable_media_id)))
                .width(Length::Fill)
                .align_x(Alignment::End),
        ];
        let watch = container(
            button::suggested("Watch").on_press_maybe(
                self.episode(media_list)
                    .ok()
                    .map(Episode::path)
                    .map(Msg::watch),
            ),
        )
        .width(Length::Fill)
        .align_x(Alignment::Center);
        let watch_sign = self.watch_sign(media_list).map(|watch_sign| {
            container(text(watch_sign).size(13).wrapping(Wrapping::WordOrGlyph))
                .width(Length::Fill)
                .align_x(Alignment::Center)
        });
        let edit_view = self.edit_view(media.chapter_path());

        let layout = Column::new()
            .push(top)
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

    fn edit_view(&self, chapter_path: &str) -> Element<Msg> {
        let spacing = theme::active().cosmic().spacing;

        container(
            column![
                signed_text_input("Name", &self.buffer_name, Msg::NameChanged),
                divider::horizontal::default(),
                row![
                    "Chapter",
                    horizontal_space(),
                    spin_button(
                        self.chapter.to_string(),
                        self.chapter,
                        1,
                        1,
                        u8::MAX,
                        Msg::ChapterChanged
                    ),
                ]
                .spacing(spacing.space_xs)
                .align_y(Alignment::Center),
                divider::horizontal::default(),
                row![
                    "Episode",
                    horizontal_space(),
                    spin_button(
                        self.episode.to_string(),
                        self.episode,
                        1,
                        1,
                        u8::MAX,
                        Msg::EpisodeChanged
                    ),
                ]
                .spacing(spacing.space_xxs)
                .align_y(Alignment::Center),
                divider::horizontal::default(),
                row![
                    signed_text_input("Chapter path", chapter_path, Msg::ChapterPathChanged),
                    button::standard("")
                        .leading_icon(icon::from_name("folder-symbolic"))
                        .height(30)
                        .on_press(Msg::OpenChapterDirectory),
                    button::standard("...")
                        .height(30)
                        .font_size(20)
                        .font_weight(Weight::Bold)
                        .on_press(Msg::ChapterPathSelect),
                ]
                .spacing(spacing.space_xxs)
            ]
            .spacing(spacing.space_xs),
        )
        .padding(spacing.space_xs)
        .class(style::Container::Card)
        .into()
    }

    pub fn update(&mut self, media_list: &mut MediaList, message: Msg) -> Result<()> {
        match message {
            Msg::NameChanged(value) => {
                self.buffer_name.clone_from(&value);
                let rename_res = media_list.rename_media(self.editable_media_id, value);
                match rename_res {
                    Ok(()) => {}
                    Err(ErrorKind::MediaNameIsUsed { .. }) => {
                        self.warning(WarningKind::NameUsed);
                        return Ok(());
                    }
                    Err(err) => return Err(err),
                }
                if matches!(self.warning.kind(), Some(WarningKind::NameUsed)) {
                    self.warning.close();
                }
            }
            Msg::ChapterChanged(value) => {
                self.chapter = value;
                if let Some(number) = NonZeroU8::new(value) {
                    self.editable_media_mut(media_list).set_chapter(number)?;
                }
            }
            Msg::EpisodeChanged(value) => {
                self.episode = value;
                if let Some(number) = NonZeroU8::new(value) {
                    self.set_episode(media_list, number)?;
                }
            }
            Msg::ChapterPathChanged(value) => {
                self.set_chapter_path(media_list, PathBuf::from(value))?;
            }
            Msg::ConfirmScreen(message) => self.confirm_screen_update(media_list, &message)?,
            Msg::ChapterPathSelect => {
                if let Some(folder) = rfd::FileDialog::new().pick_folder() {
                    self.set_chapter_path(media_list, folder)?;
                }
            }
            Msg::Warning(WarningMsg::Close) => self.warning.close(),
            Msg::OpenChapterDirectory => {
                let chapter_path = self
                    .editable_media(media_list)
                    .chapter_path()
                    .clone()
                    .into_path_buf();
                if !chapter_path.is_dir() {
                    self.warning(WarningKind::WrongChapterPath);
                    return Ok(());
                }
                open(chapter_path)?;
            }
            Msg::Back | Msg::Delete(_) | Msg::Watch { .. } => {}
        }
        Ok(())
    }

    fn watch_sign(&self, media: &[MediaHandler]) -> Option<String> {
        if self.editable_media(media).chapter_path().is_empty() {
            return None;
        }
        let watch_sign = match self.episode(media) {
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
        media: &mut [MediaHandler],
        message: &ConfirmScrnMsg,
    ) -> Result<()> {
        match message {
            ConfirmScrnMsg::Confirm => {
                if let Some(kind) = self.confirm.kind() {
                    self.confirm_kind_update(media, kind.clone())?;
                }
            }
            ConfirmScrnMsg::Cancel => self.confirm.close(),
        }
        Ok(())
    }

    fn confirm_kind_update(&mut self, media: &mut [MediaHandler], kind: ConfirmKind) -> Result<()> {
        match kind {
            ConfirmKind::SwitchToNextChapter { path } => {
                self.confirm.close();
                self.set_chapter_path(media, path)
            }
            ConfirmKind::EpisodesOverflow { .. } => {
                self.confirm.close();
                self.increase_chapter(media)
            }
        }
    }

    const fn editable_media<'a>(&self, media: &'a [MediaHandler]) -> &'a MediaHandler {
        &media[self.editable_media_id]
    }

    fn editable_media_mut<'a>(&self, media: &'a mut [MediaHandler]) -> &'a mut MediaHandler {
        &mut media[self.editable_media_id]
    }

    fn episodes(&self) -> Result<&EpisodeList> {
        let episodes = self.episodes.as_ref().map_err(Clone::clone)?;
        Ok(episodes)
    }

    fn episode(&self, media: &[MediaHandler]) -> Result<&Episode> {
        self.episodes()?
            .get(self.episode_id(media))
            .ok_or(ErrorKind::EpisodeNotFound)
    }

    const fn episode_id(&self, media: &[MediaHandler]) -> usize {
        (self.editable_media(media).episode().get() - 1) as usize
    }

    fn set_chapter_path(
        &mut self,
        media: &mut [MediaHandler],
        chapter_path: impl Into<PathBuf>,
    ) -> Result<()> {
        self.editable_media_mut(media)
            .set_chapter_path(chapter_path.into())?;
        let editable_media = self.editable_media(media);
        self.episodes = editable_media.episode_list();
        Ok(())
    }

    fn warning(&mut self, kind: WarningKind) {
        self.warning = WarningDlg::from_kind(kind);
    }

    fn is_episode_overflow(&self, value: NonZeroU8) -> bool {
        self.episodes_count()
            .is_some_and(|ec| ec < value.get() as usize)
    }

    fn set_episode(&mut self, media_list: &mut [MediaHandler], value: NonZeroU8) -> Result<()> {
        let media = self.editable_media_mut(media_list);

        match self.episodes_count() {
            Some(episodes_count) if value.get() as usize <= episodes_count => {
                self.episode = value.get();
                media.set_episode(value)?;
            }
            None => {
                self.episode = value.get();
                media.set_episode(value)?;
            }
            Some(_) => {
                if !self.is_episode_overflow(value) {
                    return Ok(());
                }
                self.episodes = media.episode_list();
                if !self.is_episode_overflow(value) {
                    return Ok(());
                }
                let Some(episodes_count) = self.episodes_count() else {
                    return Ok(());
                };
                self.confirm_episode_overflow(episodes_count);
            }
        }

        Ok(())
    }

    fn episodes_count(&self) -> Option<usize> {
        let count = self.episodes.as_ref().ok()?.len();
        Some(count)
    }

    fn increase_chapter(&mut self, media_list: &mut [MediaHandler]) -> Result<()> {
        if self.chapter == 0 {
            self.chapter = 1;
            return Ok(());
        }

        self.episode = 1;
        let media = self.editable_media_mut(media_list);
        media.set_episode_to_one();
        let next_chapter = media.chapter().saturating_add(1);
        self.chapter = next_chapter.get();
        media.set_chapter(next_chapter)?;
        if media.chapter_path().is_empty() {
            return Ok(());
        }
        let next_chapter_path = media.next_chapter_path()?;
        self.confirm_switch_to_next_chapter(next_chapter_path);
        Ok(())
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
