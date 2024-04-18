use std::num::NonZeroU8;

use iced::{
    widget::{button, column, horizontal_space, row, text, text_input, Row},
    Element,
};

use crate::{serial::model::Serial, Error};

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
    },
    Delete(usize),
    NameChanged(String),
    SeasonChanged(String),
    SeriaChanged(String),
    SeasonInc,
    SeasonDec,
    SeriaInc,
    SeriaDec,
}

pub struct SerialEditDialog {
    kind: Kind,
    name: String,
    season: NonZeroU8,
    seria: NonZeroU8,
}

impl SerialEditDialog {
    pub fn new() -> Result<Self, Error> {
        let one = NonZeroU8::new(1).ok_or(Error::SeasonAndSeriaCannotBeZero)?;
        let dialog = Self {
            kind: Kind::New,
            name: String::new(),
            season: one,
            seria: one,
        };
        Ok(dialog)
    }

    pub fn change(serial: &Serial, id: usize) -> Self {
        Self {
            kind: Kind::Change { id },
            name: serial.name.clone(),
            season: serial.current_season,
            seria: serial.current_seria,
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

    pub fn update(&mut self, message: Message) -> Result<(), Error> {
        match message {
            Message::Back | Message::Accept { .. } | Message::Delete(_) => {}
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
                self.season = self.season.checked_add(1).ok_or(Error::NumberOverflow)?;
            }
            Message::SeasonDec => {
                let one = NonZeroU8::new(1).ok_or(Error::SeasonAndSeriaCannotBeZero)?;
                self.season = self.season.checked_mul(one).ok_or(Error::NumberOverflow)?;
            }
            Message::SeriaInc => {
                self.seria = self.seria.checked_add(1).ok_or(Error::NumberOverflow)?;
            }
            Message::SeriaDec => {
                let one = NonZeroU8::new(1).ok_or(Error::SeasonAndSeriaCannotBeZero)?;
                self.seria = self.seria.checked_mul(one).ok_or(Error::NumberOverflow)?;
            }
        }
        Ok(())
    }

    fn accept(&self) -> Message {
        Message::Accept {
            kind: self.kind,
            name: self.name.clone(),
            season: self.season,
            seria: self.seria,
        }
    }
}
