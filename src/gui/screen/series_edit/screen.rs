use std::{num::NonZeroU8, path::PathBuf};

use iced::{
    alignment, theme,
    widget::{button, column, container, row, text, Column, Space},
    Color, Element, Length,
};
use iced_aw::modal;

use super::{
    kind::{ConfirmKind, WarningKind},
    message::Message,
};
use crate::{
    episdoes::Episodes,
    episode::Episode,
    error::ErrorKind,
    gui::{
        screen::{ConfirmScreen, ConfirmScreenMessage},
        utils::{link, signed_text_input, square_button, DEFAULT_INDENT},
        Dialog, WarningMessage, WarningScreen,
    },
    media::Media,
    series::Series,
    utils,
};

pub struct SeriesEditScreen {
    confirm_screen: Dialog<ConfirmScreen<ConfirmKind>>,
    warning: Dialog<WarningScreen<WarningKind>>,
    editable_series_id: usize,
    episodes: Result<Episodes, ErrorKind>,
    buffer_name: String,
}

impl SeriesEditScreen {
    pub fn new(media: &[Series], editable_series_id: usize) -> Self {
        let editable_series = &media[editable_series_id];
        let editable_series_name = editable_series.name().to_string();
        let episodes = Episodes::read(editable_series.season_path());
        Self {
            confirm_screen: Dialog::closed(),
            warning: Dialog::closed(),
            editable_series_id,
            episodes,
            buffer_name: editable_series_name,
        }
    }

    pub fn view(&self, media: &[Series]) -> Element<Message> {
        let confirm_screen = self.confirm_screen.view_into();
        let series = self.editable_series(media);
        let season_path = series.season_path().display().to_string();
        let top = row![
            container(link("< Back").on_press(Message::Back)).width(Length::Fill),
            text(series.name()),
            container(
                button("Delete")
                    .style(theme::Button::Destructive)
                    .on_press(Message::Delete(self.editable_series_id))
            )
            .width(Length::Fill)
            .align_x(alignment::Horizontal::Right),
        ];
        let watch_message = self.episode(media).ok().map(|episode| Message::Watch {
            path: episode.path().to_owned(),
        });
        let watch = container(
            button("Watch")
                .style(theme::Button::Positive)
                .on_press_maybe(watch_message),
        )
        .width(Length::Fill)
        .center_x();
        let watch_sign = self.watch_sign(media).map(|watch_sign| {
            container(
                text(watch_sign)
                    .size(13)
                    .style(theme::Text::Color(Color::new(0.6, 0.6, 0.6, 1.))),
            )
            .width(Length::Fill)
            .center_x()
        });
        let body = column![
            signed_text_input("Name", &self.buffer_name, Message::NameChanged),
            row![
                signed_text_input(
                    "Season",
                    &series.season().to_string(),
                    Message::SeasonChanged
                ),
                square_button("-").on_press(Message::SeasonDec),
                square_button("+").on_press(Message::SeasonInc)
            ]
            .spacing(DEFAULT_INDENT),
            row![
                signed_text_input(
                    "Episode",
                    &series.episode().to_string(),
                    Message::EpisodeChanged
                ),
                square_button("-").on_press(Message::EpisodeDec),
                square_button("+").on_press(Message::EpisodeInc)
            ]
            .spacing(DEFAULT_INDENT),
            row![
                signed_text_input("Season path", &season_path, Message::SeasonPathChanged),
                square_button(">").on_press(Message::OpenSeasonDirectory),
                square_button("...").on_press(Message::SeasonPathSelect),
            ]
            .spacing(DEFAULT_INDENT)
        ]
        .spacing(DEFAULT_INDENT);
        let space = Space::with_height(Length::Fixed(15.0));
        let warning = self.warning.view_into();

        let mut layout = Column::new()
            .padding(DEFAULT_INDENT)
            .spacing(DEFAULT_INDENT);

        layout = layout.push(top);
        layout = layout.push(watch);
        layout = layout.push_maybe(watch_sign);
        layout = layout.push(space);
        layout = layout.push_maybe(warning);
        layout = layout.push(body);

        modal(layout, confirm_screen).into()
    }

    pub fn update(&mut self, media: &mut Media, message: Message) -> Result<(), ErrorKind> {
        match message {
            Message::Back | Message::Delete(_) | Message::Watch { .. } => {}
            Message::NameChanged(value) => {
                self.buffer_name = value.clone();
                if media.name_is_used(&value) {
                    self.warning(WarningKind::NameUsed);
                    return Ok(());
                }
                if matches!(self.warning.kind(), Some(WarningKind::NameUsed)) {
                    self.warning.close();
                }
                let series = self.editable_series_mut(media);
                series.rename(value)?;
            }
            Message::SeasonChanged(value) => {
                if value.is_empty() {
                    return self.editable_series_mut(media).set_season(NonZeroU8::MIN);
                }
                if let Ok(number) = value.parse() {
                    self.editable_series_mut(media).set_season(number)?;
                }
            }
            Message::EpisodeChanged(value) => {
                if value.is_empty() {
                    return self.set_episode_to_one(media);
                }
                if let Ok(number) = value.parse() {
                    self.set_episode(media, number)?;
                }
            }
            Message::SeasonInc => self.increase_season(media)?,
            Message::SeasonDec => {
                let series = self.editable_series_mut(media);
                let new_value = series.season().get() - 1;
                let new_value = NonZeroU8::new(new_value);
                match new_value {
                    Some(number) => series.set_season(number)?,
                    None => self.warning(WarningKind::SeasonCanNotBeZero),
                }
            }
            Message::EpisodeInc => self.increase_episode(media)?,
            Message::EpisodeDec => {
                let series = self.editable_series_mut(media);
                let new_value = series.episode().get() - 1;
                let new_value = NonZeroU8::new(new_value);
                match new_value {
                    Some(number) => self.editable_series_mut(media).set_episode(number)?,
                    None => self.warning(WarningKind::EpisodeCanNotBeZero),
                }
            }
            Message::SeasonPathChanged(value) => {
                self.set_season_path(media, PathBuf::from(value))?
            }
            Message::ConfirmScreen(message) => self.confirm_screen_update(media, message)?,
            Message::SeasonPathSelect => {
                if let Some(folder) = rfd::FileDialog::new().pick_folder() {
                    self.set_season_path(media, folder)?;
                }
            }
            Message::Warning(WarningMessage::Close) => self.warning.close(),
            Message::OpenSeasonDirectory => {
                let season_path = self.editable_series(media).season_path();
                if !season_path.is_dir() {
                    self.warning(WarningKind::WrongSeasonPath);
                    return Ok(());
                }
                utils::open(season_path)?;
            }
        }
        Ok(())
    }

    pub fn title(&self, media: &[Series]) -> String {
        self.editable_series(media).name().to_string()
    }

    fn watch_sign(&self, media: &[Series]) -> Option<String> {
        if !self.editable_series(media).season_path_is_present() {
            return None;
        }
        let watch_sign = match self.episode(media) {
            Ok(episode) => episode.name(),
            Err(ErrorKind::FSIO { kind, .. }) => format!("Season path is incorrect: {kind}"),
            Err(err) => format!("Season path is incorrect: {err}"),
        };
        Some(watch_sign)
    }

    fn confirm_screen_update(
        &mut self,
        media: &mut [Series],
        message: ConfirmScreenMessage,
    ) -> Result<(), ErrorKind> {
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

    fn confirm_kind_update(
        &mut self,
        media: &mut [Series],
        kind: ConfirmKind,
    ) -> Result<(), ErrorKind> {
        match kind {
            ConfirmKind::SwitchToNextSeason { next_season_path } => {
                self.confirm_screen.close();
                self.set_season_path(media, next_season_path)
            }
            ConfirmKind::EpisodesOverflow { .. } => {
                self.confirm_screen.close();
                self.increase_season(media)
            }
        }
    }

    fn editable_series<'a>(&'a self, media: &'a [Series]) -> &'a Series {
        &media[self.editable_series_id]
    }

    fn editable_series_mut<'a>(&'a self, media: &'a mut [Series]) -> &'a mut Series {
        &mut media[self.editable_series_id]
    }

    fn episodes(&self) -> Result<&Episodes, ErrorKind> {
        self.episodes.as_ref().map_err(Clone::clone)
    }

    fn episode(&self, media: &[Series]) -> Result<&Episode, ErrorKind> {
        Ok(&self.episodes()?[self.episode_id(media)])
    }

    fn episode_id(&self, media: &[Series]) -> usize {
        (self.editable_series(media).episode().get() - 1) as usize
    }

    fn set_season_path(
        &mut self,
        media: &mut [Series],
        season_path: PathBuf,
    ) -> Result<(), ErrorKind> {
        self.editable_series_mut(media)
            .set_season_path(season_path)?;
        self.episodes = {
            let editable_series = self.editable_series(media);
            let series_path = editable_series.season_path();
            Episodes::read(series_path)
        };
        Ok(())
    }

    fn warning(&mut self, kind: WarningKind) {
        let screen = WarningScreen::new(kind);
        self.warning = Dialog::new(screen);
    }

    fn increase_episode(&mut self, media: &mut [Series]) -> Result<(), ErrorKind> {
        let series = self.editable_series(media);
        let next_episode = series.episode().saturating_add(1);
        self.set_episode(media, next_episode)
    }

    fn set_episode(&mut self, media: &mut [Series], value: NonZeroU8) -> Result<(), ErrorKind> {
        let series = self.editable_series_mut(media);
        if !series.season_path_is_present() || value <= series.episode() {
            series.set_episode(value)?;
            return Ok(());
        }
        let Some(episodes_count) = self.episodes_count() else {
            return Ok(());
        };
        if episodes_count < value.get() as usize {
            self.confirm_episode_overflow(episodes_count);
            return Ok(());
        }
        let series = self.editable_series_mut(media);
        series.set_episode(value)
    }

    fn episodes_count(&self) -> Option<usize> {
        let count = self.episodes.as_ref().ok()?.len();
        Some(count)
    }

    fn set_episode_to_one(&mut self, media: &mut [Series]) -> Result<(), ErrorKind> {
        let series = self.editable_series_mut(media);
        series.set_episode(NonZeroU8::MIN)
    }

    fn increase_season(&mut self, media: &mut [Series]) -> Result<(), ErrorKind> {
        self.set_episode_to_one(media)?;
        let series = self.editable_series_mut(media);
        let next_season = series.season().saturating_add(1);
        series.set_season(next_season)?;
        if !series.season_path_is_present() {
            return Ok(());
        }
        let next_dir = utils::next_dir(series.season_path())?;
        self.confirm_switch_to_next_season(next_dir);
        Ok(())
    }

    fn confirm(&mut self, kind: ConfirmKind) {
        let confirm = ConfirmScreen::new(kind);
        self.confirm_screen = Dialog::new(confirm);
    }

    fn confirm_switch_to_next_season(&mut self, next_season_path: PathBuf) {
        let kind = ConfirmKind::switch_to_next_season(next_season_path);
        self.confirm(kind);
    }

    fn confirm_episode_overflow(&mut self, episodes_count: usize) {
        let kind = ConfirmKind::episode_overflow(episodes_count);
        self.confirm(kind);
    }
}
