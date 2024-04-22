use std::{
    cell::RefCell,
    num::NonZeroU8,
    path::{Path, PathBuf},
    rc::Rc,
};

use iced::{
    theme,
    widget::{button, column, horizontal_space, row, text, Space},
    Element, Length,
};

use crate::{
    error::{Error, ErrorKind},
    serial::Serial,
    utils::{self, read_dir},
    view_utils::{link, signed_text_imput, square_button, DEFAULT_INDENT},
};

use super::confirm::{ConfirmScreen, Message as ConfirmScreenMessage};

#[derive(Debug, Clone)]
pub enum Message {
    Back,
    Delete(usize),
    Watch { path: String, seria: usize },
    NameChanged(String),
    SeasonChanged(String),
    SeriaChanged(String),
    SeasonPathChanged(String),
    SeasonPathSelect,
    SeasonInc,
    SeasonDec,
    SeriaInc,
    SeriaDec,
    ConfirmScreen(ConfirmScreenMessage),
}

enum ConfirmKind {
    TrySwitchToNewSeason { season_path: PathBuf },
    SeriaOverflow,
}

pub struct SerialEditScreen {
    serial: Rc<RefCell<Serial>>,
    confirm_screen: Option<ConfirmScreen<ConfirmKind>>,
    seies_on_disk: Option<usize>,
    id: usize,
}

impl SerialEditScreen {
    pub fn new(serial: Rc<RefCell<Serial>>, id: usize) -> Self {
        let dialog = Self {
            confirm_screen: None,
            seies_on_disk: None,
            serial,
            id,
        };
        dialog
    }

    pub fn view(&self) -> Element<Message> {
        if let Some(confirm_screen) = &self.confirm_screen {
            return confirm_screen.view().map(Message::ConfirmScreen);
        }
        let serial = self.serial.borrow();
        let season_path = serial.season_path().display().to_string();
        column![
            row![
                link("< Back").on_press(Message::Back),
                horizontal_space(),
                text(serial.name()),
                horizontal_space(),
                button("Delete")
                    .style(theme::Button::Destructive)
                    .on_press(Message::Delete(self.id)),
            ],
            Space::with_height(Length::Fixed(15.0)),
            column![
                signed_text_imput("Name", serial.name(), Message::NameChanged),
                row![
                    signed_text_imput(
                        "Season",
                        &serial.season().to_string(),
                        Message::SeasonChanged
                    ),
                    square_button("-").on_press(Message::SeasonDec),
                    square_button("+").on_press(Message::SeasonInc)
                ]
                .spacing(DEFAULT_INDENT),
                row![
                    signed_text_imput("Seria", &serial.seria().to_string(), Message::SeriaChanged),
                    square_button("-").on_press(Message::SeriaDec),
                    square_button("+").on_press(Message::SeriaInc)
                ]
                .spacing(DEFAULT_INDENT),
                row![
                    signed_text_imput("Season path", &season_path, Message::SeasonPathChanged),
                    square_button("...").on_press(Message::SeasonPathSelect),
                    square_button(">").on_press(Message::Watch {
                        path: season_path,
                        seria: serial.seria().get() as usize
                    })
                ]
                .spacing(DEFAULT_INDENT)
            ]
            .spacing(DEFAULT_INDENT)
        ]
        .padding(DEFAULT_INDENT)
        .spacing(DEFAULT_INDENT)
        .into()
    }

    pub fn update(&mut self, message: Message) -> Result<(), Error> {
        match message {
            Message::Back | Message::Delete(_) | Message::Watch { .. } => {}
            Message::NameChanged(value) => {
                let mut serial = self.serial.borrow_mut();
                serial.rename(value)?;
            }
            Message::SeasonChanged(value) => {
                if let Ok(number) = value.parse() {
                    self.serial.borrow_mut().set_season(number)?;
                }
            }
            Message::SeriaChanged(value) => {
                if let Ok(number) = value.parse() {
                    self.set_seria(number)?;
                }
            }
            Message::SeasonInc => self.increase_season()?,
            Message::SeasonDec => {
                let mut serial = self.serial.borrow_mut();
                if let Some(number) = NonZeroU8::new(serial.season().get() - 1) {
                    serial.set_season(number)?;
                }
            }
            Message::SeriaInc => self.increase_seria()?,
            Message::SeriaDec => {
                let mut serial = self.serial.borrow_mut();
                if let Some(number) = NonZeroU8::new(serial.seria().get() - 1) {
                    serial.set_seria(number)?;
                }
            }
            Message::SeasonPathChanged(value) => self
                .serial
                .borrow_mut()
                .set_season_path(PathBuf::from(value))?,
            Message::ConfirmScreen(message) => match message {
                ConfirmScreenMessage::Confirm => {
                    let Some(confirm) = &self.confirm_screen else {
                        return Ok(());
                    };
                    match &confirm.kind() {
                        ConfirmKind::TrySwitchToNewSeason { season_path } => {
                            self.serial
                                .borrow_mut()
                                .set_season_path(season_path.clone())?;
                            self.close_confirm_screen();
                        }
                        ConfirmKind::SeriaOverflow => self.increase_season()?,
                    }
                }
                ConfirmScreenMessage::Cancel => {
                    self.close_confirm_screen();
                }
            },
            Message::SeasonPathSelect => {
                if let Some(folder) = rfd::FileDialog::new().pick_folder() {
                    self.serial.borrow_mut().set_season_path(folder)?;
                }
            }
        }
        Ok(())
    }

    fn set_series_on_disk(&mut self, series: usize) {
        self.seies_on_disk = Some(series);
    }

    fn increase_seria(&mut self) -> Result<(), Error> {
        let next_seria = self.serial.borrow().seria().saturating_add(1);
        self.set_seria(next_seria)
    }

    fn set_seria(&mut self, value: NonZeroU8) -> Result<(), Error> {
        {
            let mut serial = self.serial.borrow_mut();
            if !serial.season_path_is_present() {
                serial.set_seria(value)?;
                return Ok(());
            }
        }
        let seies_on_disk = match self.seies_on_disk {
            Some(seies_on_disk) => seies_on_disk,
            None => {
                let series_on_disk = {
                    let serial = self.serial.borrow();
                    read_dir(&serial.season_path())?.len()
                };
                self.set_series_on_disk(series_on_disk);
                series_on_disk
            }
        };
        if seies_on_disk < value.get() as usize {
            self.confirm(
                ConfirmKind::SeriaOverflow,
                format!(
                    "It's seems like {} serias is a last of it season. Switch to the next season?",
                    seies_on_disk
                ),
            );
        } else {
            let mut serial = self.serial.borrow_mut();
            serial.set_seria(value)?;
        }
        Ok(())
    }

    fn set_seria_to_one(&mut self) -> Result<(), ErrorKind> {
        let mut serial = self.serial.borrow_mut();
        serial.set_seria(NonZeroU8::MIN)
    }

    fn increase_season(&mut self) -> Result<(), ErrorKind> {
        if !self.serial.borrow().season_path_is_present() {
            self.set_seria_to_one()?;
            let next_seria = self.serial.borrow().season().saturating_add(1);
            let mut serial = self.serial.borrow_mut();
            serial.set_season(next_seria)?;
        } else {
            let season_path = next_dir(&self.serial.borrow().season_path())?
                .ok_or(ErrorKind::FailedToFindNextSeasonPath)?;
            let message = format!("Proposed path: {}", season_path.display());
            self.confirm(ConfirmKind::TrySwitchToNewSeason { season_path }, message);
        }
        Ok(())
    }

    fn close_confirm_screen(&mut self) {
        self.confirm_screen = None;
    }

    fn confirm(&mut self, kind: ConfirmKind, message: String) {
        let confirm = ConfirmScreen::new(kind, message);
        self.confirm_screen = Some(confirm);
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
    Ok(Some(dirs[next_season_index].to_path_buf()))
}
