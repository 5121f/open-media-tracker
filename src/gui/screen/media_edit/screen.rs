/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::{num::NonZeroU8, path::PathBuf};

use iced::{
    alignment::{self, Horizontal},
    widget::{button, column, container, row, stack, text, Column},
    Element, Length,
};

use super::{
    kind::{ConfirmKind, WarningKind},
    message::Message,
};
use crate::{
    gui::{
        icon::{self, Icon},
        screen::{ConfirmScreen, ConfirmScreenMessage},
        utils::{link, signed_text_input, square_button, GRAY_TEXT, INDENT, PADDING},
        Dialog, WarningMessage, WarningScreen,
    },
    model::{Episode, EpisodeList, ErrorKind, FSIOError, MediaHandler, MediaList, Result},
    utils,
};

pub struct MediaEditScreen {
    confirm_screen: Dialog<ConfirmScreen<ConfirmKind>>,
    warning: Dialog<WarningScreen<WarningKind>>,
    editable_media_id: usize,
    episodes: Result<EpisodeList>,
    buffer_name: String,
}

impl MediaEditScreen {
    pub fn new(media: &[MediaHandler], editable_media_id: usize) -> Self {
        let editable_media = &media[editable_media_id];
        let editable_episode_name = editable_media.name().to_string();
        let episodes = EpisodeList::read(editable_media.chapter_path()).map_err(Into::into);
        Self {
            confirm_screen: Dialog::closed(),
            warning: Dialog::closed(),
            editable_media_id,
            episodes,
            buffer_name: editable_episode_name,
        }
    }

    pub fn view<'a>(&'a self, media_list: &'a [MediaHandler]) -> Element<Message> {
        let confirm_screen = self.confirm_screen.view_into();
        let media = self.editable_media(media_list);
        let chapter_path = media.chapter_path().display().to_string();
        let top = row![
            container(link("< Back").on_press(Message::Back)).width(Length::Fill),
            text(media.name()),
            container(
                button("Delete")
                    .style(button::danger)
                    .on_press(Message::Delete(self.editable_media_id))
            )
            .width(Length::Fill)
            .align_x(alignment::Horizontal::Right),
        ];
        let watch_message = self.episode(media_list).ok().map(|episode| Message::Watch {
            path: episode.path().to_owned(),
        });
        let watch = container(
            button("Watch")
                .style(button::success)
                .on_press_maybe(watch_message),
        )
        .width(Length::Fill)
        .align_x(Horizontal::Center);
        let watch_sign = self.watch_sign(media_list).map(|watch_sign| {
            container(text(watch_sign).size(13).color(GRAY_TEXT))
                .width(Length::Fill)
                .align_x(Horizontal::Center)
        });
        let body = column![
            signed_text_input("Name", &self.buffer_name, Message::NameChanged),
            row![
                signed_text_input(
                    "Chapter",
                    &media.chapter().to_string(),
                    Message::ChapterChanged
                ),
                square_button("-").on_press(Message::ChapterDec),
                square_button("+").on_press(Message::ChapterInc)
            ]
            .spacing(INDENT),
            row![
                signed_text_input(
                    "Episode",
                    &media.episode().to_string(),
                    Message::EpisodeChanged
                ),
                square_button("-").on_press(Message::EpisodeDec),
                square_button("+").on_press(Message::EpisodeInc)
            ]
            .spacing(INDENT),
            row![
                signed_text_input("Chapter path", &chapter_path, Message::ChapterPathChanged),
                icon::button(Icon::open_folder()).on_press(Message::OpenChapterDirectory),
                icon::button(Icon::triple_dot()).on_press(Message::ChapterPathSelect),
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
            .padding(PADDING)
            .spacing(PADDING);

        if let Some(confirm_screen) = confirm_screen {
            return stack![layout, confirm_screen].into();
        }

        layout.into()
    }

    pub fn update(&mut self, media_list: &mut MediaList, message: Message) -> Result<()> {
        match message {
            Message::Back | Message::Delete(_) | Message::Watch { .. } => {}
            Message::NameChanged(value) => {
                self.buffer_name = value.clone();
                let rename_res = media_list.rename_media(self.editable_media_id, value);
                if matches!(rename_res, Err(ErrorKind::MediaNameIsUsed { .. })) {
                    self.warning(WarningKind::NameUsed);
                    return Ok(());
                }
                if matches!(self.warning.kind(), Some(WarningKind::NameUsed)) {
                    self.warning.close();
                }
            }
            Message::ChapterChanged(value) => {
                if value.is_empty() {
                    self.editable_media_mut(media_list)
                        .set_chapter(NonZeroU8::MIN)?;
                    return Ok(());
                }
                if let Ok(number) = value.parse() {
                    self.editable_media_mut(media_list).set_chapter(number)?;
                }
            }
            Message::EpisodeChanged(value) => {
                if value.is_empty() {
                    return self.set_episode_to_one(media_list);
                }
                if let Ok(number) = value.parse() {
                    self.set_episode(media_list, number)?;
                }
            }
            Message::ChapterInc => self.increase_chapter(media_list)?,
            Message::ChapterDec => {
                let media = self.editable_media_mut(media_list);
                let new_value = media.chapter().get() - 1;
                let new_value = NonZeroU8::new(new_value);
                match new_value {
                    Some(number) => media.set_chapter(number)?,
                    None => self.warning(WarningKind::ChapterCanNotBeZero),
                }
            }
            Message::EpisodeInc => self.increase_episode(media_list)?,
            Message::EpisodeDec => {
                let media = self.editable_media_mut(media_list);
                let new_value = media.episode().get() - 1;
                let new_value = NonZeroU8::new(new_value);
                match new_value {
                    Some(number) => self.editable_media_mut(media_list).set_episode(number)?,
                    None => self.warning(WarningKind::EpisodeCanNotBeZero),
                }
            }
            Message::ChapterPathChanged(value) => {
                self.set_chapter_path(media_list, PathBuf::from(value))?
            }
            Message::ConfirmScreen(message) => self.confirm_screen_update(media_list, message)?,
            Message::ChapterPathSelect => {
                if let Some(folder) = rfd::FileDialog::new().pick_folder() {
                    self.set_chapter_path(media_list, folder)?;
                }
            }
            Message::Warning(WarningMessage::Close) => self.warning.close(),
            Message::OpenChapterDirectory => {
                let chapter_path = self.editable_media(media_list).chapter_path();
                if !chapter_path.is_dir() {
                    self.warning(WarningKind::WrongChapterPath);
                    return Ok(());
                }
                utils::open(chapter_path)?;
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
            Err(ErrorKind::Fsio(FSIOError { kind, .. })) => {
                format!("Chapter path is incorrect: {kind}")
            }
            Err(err) => format!("Chapter path is incorrect: {err}"),
        };
        Some(watch_sign)
    }

    fn confirm_screen_update(
        &mut self,
        media: &mut [MediaHandler],
        message: ConfirmScreenMessage,
    ) -> Result<()> {
        match message {
            ConfirmScreenMessage::Confirm => {
                if let Some(kind) = self.confirm_screen.kind() {
                    self.confirm_kind_update(media, kind.clone())?
                }
            }
            ConfirmScreenMessage::Cancel => self.confirm_screen.close(),
        }
        Ok(())
    }

    fn confirm_kind_update(&mut self, media: &mut [MediaHandler], kind: ConfirmKind) -> Result<()> {
        match kind {
            ConfirmKind::SwitchToNextChapter { path } => {
                self.confirm_screen.close();
                self.set_chapter_path(media, path)
            }
            ConfirmKind::EpisodesOverflow { .. } => {
                self.confirm_screen.close();
                self.increase_chapter(media)
            }
        }
    }

    fn editable_media<'a>(&'a self, media: &'a [MediaHandler]) -> &'a MediaHandler {
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

    fn episode_id(&self, media: &[MediaHandler]) -> usize {
        (self.editable_media(media).episode().get() - 1) as usize
    }

    fn set_chapter_path(
        &mut self,
        media: &mut [MediaHandler],
        chapter_path: PathBuf,
    ) -> Result<()> {
        self.editable_media_mut(media)
            .set_chapter_path(chapter_path)?;
        self.episodes = {
            let editable_media = self.editable_media(media);
            let media_path = editable_media.chapter_path();
            EpisodeList::read(media_path).map_err(Into::into)
        };
        Ok(())
    }

    fn warning(&mut self, kind: WarningKind) {
        let screen = WarningScreen::new(kind);
        self.warning = Dialog::new(screen);
    }

    fn increase_episode(&mut self, media_list: &mut [MediaHandler]) -> Result<()> {
        let media = self.editable_media(media_list);
        let next_episode = media.episode().saturating_add(1);
        self.set_episode(media_list, next_episode)
    }

    fn set_episode(&mut self, media_list: &mut [MediaHandler], value: NonZeroU8) -> Result<()> {
        let media = self.editable_media_mut(media_list);
        if !media.chapter_path_is_present() || value <= media.episode() {
            media.set_episode(value)?;
            return Ok(());
        }
        let Some(episodes_count) = self.episodes_count() else {
            return Ok(());
        };
        if episodes_count < value.get() as usize {
            self.confirm_episode_overflow(episodes_count);
            return Ok(());
        }
        let media = self.editable_media_mut(media_list);
        media.set_episode(value)?;
        Ok(())
    }

    fn episodes_count(&self) -> Option<usize> {
        let count = self.episodes.as_ref().ok()?.len();
        Some(count)
    }

    fn set_episode_to_one(&mut self, media_list: &mut [MediaHandler]) -> Result<()> {
        let media = self.editable_media_mut(media_list);
        media.set_episode(NonZeroU8::MIN)?;
        Ok(())
    }

    fn increase_chapter(&mut self, media_list: &mut [MediaHandler]) -> Result<()> {
        self.set_episode_to_one(media_list)?;
        let media = self.editable_media_mut(media_list);
        let next_chapter = media.chapter().saturating_add(1);
        media.set_chapter(next_chapter)?;
        if !media.chapter_path_is_present() {
            return Ok(());
        }
        let next_chapter_path = media.next_chapter_path()?;
        self.confirm_switch_to_next_chapter(next_chapter_path);
        Ok(())
    }

    fn confirm(&mut self, kind: ConfirmKind) {
        let confirm = ConfirmScreen::new(kind);
        self.confirm_screen = Dialog::new(confirm);
    }

    fn confirm_switch_to_next_chapter(&mut self, next_chapter_path: PathBuf) {
        let kind = ConfirmKind::switch_to_next_chapter(next_chapter_path);
        self.confirm(kind);
    }

    fn confirm_episode_overflow(&mut self, episodes_count: usize) {
        let kind = ConfirmKind::episode_overflow(episodes_count);
        self.confirm(kind);
    }
}
