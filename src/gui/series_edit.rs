use std::{
    cell::RefCell,
    fmt::Display,
    num::NonZeroU8,
    path::{Path, PathBuf},
    rc::Rc,
};

use iced::{
    theme,
    widget::{button, column, horizontal_space, row, text, Column, Space},
    Element, Length,
};
use iced_aw::modal;

use crate::{
    error::{Error, ErrorKind},
    gui::{ConfirmScreen, ConfirmScreenMessage, Dialog, WarningMessage, WarningPopUp},
    series::Series,
    utils::{self, read_dir},
    view_utils::{link, signed_text_imput, square_button, DEFAULT_INDENT},
};

#[derive(Debug, Clone)]
pub enum Message {
    Back,
    Delete(usize),
    Watch { path: String, episode: usize },
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
    episodes_on_disk: Option<usize>,
    editable_series_id: usize,
    buffer_name: String,
}

impl SeriesEditScreen {
    pub fn new(series: Vec<Rc<RefCell<Series>>>, editable_series_id: usize) -> Self {
        let editable_series_name = {
            let editable_series = series[editable_series_id].borrow();
            editable_series.name().to_string()
        };
        Self {
            confirm_screen: Dialog::closed(),
            episodes_on_disk: None,
            media: series,
            editable_series_id,
            warning: Dialog::closed(),
            buffer_name: editable_series_name,
        }
    }

    pub fn view(&self) -> Element<Message> {
        let confirm_screen = self.confirm_screen.view_into();
        let series = self.editable_series().borrow();
        let season_path = series.season_path().display().to_string();
        let top = row![
            link("< Back").on_press(Message::Back),
            horizontal_space(),
            text(series.name()),
            horizontal_space(),
            button("Delete")
                .style(theme::Button::Destructive)
                .on_press(Message::Delete(self.editable_series_id)),
        ];
        let watch = row![
            horizontal_space(),
            button("Watch")
                .style(theme::Button::Positive)
                .on_press(Message::Watch {
                    path: season_path.clone(),
                    episode: series.episode().get() as usize - 1
                }),
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
        layout = layout.push(space);
        layout = layout.push_maybe(self.warning.view_into());
        layout = layout.push(body);

        modal(layout, confirm_screen).into()
    }

    pub fn update(&mut self, message: Message) -> Result<(), Error> {
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
            Message::SeasonPathChanged(value) => self
                .editable_series()
                .borrow_mut()
                .set_season_path(PathBuf::from(value))?,
            Message::ConfirmScreen(message) => match message {
                ConfirmScreenMessage::Confirm => {
                    let Some(confirm) = self.confirm_screen.take() else {
                        return Ok(());
                    };
                    match confirm.take() {
                        ConfirmKind::TrySwitchToNewSeason { season_path } => {
                            self.editable_series()
                                .borrow_mut()
                                .set_season_path(season_path)?;
                            self.confirm_screen.close();
                        }
                        ConfirmKind::EpisodesOverflow { .. } => self.increase_season()?,
                    }
                }
                ConfirmScreenMessage::Cancel => self.confirm_screen.close(),
            },
            Message::SeasonPathSelect => {
                if let Some(folder) = rfd::FileDialog::new().pick_folder() {
                    self.editable_series()
                        .borrow_mut()
                        .set_season_path(folder)?;
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

    fn warning(&mut self, kind: WarningKind) {
        let pop_up = WarningPopUp::new(kind);
        self.warning = Dialog::new(pop_up);
    }

    fn set_series_on_disk(&mut self, series: usize) {
        self.episodes_on_disk = Some(series);
    }

    fn increase_episode(&mut self) -> Result<(), Error> {
        let series = self.editable_series();
        let next_episode = series.borrow().episode().saturating_add(1);
        self.set_episode(next_episode)
    }

    fn set_episode(&mut self, value: NonZeroU8) -> Result<(), Error> {
        let series = self.editable_series();
        if !series.borrow().season_path_is_present() || value <= series.borrow().episode() {
            series.borrow_mut().set_episode(value)?;
            return Ok(());
        }
        let series_on_disk = self.series_on_disk()?;
        if series_on_disk < value.get() as usize {
            self.confirm(ConfirmKind::EpisodesOverflow { series_on_disk });
            return Ok(());
        }
        let series = self.editable_series();
        series.borrow_mut().set_episode(value)?;
        Ok(())
    }

    fn series_on_disk(&mut self) -> Result<usize, ErrorKind> {
        if let Some(series_on_disk) = self.episodes_on_disk {
            return Ok(series_on_disk);
        }
        self.read_series_on_disk()
    }

    fn season_path(&self) -> Result<PathBuf, ErrorKind> {
        let series = self.editable_series();
        let season_path = series.borrow().season_path().to_path_buf();
        if !season_path.exists() {
            return Err(ErrorKind::SeasonPathDidNotExists { season_path });
        }
        Ok(season_path)
    }

    fn read_series_on_disk(&mut self) -> Result<usize, ErrorKind> {
        let season_path = self.season_path()?;
        let series_on_disk = read_dir(season_path)?.len();
        self.set_series_on_disk(series_on_disk);
        Ok(series_on_disk)
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
