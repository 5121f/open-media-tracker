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
    serial::model::Serial,
    utils,
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

pub struct SerialEditScreen {
    kind: Kind,
    name: String,
    season: NonZeroU8,
    seria: NonZeroU8,
    season_path: String,
    confirm_screen: Option<ConfirmScreen>,
    potential_new_season: Option<PathBuf>,
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
            potential_new_season: None,
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
            potential_new_season: None,
        }
    }

    pub fn view(&self) -> Element<Message> {
        if let Some(confirm_screen) = &self.confirm_screen {
            return confirm_screen.view().map(Message::ConfirmScreen);
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
            button("Accept").on_press(self.accept()).into(),
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
                self.seria = self.seria.saturating_add(1);
            }
            Message::SeriaDec => {
                if let Some(number) = NonZeroU8::new(self.seria.get() - 1) {
                    self.seria = number;
                }
            }
            Message::SeasonPathChanged(value) => self.season_path = value,
            Message::SeasonTryNext => {
                let parent = PathBuf::from(&self.season_path)
                    .parent()
                    .ok_or(ErrorKind::parent_dir(&self.season_path))?
                    .to_owned();
                let paths = utils::read_dir(parent)?;
                let dirs: Vec<_> = paths.into_iter().filter(|path| path.is_dir()).collect();
                let proposed_index = (self.season.get() + 1 + 1) as usize;
                let proposed_path = &dirs[proposed_index];
                self.confirm_proposed_season(proposed_path);
            }
            Message::ConfirmScreen(message) => match message {
                ConfirmScreenMessage::Confirm => {
                    if let Some(new_season_path) = &self.potential_new_season {
                        self.season_path = new_season_path.display().to_string();
                    }
                    self.potential_new_season = None;
                    self.close_confirm_screen();
                }
                ConfirmScreenMessage::Cancel => {
                    self.potential_new_season = None;
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

    fn close_confirm_screen(&mut self) {
        self.confirm_screen = None;
    }

    fn confirm_proposed_season(&mut self, path: impl AsRef<Path>) {
        let path = path.as_ref();
        let screen = ConfirmScreen::new(format!("Proposed path: {}", path.display()));
        self.confirm_screen = Some(screen);
        self.potential_new_season = Some(path.to_path_buf());
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
