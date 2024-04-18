mod dialogs;
mod error;
mod serial;

use std::{
    fs,
    path::{Path, PathBuf},
};

use dialogs::error_dialog::ErrorDialog;
use iced::{executor, window, Application, Command, Element, Settings, Theme};

use crate::{
    dialogs::{error_dialog, main_window, serial_edit_dialog, Dialog},
    error::Error,
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

#[derive(Default)]
struct ZCinema {
    media: Vec<Serial>,
    dialog: Dialog,
    error_dialog: Option<ErrorDialog>,
    state_dir: PathBuf,
}

impl ZCinema {
    fn add_serial_dialog(&mut self) {
        self.dialog = Dialog::add_serial();
    }

    fn change_serial_dialog(&mut self, id: usize) {
        let serial = &self.media[id];
        self.dialog = Dialog::change_serial(serial, id)
    }

    fn main_window(&mut self) {
        self.dialog = Dialog::main_window(&self.media);
    }

    fn error_dialog(&mut self, message: impl ToString, critical: bool) {
        let dialog = ErrorDialog::new(message, critical);
        self.error_dialog = Some(dialog);
    }

    fn handle_error<T, E>(&mut self, result: Result<T, E>, critical: bool) -> Option<T>
    where
        E: std::error::Error,
    {
        match result {
            Ok(value) => Some(value),
            Err(err) => {
                self.error_dialog(err, critical);
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

    fn read_media(dir: &Path) -> Result<Vec<Serial>, Error> {
        let media = if dir.exists() {
            read_media(dir)?
        } else {
            Vec::new()
        };
        Ok(media)
    }

    fn state_dir() -> Result<PathBuf, Error> {
        Ok(dirs::state_dir()
            .ok_or(Error::StateDirNotFound)?
            .join("zcinema"))
    }

    fn new2() -> Result<Self, Error> {
        let state_dir = Self::state_dir()?;
        let media = Self::read_media(&state_dir)?;
        let main_window = Dialog::main_window(&media);
        Ok(Self {
            media,
            dialog: main_window,
            error_dialog: None,
            state_dir,
        })
    }
}

impl Application for ZCinema {
    type Executor = executor::Default;
    type Theme = Theme;
    type Flags = ();
    type Message = Message;

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        let zcinema = match Self::new2() {
            Ok(s) => s,
            Err(err) => {
                let mut zcinema = Self::default();
                zcinema.error_dialog(err, true);
                zcinema
            }
        };

        (zcinema, Command::none())
    }

    fn title(&self) -> String {
        String::from("ZCinema")
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match message {
            Message::MainWindow(main_window::Message::AddSerial) => {
                self.add_serial_dialog();
                Command::none()
            }
            Message::MainWindow(main_window::Message::ChangeSerial(id)) => {
                self.change_serial_dialog(id);
                Command::none()
            }
            Message::SerialChange(serial_edit_dialog::Message::Accept {
                kind,
                name,
                season,
                seria,
            }) => {
                if let serial_edit_dialog::Kind::Change { id } = kind {
                    let res = self.media[id].rename(&self.state_dir, name);
                    self.handle_error(res, false);
                    self.media[id].change_season(season);
                    self.media[id].change_seria(seria);
                    self.save_serial(id);
                } else {
                    let serial = Serial::new(name, season, seria);
                    self.media.push(serial);
                    self.save_serial(self.media.len() - 1);
                }
                self.main_window();
                Command::none()
            }
            Message::SerialChange(serial_edit_dialog::Message::Delete(id)) => {
                self.remove_serial(id);
                self.main_window();
                Command::none()
            }
            Message::SerialChange(serial_edit_dialog::Message::Back) => {
                self.main_window();
                Command::none()
            }
            Message::SerialChange(dialog_message) => {
                if let Dialog::SerialChange(dialog) = &mut self.dialog {
                    dialog.update(dialog_message);
                }
                Command::none()
            }
            Message::ErrorDialog(error_dialog::Message::Ok { critical }) => {
                if critical {
                    window::close(window::Id::MAIN)
                } else {
                    self.error_dialog = None;
                    Command::none()
                }
            }
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

fn read_media(dir: &Path) -> Result<Vec<Serial>, Error> {
    let read_dir = fs::read_dir(dir).map_err(|source| Error::fsio(&dir, source))?;
    let mut media = Vec::new();
    for entry in read_dir {
        let entry = entry.map_err(|source| Error::fsio(dir, source))?;
        let path = entry.path();
        if path.is_file() {
            let serial = Serial::read_from_file(&path)?;
            media.push(serial);
        }
    }
    Ok(media)
}
