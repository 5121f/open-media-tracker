use std::{
    borrow::{Borrow, BorrowMut},
    fs,
    num::NonZeroU8,
    path::PathBuf,
    rc::Rc,
    sync::Arc,
};

use anyhow::Context;
use error_dialog::ErrorDialog;
use iced::{widget::shader::wgpu::core::error, Element, Sandbox, Settings, Theme};
use ron::ser::PrettyConfig;
use serde::{Deserialize, Serialize};

use crate::{main_window::MainWindow, serial_chamge_dialog::SerialChangeDialog};

fn main() -> iced::Result {
    ZCinema::run(Settings::default())
}

#[derive(Debug, Clone)]
enum Message {
    MainWindow(main_window::Message),
    SerialChange(serial_chamge_dialog::Message),
    ErrorDialog(error_dialog::Message),
}

struct ZCinema {
    media: Vec<Rc<Serial>>,
    dialog: Dialog,
    state_dir: PathBuf,
}

impl ZCinema {
    fn add_serial_dialog(&mut self) -> Result<(), Error> {
        self.dialog = Dialog::add_serial()?;
        Ok(())
    }

    fn change_serial_dialog(&mut self, id: usize) {
        let serial = &self.media[id];
        self.dialog = Dialog::change_serial(serial, id)
    }

    fn main_window(&mut self) {
        let media = clone_rc_vec(&self.media);
        self.dialog = Dialog::main_window(media);
    }

    fn error_dialog(&mut self, message: impl ToString) {
        self.dialog = Dialog::error(message.to_string());
    }

    fn handle_error<T, E>(&mut self, result: Result<T, E>) -> Option<T>
    where
        E: std::error::Error,
    {
        match result {
            Ok(value) => Some(value),
            Err(err) => {
                self.error_dialog(err);
                None
            }
        }
    }

    fn save_serial(&self, id: usize) {
        let serial = self.media[id].as_ref();
        let content = ron::ser::to_string_pretty(serial, PrettyConfig::new()).unwrap();
        if !self.state_dir.exists() {
            fs::create_dir(&self.state_dir).unwrap();
        }
        let file_name = format!("{}.ron", &serial.name);
        let path = self.state_dir.join(&file_name);
        fs::write(path, content).unwrap();
    }
}

impl Sandbox for ZCinema {
    type Message = Message;

    fn new() -> Self {
        let state_dir = dirs::state_dir().unwrap().join("zcinema");
        let mut media = Vec::new();
        for entry in fs::read_dir(&state_dir).unwrap() {
            let entry = entry.unwrap().path();
            if entry.is_file() {
                let file_content = fs::read_to_string(entry).unwrap();
                let m: Serial = ron::from_str(&file_content).unwrap();
                media.push(Rc::new(m));
            }
        }
        let media2 = clone_rc_vec(&media);
        Self {
            media,
            dialog: Dialog::main_window(media2),
            state_dir,
        }
    }

    fn title(&self) -> String {
        String::from("ZCinema")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::MainWindow(main_window::Message::AddSerial) => {
                let res = self.add_serial_dialog();
                self.handle_error(res);
            }
            Message::MainWindow(main_window::Message::ChangeSerial(id)) => {
                self.change_serial_dialog(id)
            }
            Message::SerialChange(serial_chamge_dialog::Message::Accept) => {
                if let Dialog::SerialChange(dialog) = &mut self.dialog {
                    if let Some(id) = dialog.id {
                        let serial = Rc::get_mut(&mut self.media[id]).unwrap();
                        serial.name = dialog.name.clone();
                        serial.current_season = dialog.season;
                        serial.current_seria = dialog.seria;
                        self.save_serial(id);
                    } else {
                        let serial = Rc::new(dialog.build());
                        self.media.push(serial);
                        self.save_serial(self.media.len() - 1);
                    }
                    self.main_window();
                }
            }
            Message::SerialChange(serial_chamge_dialog::Message::Back) => {
                self.main_window();
            }
            Message::SerialChange(dialog_message) => {
                if let Dialog::SerialChange(dialog) = &mut self.dialog {
                    let res = dialog.update(dialog_message);
                    self.handle_error(res);
                }
            }
            Message::ErrorDialog(error_dialog::Message::Ok) => self.main_window(),
        }
    }

    fn view(&self) -> Element<Message> {
        self.dialog.view()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

enum Dialog {
    MainWindow(MainWindow),
    SerialChange(SerialChangeDialog),
    Error(ErrorDialog),
}

impl Dialog {
    fn main_window(media: Vec<Rc<Serial>>) -> Self {
        let dialog = MainWindow::new(media);
        Self::MainWindow(dialog)
    }

    fn add_serial() -> Result<Self, Error> {
        let dialog = SerialChangeDialog::new()?;
        Ok(Self::SerialChange(dialog))
    }

    fn change_serial(serial: &Serial, id: usize) -> Self {
        let dialog = SerialChangeDialog::change(serial, id);
        Self::SerialChange(dialog)
    }

    fn error(message: String) -> Self {
        let dialog = ErrorDialog::new(message);
        Self::Error(dialog)
    }
}

impl Dialog {
    fn view(&self) -> Element<Message> {
        match self {
            Dialog::MainWindow(dialog) => dialog.view().map(Message::MainWindow),
            Dialog::SerialChange(dialog) => dialog.view().map(Message::SerialChange),
            Dialog::Error(dialog) => dialog.view().map(Message::ErrorDialog),
        }
    }
}

mod main_window {
    use std::{rc::Rc, sync::Arc};

    use iced::{
        widget::{button, column, horizontal_space, row, text},
        Element,
    };

    use crate::Serial;

    #[derive(Debug, Clone)]
    pub enum Message {
        AddSerial,
        ChangeSerial(usize),
    }

    pub struct MainWindow {
        media: Vec<Rc<Serial>>,
    }

    impl MainWindow {
        pub fn new(media: Vec<Rc<Serial>>) -> Self {
            Self { media }
        }

        pub fn view(&self) -> Element<Message> {
            column![
                button("+").on_press(Message::AddSerial),
                column(
                    self.media
                        .iter()
                        .enumerate()
                        .map(|(id, m)| row![
                            text(&m.name),
                            horizontal_space(),
                            button("...").on_press(Message::ChangeSerial(id))
                        ]
                        .into())
                        .collect::<Vec<_>>()
                )
            ]
            .into()
        }
    }
}

mod serial_chamge_dialog {
    use std::num::NonZeroU8;

    use anyhow::anyhow;
    use iced::{
        widget::{button, column, horizontal_space, row, text, text_input},
        Element,
    };

    use crate::{Error, Serial};

    #[derive(Debug, Clone)]
    pub enum Message {
        Back,
        Accept,
        NameChanged(String),
        SeasonChanged(String),
        SeriaChanged(String),
        SeasonInc,
        SeasonDec,
        SeriaInc,
        SeriaDec,
    }

    pub struct SerialChangeDialog {
        pub id: Option<usize>,
        pub name: String,
        pub season: NonZeroU8,
        pub seria: NonZeroU8,
    }

    impl SerialChangeDialog {
        pub fn new() -> Result<Self, Error> {
            let one = NonZeroU8::new(1).ok_or(Error::SeasonAndSeriaCannotBeZero)?;
            let dialog = Self {
                id: None,
                name: String::new(),
                season: one,
                seria: one,
            };
            Ok(dialog)
        }

        pub fn change(serial: &Serial, id: usize) -> Self {
            let name = serial.name.clone();
            Self {
                id: Some(id),
                name,
                season: serial.current_season,
                seria: serial.current_seria,
            }
        }

        pub fn view(&self) -> Element<Message> {
            column![
                button("< Back").on_press(Message::Back),
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
                    horizontal_space(),
                    button("Accept").on_press(Message::Accept)
                ]
            ]
            .into()
        }

        pub fn update(&mut self, message: Message) -> Result<(), Error> {
            match message {
                Message::Back | Message::Accept => {}
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

        pub fn build(&self) -> Serial {
            Serial {
                name: self.name.clone(),
                current_season: self.season,
                current_seria: self.seria,
            }
        }
    }
}

mod error_dialog {
    use iced::{
        widget::{button, column, horizontal_space, row, text, vertical_space},
        Element,
    };

    #[derive(Debug, Clone)]
    pub enum Message {
        Ok,
    }

    pub struct ErrorDialog {
        message: String,
    }

    impl ErrorDialog {
        pub fn new(message: String) -> Self {
            Self { message }
        }

        pub fn view(&self) -> Element<Message> {
            row![
                horizontal_space(),
                column![
                    vertical_space(),
                    text(format!("Error: {}", &self.message)),
                    row![horizontal_space(), button("Ok").on_press(Message::Ok)],
                    vertical_space()
                ],
                horizontal_space()
            ]
            .into()
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Serial {
    name: String,
    current_season: NonZeroU8,
    current_seria: NonZeroU8,
}

impl Serial {
    pub fn new(name: String) -> Result<Self, Error> {
        let one = NonZeroU8::new(1).ok_or(Error::SeasonAndSeriaCannotBeZero)?;
        let serial = Self {
            name,
            current_season: one,
            current_seria: one,
        };
        Ok(serial)
    }
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("Season and seria number can not be zero")]
    SeasonAndSeriaCannotBeZero,
    #[error("Number overflow")]
    NumberOverflow,
}

fn clone_rc_vec<T>(v: &[Rc<T>]) -> Vec<Rc<T>> {
    v.iter().map(|m| Rc::clone(&m)).collect()
}
