use std::{
    cell::RefCell,
    fmt::Display,
    num::NonZeroU8,
    path::{Path, PathBuf},
    rc::Rc,
};

use iced::{
    theme,
    widget::{button, column, container, horizontal_space, row, text, Column, Space},
    Color, Element, Length,
};
use iced_aw::modal;
use mime_guess::mime;

use crate::{
    error::ErrorKind,
    gui::{ConfirmScreen, ConfirmScreenMessage, Dialog, WarningMessage, WarningPopUp},
    series::Series,
    utils::{self, read_dir},
    view_utils::{link, signed_text_imput, square_button, DEFAULT_INDENT},
};

#[derive(Debug, Clone)]
pub enum Message {
    Back,
    Delete(usize),
    Watch { path: PathBuf },
    NameChanged(String),
    SeasonChanged(String),
    EpisodeChanged(String),
    SeasonPathChanged(String),
    SeasonPathSelect,
    SeasonInc,
    SeasonDec,
    EpisodeInc,
    EpisodeDec,
    ConfirmScreen(ConfirmScreenMessage),
    Warning(WarningMessage),
}

pub struct SeriesEditScreen {
    media: Vec<Rc<RefCell<Series>>>,
    confirm_screen: Dialog<ConfirmScreen<ConfirmKind>>,
    warning: Dialog<WarningPopUp<WarningKind>>,
    editable_series_id: usize,
    episode_paths: Option<Vec<PathBuf>>,
    buffer_name: String,
}

impl SeriesEditScreen {
    pub fn new(
        series: Vec<Rc<RefCell<Series>>>,
        editable_series_id: usize,
    ) -> Result<Self, ErrorKind> {
        let editable_series = &series[editable_series_id];
        let editable_series_name = editable_series.borrow().name().to_string();
        let episode_paths = episode_paths(editable_series.borrow().season_path())?;
        Ok(Self {
            confirm_screen: Dialog::closed(),
            media: series,
            editable_series_id,
            warning: Dialog::closed(),
            episode_paths,
            buffer_name: editable_series_name,
        })
    }

    pub fn view(&self) -> Element<Message> {
        let confirm_screen = self.confirm_screen.view_into();
        let series = self.editable_series().borrow();
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
            .align_x(iced::alignment::Horizontal::Right),
        ];
        let episode_name = self.episode_name();
        let watch = row![
            horizontal_space(),
            button("Watch")
                .style(theme::Button::Positive)
                .on_press_maybe(episode_name.clone().ok().flatten().map(|episode_name| {
                    Message::Watch {
                        path: self
                            .editable_series()
                            .borrow()
                            .season_path()
                            .join(&episode_name),
                    }
                })),
            horizontal_space()
        ];
        let watch_sign = match episode_name {
            Ok(Some(name)) => name,
            Ok(None) => "Select correct season path to watch".to_string(),
            Err(err) => err.to_string(),
        };
        let watch_sign = row![
            horizontal_space(),
            text(watch_sign)
                .size(13)
                .style(theme::Text::Color(Color::new(0.6, 0.6, 0.6, 1.))),
            horizontal_space()
        ];
        let body = column![
            signed_text_imput("Name", &self.buffer_name, Message::NameChanged),
            row![
                signed_text_imput(
                    "Season",
                    &series.season().to_string(),
                    Message::SeasonChanged
                ),
                square_button("-").on_press(Message::SeasonDec),
                square_button("+").on_press(Message::SeasonInc)
            ]
            .spacing(DEFAULT_INDENT),
            row![
                signed_text_imput(
                    "Episode",
                    &series.episode().to_string(),
                    Message::EpisodeChanged
                ),
                square_button("-").on_press(Message::EpisodeDec),
                square_button("+").on_press(Message::EpisodeInc)
            ]
            .spacing(DEFAULT_INDENT),
            row![
                signed_text_imput("Season path", &season_path, Message::SeasonPathChanged),
                square_button("...").on_press(Message::SeasonPathSelect),
            ]
            .spacing(DEFAULT_INDENT)
        ]
        .spacing(DEFAULT_INDENT);
        let space = Space::with_height(Length::Fixed(15.0));
        let mut layout = Column::new()
            .padding(DEFAULT_INDENT)
            .spacing(DEFAULT_INDENT);

        layout = layout.push(top);
        layout = layout.push(watch);
        layout = layout.push(watch_sign);
        layout = layout.push(space);
        layout = layout.push_maybe(self.warning.view_into());
        layout = layout.push(body);

        modal(layout, confirm_screen).into()
    }

    pub fn update(&mut self, message: Message) -> Result<(), ErrorKind> {
        match message {
            Message::Back | Message::Delete(_) | Message::Watch { .. } => {}
            Message::NameChanged(value) => {
                self.buffer_name = value.clone();
                let name_is_used = self.media.iter().any(|s| s.borrow().name() == &value);
                if name_is_used {
                    self.warning(WarningKind::NameUsed);
                    return Ok(());
                }
                if let Some(WarningKind::NameUsed) = self.warning.as_ref().map(|w| w.kind()) {
                    self.warning.close();
                }
                let episode = self.editable_series();
                episode.borrow_mut().rename(value)?;
            }
            Message::SeasonChanged(value) => {
                if let Ok(number) = value.parse() {
                    self.editable_series().borrow_mut().set_season(number)?;
                }
            }
            Message::EpisodeChanged(value) => {
                if let Ok(number) = value.parse() {
                    self.set_episode(number)?;
                }
            }
            Message::SeasonInc => self.increase_season()?,
            Message::SeasonDec => {
                let series = self.editable_series();
                let new_value = series.borrow().season().get() - 1;
                let new_value = NonZeroU8::new(new_value);
                match new_value {
                    Some(number) => series.borrow_mut().set_season(number)?,
                    None => self.warning(WarningKind::SeasonCanNotBeZero),
                }
            }
            Message::EpisodeInc => self.increase_episode()?,
            Message::EpisodeDec => {
                let series = self.editable_series();
                let new_value = series.borrow().episode().get() - 1;
                let new_value = NonZeroU8::new(new_value);
                match new_value {
                    Some(number) => self.editable_series().borrow_mut().set_episode(number)?,
                    None => self.warning(WarningKind::EpisodeCanNotBeZero),
                }
            }
            Message::SeasonPathChanged(value) => self.set_season_path(PathBuf::from(value))?,
            Message::ConfirmScreen(message) => match message {
                ConfirmScreenMessage::Confirm => {
                    let Some(confirm) = self.confirm_screen.take() else {
                        return Ok(());
                    };
                    match confirm.take() {
                        ConfirmKind::TrySwitchToNewSeason { season_path } => {
                            self.set_season_path(season_path)?;
                        }
                        ConfirmKind::EpisodesOverflow { .. } => self.increase_season()?,
                    }
                }
                ConfirmScreenMessage::Cancel => self.confirm_screen.close(),
            },
            Message::SeasonPathSelect => {
                if let Some(folder) = rfd::FileDialog::new().pick_folder() {
                    self.set_season_path(folder)?;
                }
            }
            Message::Warning(message) => match message {
                WarningMessage::Close => self.warning.close(),
            },
        }
        Ok(())
    }

    pub fn title(&self) -> String {
        self.editable_series().borrow().name().to_string()
    }

    fn editable_series(&self) -> &Rc<RefCell<Series>> {
        &self.media[self.editable_series_id]
    }

    fn episode_path(&self) -> Result<Option<PathBuf>, Error> {
        match self.episode_paths.as_ref() {
            Some(ep) => {
                if ep.is_empty() {
                    return Err(Error::EpisodesDidNotFound);
                }
                Ok(Some(ep[self.episode_id()].clone()))
            }
            None => Ok(None),
        }
    }

    fn episode_id(&self) -> usize {
        self.editable_series().borrow().episode().get() as usize - 1
    }

    fn episode_name(&self) -> Result<Option<String>, Error> {
        Ok(self.episode_path()?.map(|p| {
            p.file_name()
                .unwrap_or_default()
                .to_str()
                .unwrap_or_default()
                .to_string()
        }))
    }

    fn set_season_path(&mut self, season_path: PathBuf) -> Result<(), ErrorKind> {
        self.editable_series()
            .borrow_mut()
            .set_season_path(season_path)?;
        self.update_edpisode_paths()?;
        Ok(())
    }

    fn update_edpisode_paths(&mut self) -> Result<(), ErrorKind> {
        self.episode_paths = {
            let editable_series = self.editable_series().borrow();
            let series_path = editable_series.season_path();
            episode_paths(series_path)?
        };
        Ok(())
    }

    fn warning(&mut self, kind: WarningKind) {
        let pop_up = WarningPopUp::new(kind);
        self.warning = Dialog::new(pop_up);
    }

    fn increase_episode(&mut self) -> Result<(), ErrorKind> {
        let series = self.editable_series();
        let next_episode = series.borrow().episode().saturating_add(1);
        self.set_episode(next_episode)
    }

    fn set_episode(&mut self, value: NonZeroU8) -> Result<(), ErrorKind> {
        let series = self.editable_series();
        if !series.borrow().season_path_is_present() || value <= series.borrow().episode() {
            series.borrow_mut().set_episode(value)?;
            return Ok(());
        }
        let Some(episodes_count) = self.episodes_count() else {
            return Ok(());
        };
        if episodes_count < value.get() as usize {
            self.confirm(ConfirmKind::EpisodesOverflow {
                series_on_disk: episodes_count,
            });
            return Ok(());
        }
        let series = self.editable_series();
        series.borrow_mut().set_episode(value)?;
        Ok(())
    }

    fn episodes_count(&self) -> Option<usize> {
        self.episode_paths.as_ref().map(|p| p.len())
    }

    fn season_path(&self) -> Result<PathBuf, ErrorKind> {
        let series = self.editable_series();
        let season_path = series.borrow().season_path().to_path_buf();
        if !season_path.exists() {
            return Err(ErrorKind::SeasonPathDidNotExists { season_path });
        }
        Ok(season_path)
    }

    fn set_episode_to_one(&mut self) -> Result<(), ErrorKind> {
        let series = self.editable_series();
        series.borrow_mut().set_episode(NonZeroU8::MIN)
    }

    fn increase_season(&mut self) -> Result<(), ErrorKind> {
        self.set_episode_to_one()?;
        let series = self.editable_series();
        let next_season = series.borrow().season().saturating_add(1);
        series.borrow_mut().set_season(next_season)?;
        if !series.borrow().season_path_is_present() {
            return Ok(());
        }
        let season_path = self.season_path()?;
        let next_season_path =
            next_dir(&season_path)?.ok_or(ErrorKind::FailedToFindNextSeasonPath)?;
        self.confirm(ConfirmKind::TrySwitchToNewSeason {
            season_path: next_season_path,
        });
        Ok(())
    }

    fn confirm(&mut self, kind: ConfirmKind) {
        let confirm = ConfirmScreen::new(kind);
        self.confirm_screen = Dialog::new(confirm);
    }
}

fn episode_paths(series_path: impl AsRef<Path>) -> Result<Option<Vec<PathBuf>>, ErrorKind> {
    let mut episode_paths = series_path
        .as_ref()
        .exists()
        .then(|| read_dir(series_path))
        .transpose()?;
    if let Some(episode_paths) = &mut episode_paths {
        episode_paths.retain(|p| {
            let mime = mime_guess::from_path(p);
            match mime.first() {
                Some(mime) => {
                    let mtype = mime.type_();
                    mtype == mime::VIDEO || mtype == mime::AUDIO
                }
                None => false,
            }
        });
        episode_paths.sort();
    }
    Ok(episode_paths)
}

fn next_dir(path: impl AsRef<Path>) -> Result<Option<PathBuf>, ErrorKind> {
    let path = path.as_ref();
    let dir_name = path
        .file_name()
        .ok_or(ErrorKind::FailedToFindNextSeasonPath)?;
    let parent = path
        .parent()
        .ok_or(ErrorKind::parent_dir(&dir_name))?
        .to_owned();
    let paths = utils::read_dir_sort(parent)?;
    let dirs: Vec<_> = paths.into_iter().filter(|path| path.is_dir()).collect();
    let mut season_dir_index = None;
    for (i, dir) in dirs.iter().enumerate() {
        let dir = dir
            .file_name()
            .ok_or(ErrorKind::FailedToFindNextSeasonPath)?
            .to_str()
            .ok_or(ErrorKind::FailedToFindNextSeasonPath)?;
        if dir_name == dir {
            season_dir_index = Some(i);
        }
    }
    let Some(season_dir_index) = season_dir_index else {
        return Ok(None);
    };
    let next_season_index = season_dir_index + 1;
    if next_season_index >= dirs.len() {
        return Ok(None);
    }
    let next_dir = dirs[next_season_index].to_path_buf();
    Ok(Some(next_dir))
}

enum ConfirmKind {
    TrySwitchToNewSeason { season_path: PathBuf },
    EpisodesOverflow { series_on_disk: usize },
}

impl Display for ConfirmKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfirmKind::TrySwitchToNewSeason { season_path } => {
                write!(f, "Proposed path to next season: {}", season_path.display())
            }
            ConfirmKind::EpisodesOverflow { series_on_disk } => write!(
                f,
                "Seems like {} series is a last of it season. Switch to the next season?",
                series_on_disk
            ),
        }
    }
}

enum WarningKind {
    SeasonCanNotBeZero,
    EpisodeCanNotBeZero,
    NameUsed,
}

impl Display for WarningKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WarningKind::SeasonCanNotBeZero => write!(f, "Season can not be zero"),
            WarningKind::EpisodeCanNotBeZero => write!(f, "Episode can not be zero"),
            WarningKind::NameUsed => write!(f, "Name must be unic"),
        }
    }
}

impl From<ConfirmScreenMessage> for Message {
    fn from(value: ConfirmScreenMessage) -> Self {
        Self::ConfirmScreen(value)
    }
}

impl From<WarningMessage> for Message {
    fn from(value: WarningMessage) -> Self {
        Self::Warning(value)
    }
}

#[derive(Debug, Clone, thiserror::Error)]
enum Error {
    #[error("Episodes didn't found")]
    EpisodesDidNotFound,
}
