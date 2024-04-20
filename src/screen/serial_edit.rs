use std::{
    num::NonZeroU8,
    path::{Path, PathBuf},
};

use iced::{
    theme,
    widget::{button, column, horizontal_space, row, Row},
    Element,
};

use crate::{
    error::{Error, ErrorKind},
    serial::Serial,
    utils::{self, read_dir},
    view_utils::{link, signed_text_imput, square_button, DEFAULT_INDENT},
};

use super::confirm::{ConfirmScreen, Message as ConfirmScreenMessage};

#[derive(Debug, Clone, Copy)]
pub enum Kind {
    New,
    Change { id: usize },
}

#[derive(Debug, Clone)]
pub enum Message {
    Back,
    Accept {
        kind: Kind,
        name: String,
        season: NonZeroU8,
        seria: NonZeroU8,
        season_path: PathBuf,
    },
    Delete(usize),
    Watch {
        path: String,
        seria: usize,
    },
    NameChanged(String),
    SeasonChanged(String),
    SeriaChanged(String),
    SeasonPathChanged(String),
    SeasonPathSelect,
    SeasonTryNext,
    SeasonInc,
    SeasonDec,
    SeriaInc,
    SeriaDec,
    ConfirmScreen(ConfirmScreenMessage),
}

enum ConfirmKind {
    TrySwitchToNewSeason { potential_new_season: PathBuf },
    SeriaOverflow,
}

struct Confirm {
    screen: ConfirmScreen,
    kind: ConfirmKind,
}

pub struct SerialEditScreen {
    kind: Kind,
    name: String,
    season: NonZeroU8,
    seria: NonZeroU8,
    season_path: String,
    confirm_screen: Option<Confirm>,
    seies_on_disk: Option<usize>,
}

impl SerialEditScreen {
    pub fn new() -> Self {
        let one = NonZeroU8::MIN;
        let dialog = Self {
            kind: Kind::New,
            name: String::new(),
            season: one,
            seria: one,
            season_path: String::new(),
            confirm_screen: None,
            seies_on_disk: None,
        };
        dialog
    }

    pub fn change(serial: &Serial, id: usize) -> Self {
        Self {
            kind: Kind::Change { id },
            name: serial.name.clone(),
            season: serial.current_season,
            seria: serial.current_seria,
            season_path: serial.season_path.display().to_string(),
            confirm_screen: None,
            seies_on_disk: None,
        }
    }

    pub fn view(&self) -> Element<Message> {
        if let Some(confirm_screen) = &self.confirm_screen {
            return confirm_screen.screen.view().map(Message::ConfirmScreen);
        }
        let back_button = link("< Back").on_press(Message::Back);
        let edit_area = column![
            signed_text_imput("Name", &self.name, Message::NameChanged),
            row![
                signed_text_imput("Season", &self.season.to_string(), Message::SeasonChanged),
                square_button("-").on_press(Message::SeasonDec),
                square_button("+").on_press(Message::SeasonInc)
            ]
            .spacing(DEFAULT_INDENT),
            row![
                signed_text_imput("Seria", &self.seria.to_string(), Message::SeriaChanged),
                square_button("-").on_press(Message::SeriaDec),
                square_button("+").on_press(Message::SeriaInc)
            ]
            .spacing(DEFAULT_INDENT),
            row![
                signed_text_imput("Season path", &self.season_path, Message::SeasonPathChanged),
                square_button("...").on_press(Message::SeasonPathSelect),
                button("try next").on_press_maybe(
                    (!self.season_path.is_empty()).then_some(Message::SeasonTryNext)
                ),
                square_button(">").on_press(Message::Watch {
                    path: self.season_path.clone(),
                    seria: self.seria.get() as usize
                })
            ]
            .spacing(DEFAULT_INDENT)
        ]
        .spacing(DEFAULT_INDENT);
        let mut bottom_buttons = Row::new();
        if let Kind::Change { id } = self.kind {
            let delete_button = button("Delete")
                .style(theme::Button::Destructive)
                .on_press(Message::Delete(id));
            bottom_buttons = bottom_buttons.push(delete_button);
        }
        bottom_buttons = bottom_buttons.extend([
            horizontal_space().into(),
            button("Accept")
                .style(theme::Button::Positive)
                .on_press(self.accept())
                .into(),
        ]);
        column![back_button, edit_area, bottom_buttons]
            .padding(DEFAULT_INDENT)
            .spacing(DEFAULT_INDENT)
            .into()
    }

    pub fn update(&mut self, message: Message) -> Result<(), Error> {
        match message {
            Message::Back | Message::Accept { .. } | Message::Delete(_) | Message::Watch { .. } => {
            }
            Message::NameChanged(value) => self.name = value,
            Message::SeasonChanged(value) => {
                if let Ok(number) = value.parse() {
                    self.season = number;
                }
            }
            Message::SeriaChanged(value) => {
                if let Ok(number) = value.parse() {
                    self.seria = number;
                }
            }
            Message::SeasonInc => {
                self.season = self.season.saturating_add(1);
            }
            Message::SeasonDec => {
                if let Some(number) = NonZeroU8::new(self.season.get() - 1) {
                    self.season = number;
                }
            }
            Message::SeriaInc => {
                if self.season_path.is_empty() {
                    self.seria = self.seria.saturating_add(1);
                    return Ok(());
                }
                let seies_on_disk = match self.seies_on_disk {
                    Some(seies_on_disk) => seies_on_disk,
                    None => {
                        let series_on_disk = read_dir(&self.season_path)?.len();
                        self.set_series_on_disk(series_on_disk);
                        series_on_disk
                    }
                };
                if seies_on_disk == self.seria.get() as usize {
                    self.confirm(format!("It's seems like {} serias is a last of it season. Switch to the next season?", self.seria));
                } else {
                    self.seria = self.seria.saturating_add(1);
                }
            }
            Message::SeriaDec => {
                if let Some(number) = NonZeroU8::new(self.seria.get() - 1) {
                    self.seria = number;
                }
            }
            Message::SeasonPathChanged(value) => self.season_path = value,
            Message::SeasonTryNext => self.next_season()?,
            Message::ConfirmScreen(message) => match message {
                ConfirmScreenMessage::Confirm => {
                    let Some(confirm) = &self.confirm_screen else {
                        return Ok(());
                    };
                    match &confirm.kind {
                        ConfirmKind::TrySwitchToNewSeason {
                            potential_new_season,
                        } => {
                            self.season_path = potential_new_season.display().to_string();
                            self.close_confirm_screen();
                        }
                        ConfirmKind::SeriaOverflow => {
                            self.seria = NonZeroU8::MIN;
                            self.season = self.season.saturating_add(1);
                            self.next_season()?;
                        }
                    }
                }
                ConfirmScreenMessage::Cancel => {
                    self.close_confirm_screen();
                }
            },
            Message::SeasonPathSelect => {
                if let Some(folder) = rfd::FileDialog::new().pick_folder() {
                    self.season_path = folder.display().to_string();
                }
            }
        }
        Ok(())
    }

    fn set_series_on_disk(&mut self, series: usize) {
        self.seies_on_disk = Some(series);
    }

    fn next_season(&mut self) -> Result<(), ErrorKind> {
        if let Err(error) = self.next_season2() {
            self.close_confirm_screen();
            return Err(error.into());
        }
        Ok(())
    }

    fn next_season2(&mut self) -> Result<(), ErrorKind> {
        let season_path = PathBuf::from(&self.season_path);
        let next_season = next_dir(&season_path)?.ok_or(ErrorKind::FailedToFindNextSeasonPath)?;
        self.confirm_proposed_season(next_season);
        Ok(())
    }

    fn close_confirm_screen(&mut self) {
        self.confirm_screen = None;
    }

    fn confirm(&mut self, message: String) {
        let confirm = Confirm {
            screen: ConfirmScreen::new(message),
            kind: ConfirmKind::SeriaOverflow,
        };
        self.confirm_screen = Some(confirm);
    }

    fn confirm_proposed_season(&mut self, path: impl AsRef<Path>) {
        let path = path.as_ref();
        let confirm = Confirm {
            screen: ConfirmScreen::new(format!("Proposed path: {}", path.display())),
            kind: ConfirmKind::TrySwitchToNewSeason {
                potential_new_season: path.to_path_buf(),
            },
        };
        self.confirm_screen = Some(confirm);
    }

    fn accept(&self) -> Message {
        Message::Accept {
            kind: self.kind,
            name: self.name.clone(),
            season: self.season,
            seria: self.seria,
            season_path: PathBuf::from(&self.season_path),
        }
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
    let paths = utils::read_dir(parent)?;
    let dirs: Vec<_> = paths.into_iter().filter(|path| path.is_dir()).collect();
    let mut season_dir_index = None;
    for (i, dir) in dirs.iter().enumerate() {
        let dir = dir
            .file_name()
            .ok_or(ErrorKind::FailedToFindNextSeasonPath)?
            .to_str()
            .ok_or(ErrorKind::FailedToFindNextSeasonPath)?;
        dbg!(dir_name);
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
    Ok(Some(dirs[next_season_index].to_path_buf()))
}
