/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::{num::NonZeroU8, path::PathBuf};

use iced::{
    Alignment, Element, Length,
    widget::{Column, Stack, button, column, container, row, text},
};

use super::{
    kind::{ConfirmKind, WarningKind},
    message::Msg,
};
use crate::{
    gui::{
        Icon,
        screen::{ConfirmDlg, ConfirmScrnMsg, WarningDlg, WarningMsg},
        utils::{GRAY, INDENT, LONG_INDENT, link, signed_text_input, square_button},
    },
    model::{Episode, EpisodeList, ErrorKind, MediaHandler, MediaList, Result},
    open,
};

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
        let editable_episode_name = editable_media.name().to_string();
        let episodes = editable_media.episode_list();
        Self {
            confirm: ConfirmDlg::closed(),
            warning: WarningDlg::closed(),
            editable_media_id,
            episodes,
            buffer_name: editable_episode_name,
            chapter: editable_media.chapter().get(),
            episode: editable_media.episode().get(),
        }
    }

    pub fn view<'a>(&'a self, media_list: &'a [MediaHandler]) -> Element<'a, Msg> {
        let confirm_screen = self.confirm.view_into();
        let media = self.editable_media(media_list);
        let chapter_path = media.chapter_path().display().to_string();
        let top = row![
            container(link("< Back").on_press(Msg::Back)).width(Length::Fill),
            text(media.name()),
            container(
                button("Delete")
                    .style(button::danger)
                    .on_press(Msg::Delete(self.editable_media_id))
            )
            .width(Length::Fill)
            .align_x(Alignment::End),
        ];
        let watch = container(
            button("Watch").style(button::success).on_press_maybe(
                self.episode(media_list)
                    .ok()
                    .map(Episode::path)
                    .map(Msg::watch),
            ),
        )
        .width(Length::Fill)
        .align_x(Alignment::Center);
        let watch_sign = self.watch_sign(media_list).map(|watch_sign| {
            container(text(watch_sign).size(13).color(GRAY))
                .width(Length::Fill)
                .align_x(Alignment::Center)
        });
        let body = column![
            signed_text_input("Name", &self.buffer_name, Msg::NameChanged),
            row![
                signed_text_input("Chapter", &self.chapter.to_string(), Msg::ChapterChanged),
                square_button("-").on_press(Msg::ChapterDec),
                square_button("+").on_press(Msg::ChapterInc)
            ]
            .spacing(INDENT),
            row![
                signed_text_input("Episode", &self.episode.to_string(), Msg::EpisodeChanged),
                square_button("-").on_press(Msg::EpisodeDec),
                square_button("+").on_press(Msg::EpisodeInc)
            ]
            .spacing(INDENT),
            row![
                signed_text_input("Chapter path", &chapter_path, Msg::ChapterPathChanged),
                Icon::open_folder()
                    .button()
                    .on_press(Msg::OpenChapterDirectory),
                Icon::triple_dot().button().on_press(Msg::ChapterPathSelect),
            ]
            .spacing(INDENT)
        ]
        .spacing(INDENT);
        let warning = self.warning.view_into();

        let layout = Column::new()
            .push(top)
            .push(watch)
            .push_maybe(watch_sign)
            .push_maybe(warning)
            .push(body)
            .padding(LONG_INDENT)
            .spacing(LONG_INDENT);

        Stack::new().push(layout).push_maybe(confirm_screen).into()
    }

    pub fn update(&mut self, media_list: &mut MediaList, message: Msg) -> Result<()> {
        match message {
            Msg::Back | Msg::Delete(_) | Msg::Watch { .. } => {}
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
                if value.is_empty() {
                    // TODO: Warning: Chapter can not be zero
                    self.chapter = 0;
                    self.editable_media_mut(media_list).set_chapter_to_one();
                    return Ok(());
                }
                if let Ok(number) = value.parse::<NonZeroU8>() {
                    self.chapter = number.get();
                    self.editable_media_mut(media_list).set_chapter(number)?;
                }
            }
            Msg::EpisodeChanged(value) => {
                if value.is_empty() {
                    // TODO: Warning: Episode can not be zero
                    self.episode = 0;
                    self.editable_media_mut(media_list).set_episode_to_one();
                }
                if let Ok(number) = value.parse::<NonZeroU8>() {
                    self.episode = number.get();
                    self.set_episode(media_list, number)?;
                }
            }
            Msg::ChapterInc => self.increase_chapter(media_list)?,
            Msg::ChapterDec => {
                let new_value = self.editable_media(media_list).chapter().get() - 1;
                self.chapter = new_value;
                if let Some(number) = NonZeroU8::new(new_value) {
                    self.editable_media_mut(media_list).set_chapter(number)?;
                }
            }
            Msg::EpisodeInc => self.increase_episode(media_list)?,
            Msg::EpisodeDec => {
                let media = self.editable_media_mut(media_list);
                let new_value = media.episode().get() - 1;
                self.episode = new_value;
                if let Some(number) = NonZeroU8::new(new_value) {
                    self.editable_media_mut(media_list).set_episode(number)?;
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
                let chapter_path = self.editable_media(media_list).chapter_path();
                if !chapter_path.is_dir() {
                    self.warning(WarningKind::WrongChapterPath);
                    return Ok(());
                }
                open(chapter_path)?;
            }
        }
        Ok(())
    }

    pub fn title(&self, media: &[MediaHandler]) -> String {
        self.editable_media(media).name().to_string()
    }

    fn watch_sign(&self, media: &[MediaHandler]) -> Option<String> {
        if !self.editable_media(media).chapter_path_is_present() {
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

    const fn editable_media<'a>(&'a self, media: &'a [MediaHandler]) -> &'a MediaHandler {
        &media[self.editable_media_id]
    }

    fn editable_media_mut<'a>(&'a self, media: &'a mut [MediaHandler]) -> &'a mut MediaHandler {
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
            .set_chapter_path(chapter_path)?;
        let editable_media = self.editable_media(media);
        self.episodes = editable_media.episode_list();
        Ok(())
    }

    fn warning(&mut self, kind: WarningKind) {
        self.warning = WarningDlg::from_kind(kind);
    }

    fn increase_episode(&mut self, media_list: &mut [MediaHandler]) -> Result<()> {
        if self.episode == 0 {
            self.episode = 1;
            return Ok(());
        }

        let media = self.editable_media(media_list);
        let next_episode = media.episode().saturating_add(1);
        self.episode = next_episode.get();
        self.set_episode(media_list, next_episode)
    }

    fn is_episode_overflow(&self, value: NonZeroU8) -> bool {
        self.episodes_count()
            .is_some_and(|ec| ec < value.get() as usize)
    }

    fn set_episode(&mut self, media_list: &mut [MediaHandler], value: NonZeroU8) -> Result<()> {
        self.episode = value.get();

        let media = self.editable_media_mut(media_list);
        if !media.chapter_path_is_present() || value <= media.episode() {
            media.set_episode(value)?;
            return Ok(());
        }

        if self.is_episode_overflow(value) {
            self.episodes = media.episode_list();
            if self.is_episode_overflow(value) {
                let Some(episodes_count) = self.episodes_count() else {
                    return Ok(());
                };
                self.confirm_episode_overflow(episodes_count);
                return Ok(());
            }
        }

        let media = self.editable_media_mut(media_list);
        media.set_episode(value)?;
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
        self.editable_media_mut(media_list).set_episode_to_one();
        let next_chapter = self.editable_media(media_list).chapter().saturating_add(1);
        self.chapter = next_chapter.get();
        let media = self.editable_media_mut(media_list);
        media.set_chapter(next_chapter)?;
        if !media.chapter_path_is_present() {
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
