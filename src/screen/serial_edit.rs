use std::{num::NonZeroU8, path::PathBuf};

use iced::{
    widget::{button, column, horizontal_space, row, text, text_input, Row},
    Element,
};

use crate::serial::model::Serial;

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
    SeasonInc,
    SeasonDec,
    SeriaInc,
    SeriaDec,
}

pub struct SerialEditScreen {
    kind: Kind,
    name: String,
    season: NonZeroU8,
    seria: NonZeroU8,
    season_path: String,
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
        }
    }

    pub fn view(&self) -> Element<Message> {
        let back_button = button("< Back").on_press(Message::Back);
        let edit_area = column![
            row![
                text("Name"),
                text_input("Name", &self.name).on_input(Message::NameChanged)
            ],
            row![
                text("Season"),
                text_input("Season", &self.season.to_string()).on_input(Message::SeasonChanged),
                button("-").on_press(Message::SeasonDec),
                button("+").on_press(Message::SeasonInc)
            ],
            row![
                text("Seria"),
                text_input("Seria", &self.seria.to_string()).on_input(Message::SeriaChanged),
                button("-").on_press(Message::SeriaDec),
                button("+").on_press(Message::SeriaInc)
            ],
            row![
                text("Season path"),
                text_input("Season path", &self.season_path).on_input(Message::SeasonPathChanged),
                button(">").on_press(Message::Watch {
                    path: self.season_path.clone(),
                    seria: self.seria.get() as usize
                })
            ]
        ];
        let mut bottom_buttons = Row::new();
        if let Kind::Change { id } = self.kind {
            let delete_button = button("Delete").on_press(Message::Delete(id));
            bottom_buttons = bottom_buttons.push(delete_button);
        }
        bottom_buttons = bottom_buttons.extend([
            horizontal_space().into(),
            button("Accept").on_press(self.accept()).into(),
        ]);
        column![back_button, edit_area, bottom_buttons].into()
    }

    pub fn update(&mut self, message: Message) {
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
        }
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
