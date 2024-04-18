mod dialogs;
mod serial;

use std::{
    fs,
    path::{Path, PathBuf},
};

use dialogs::error_dialog::ErrorDialog;
use iced::{Element, Sandbox, Settings, Theme};

use crate::{
    dialogs::{error_dialog, main_window, serial_edit_dialog, Dialog},
    serial::viewmodel::Serial,
};

fn main() -> iced::Result {
    ZCinema::run(Settings::default())
}

#[derive(Debug, Clone)]
enum Message {
    MainWindow(main_window::Message),
    SerialChange(serial_edit_dialog::Message),
    ErrorDialog(error_dialog::Message),
}

struct ZCinema {
    media: Vec<Serial>,
    dialog: Dialog,
    error_dialog: Option<ErrorDialog>,
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
        self.dialog = Dialog::main_window(&self.media);
    }

    fn error_dialog(&mut self, message: impl ToString) {
        let dialog = ErrorDialog::new(message);
        self.error_dialog = Some(dialog);
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
        self.media[id].save(&self.state_dir);
    }

    fn remove_serial(&mut self, id: usize) {
        let serial = &self.media[id];
        serial.remove_file(&self.state_dir);
        self.media.remove(id);
    }
}

impl Sandbox for ZCinema {
    type Message = Message;

    fn new() -> Self {
        let state_dir = dirs::state_dir().unwrap().join("zcinema");
        let media = if state_dir.exists() {
            read_media(&state_dir)
        } else {
            Vec::new()
        };
        let main_window = Dialog::main_window(&media);
        Self {
            media,
            dialog: main_window,
            error_dialog: None,
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
            Message::SerialChange(serial_edit_dialog::Message::Accept {
                kind,
                name,
                season,
                seria,
            }) => {
                if let serial_edit_dialog::Kind::Change { id } = kind {
                    let serial = &mut self.media[id];
                    serial.rename(&self.state_dir, name);
                    serial.change_season(season);
                    serial.change_seria(seria);
                    self.save_serial(id);
                } else {
                    let serial = Serial::new(name, season, seria);
                    self.media.push(serial);
                    self.save_serial(self.media.len() - 1);
                }
                self.main_window();
            }
            Message::SerialChange(serial_edit_dialog::Message::Delete(id)) => {
                self.remove_serial(id);
                self.main_window();
            }
            Message::SerialChange(serial_edit_dialog::Message::Back) => {
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
        if let Some(error_dialog) = &self.error_dialog {
            error_dialog.view().map(Message::ErrorDialog)
        } else {
            self.dialog.view()
        }
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("Season and seria number can not be zero")]
    SeasonAndSeriaCannotBeZero,
    #[error("Number overflow")]
    NumberOverflow,
}

fn read_media(dir: &Path) -> Vec<Serial> {
    let mut media = Vec::new();
    for entry in fs::read_dir(&dir).unwrap() {
        let entry = entry.unwrap().path();
        if entry.is_file() {
            let serial = Serial::read_from_file(entry);
            media.push(serial);
        }
    }
    media
}
