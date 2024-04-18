mod error;
mod screen;
mod serial;

use std::{
    fs,
    path::{Path, PathBuf},
};

use error::ErrorKind;
use iced::{executor, window, Application, Command, Element, Settings, Theme};

use crate::{
    error::Error,
    screen::{
        serial_edit, Dialog, ErrorScreen, ErrorScreenMessage, MainScreenMessage,
        SerialEditScreenMessage,
    },
    serial::viewmodel::Serial,
};

fn main() -> iced::Result {
    ZCinema::run(Settings::default())
}

#[derive(Debug, Clone)]
enum Message {
    MainScreen(MainScreenMessage),
    SerialEditScreen(SerialEditScreenMessage),
    ErrorScreen(ErrorScreenMessage),
}

#[derive(Default)]
struct ZCinema {
    media: Vec<Serial>,
    dialog: Dialog,
    error_dialog: Option<ErrorScreen>,
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

    fn error_dialog(&mut self, error: Error) {
        let critical = error.critical;
        let dialog = ErrorScreen::new(error, critical);
        self.error_dialog = Some(dialog);
    }

    fn save_serial(&self, id: usize) -> Result<(), Error> {
        self.media[id]
            .save(&self.state_dir)
            .map_err(|kind| Error::general(kind))
    }

    fn remove_serial(&mut self, id: usize) -> Result<(), Error> {
        let serial = &self.media[id];
        serial
            .remove_file(&self.state_dir)
            .map_err(|kind| Error::general(kind))?;
        self.media.remove(id);
        Ok(())
    }

    fn read_media(dir: &Path) -> Result<Vec<Serial>, ErrorKind> {
        let media = if dir.exists() {
            read_media(dir)?
        } else {
            Vec::new()
        };
        Ok(media)
    }

    fn state_dir() -> Result<PathBuf, ErrorKind> {
        Ok(dirs::state_dir()
            .ok_or(ErrorKind::StateDirNotFound)?
            .join("zcinema"))
    }

    fn update2(&mut self, message: Message) -> Result<Command<Message>, Error> {
        match message {
            Message::MainScreen(message) => {
                match message {
                    MainScreenMessage::AddSerial => self.add_serial_dialog(),
                    MainScreenMessage::ChangeSerial(id) => self.change_serial_dialog(id),
                }
                Ok(Command::none())
            }
            Message::SerialEditScreen(message) => {
                match message {
                    SerialEditScreenMessage::Accept {
                        kind,
                        name,
                        season,
                        seria,
                        season_path,
                    } => {
                        if let serial_edit::Kind::Change { id } = kind {
                            self.media[id]
                                .rename(&self.state_dir, name)
                                .map_err(|kind| Error::general(kind))?;
                            self.media[id]
                                .change_season(season)
                                .map_err(|kind| Error::general(kind))?;
                            self.media[id]
                                .change_seria(seria)
                                .map_err(|kind| Error::general(kind))?;
                            self.media[id]
                                .change_season_path(season_path)
                                .map_err(|kind| Error::general(kind))?;
                            self.save_serial(id)?;
                        } else {
                            let serial = Serial::new(name, season, seria, season_path);
                            self.media.push(serial);
                            self.save_serial(self.media.len() - 1)?;
                        }
                        self.main_window();
                    }
                    SerialEditScreenMessage::Delete(id) => {
                        self.remove_serial(id)?;
                        self.main_window();
                    }
                    SerialEditScreenMessage::Back => {
                        self.main_window();
                    }
                    SerialEditScreenMessage::Watch { path, seria } => {
                        watch(path, seria)?;
                    }
                    _ => {
                        if let Dialog::SerialChange(dialog) = &mut self.dialog {
                            dialog.update(message);
                        };
                    }
                }
                Ok(Command::none())
            }
            Message::ErrorScreen(ErrorScreenMessage::Ok { critical }) => {
                if critical {
                    Ok(window::close(window::Id::MAIN))
                } else {
                    self.error_dialog = None;
                    Ok(Command::none())
                }
            }
        }
    }

    fn new2() -> Result<Self, Error> {
        let state_dir = Self::state_dir().map_err(|kind| Error::critical(kind))?;
        let media = Self::read_media(&state_dir).map_err(|kind| Error::critical(kind))?;
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
            Err(error) => {
                let mut zcinema = Self::default();
                zcinema.error_dialog(error);
                zcinema
            }
        };

        (zcinema, Command::none())
    }

    fn title(&self) -> String {
        String::from("ZCinema")
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match self.update2(message) {
            Ok(cmd) => cmd,
            Err(error) => {
                self.error_dialog(error);
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        if let Some(error_dialog) = &self.error_dialog {
            error_dialog.view().map(Message::ErrorScreen)
        } else {
            self.dialog.view()
        }
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

fn read_media(dir: &Path) -> Result<Vec<Serial>, ErrorKind> {
    let read_dir = fs::read_dir(dir).map_err(|source| ErrorKind::fsio(&dir, source))?;
    let mut media = Vec::new();
    for entry in read_dir {
        let entry = entry.map_err(|source| ErrorKind::fsio(dir, source))?;
        let path = entry.path();
        if path.is_file() {
            let serial = Serial::read_from_file(&path)?;
            media.push(serial);
        }
    }
    Ok(media)
}

fn watch(path: impl AsRef<Path>, seria_number: usize) -> Result<(), Error> {
    let read_dir =
        fs::read_dir(&path).map_err(|source| Error::general(ErrorKind::fsio(&path, source)))?;
    let mut files = Vec::new();
    for entry in read_dir {
        let entry = entry.map_err(|source| Error::general(ErrorKind::fsio(&path, source)))?;
        files.push(entry.path());
    }
    let seria = &files[seria_number];
    let mut cmd = std::process::Command::new("xdg-open");
    cmd.arg(seria);
    cmd.spawn()
        .map_err(|source| Error::general(ErrorKind::fsio(path, source)))?;
    Ok(())
}
