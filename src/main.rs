mod error;
mod screen;
mod serial;
mod utils;
mod view_utils;

use std::{
    path::{Path, PathBuf},
    rc::Rc,
};

use error::ErrorKind;
use iced::{executor, window, Application, Command, Element, Settings, Theme};

use crate::{
    error::Error,
    screen::{
        serial_edit, Dialog, ErrorScreen, ErrorScreenMessage, MainScreenMessage,
        SerialEditScreenMessage,
    },
    serial::Serial,
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
    media: Vec<Rc<Serial>>,
    dialog: Dialog,
    error_dialog: Option<ErrorScreen>,
    data_dir: PathBuf,
}

impl ZCinema {
    fn add_serial_screen(&mut self) {
        self.dialog = Dialog::add_serial();
    }

    fn change_serial_screen(&mut self, id: usize) {
        let serial = &self.media[id];
        self.dialog = Dialog::change_serial(serial, id)
    }

    fn main_screen(&mut self) {
        let media: Vec<_> = self.media.iter().map(Rc::clone).collect();
        self.dialog = Dialog::main(&media);
    }

    fn error_screen(&mut self, error: Error) {
        self.error_dialog = Some(error.into());
    }

    fn save_serial(&self, id: usize) -> Result<(), Error> {
        Ok(self.media[id].save(&self.data_dir)?)
    }

    fn remove_serial(&mut self, id: usize) -> Result<(), Error> {
        let serial = &self.media[id];
        serial.remove_file(&self.data_dir)?;
        self.media.remove(id);
        Ok(())
    }

    fn read_media(dir: impl AsRef<Path>) -> Result<Vec<Serial>, ErrorKind> {
        let dir = dir.as_ref();
        let media = if dir.exists() {
            utils::read_media(dir)?
        } else {
            Vec::new()
        };
        Ok(media)
    }

    fn data_dir() -> Result<PathBuf, ErrorKind> {
        Ok(dirs::data_dir()
            .ok_or(ErrorKind::StateDirNotFound)?
            .join("zcinema"))
    }

    fn close_error_screen(&mut self) {
        self.error_dialog = None;
    }

    fn close_app(&self) -> Command<Message> {
        window::close(window::Id::MAIN)
    }

    fn update2(&mut self, message: Message) -> Result<Command<Message>, Error> {
        match message {
            Message::MainScreen(message) => {
                match message {
                    MainScreenMessage::AddSerial => self.add_serial_screen(),
                    MainScreenMessage::ChangeSerial(id) => self.change_serial_screen(id),
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
                            let serial =
                                Rc::get_mut(&mut self.media[id]).ok_or(ErrorKind::Unknown)?;
                            serial.rename(&self.data_dir, name)?;
                            serial.season = season;
                            serial.seria = seria;
                            serial.season_path = season_path;
                            self.save_serial(id)?;
                        } else {
                            let serial = Serial::new(name, season, seria, season_path).into();
                            self.media.push(serial);
                            self.save_serial(self.media.len() - 1)?;
                        }
                        self.main_screen();
                    }
                    SerialEditScreenMessage::Delete(id) => {
                        self.remove_serial(id)?;
                        self.main_screen();
                    }
                    SerialEditScreenMessage::Back => {
                        self.main_screen();
                    }
                    SerialEditScreenMessage::Watch { path, seria } => {
                        utils::watch(path, seria)?;
                    }
                    _ => {
                        if let Dialog::SerialChange(dialog) = &mut self.dialog {
                            dialog.update(message)?;
                        };
                    }
                }
                Ok(Command::none())
            }
            Message::ErrorScreen(ErrorScreenMessage::Ok { critical }) => {
                if critical {
                    return Ok(self.close_app());
                }
                self.close_error_screen();
                Ok(Command::none())
            }
        }
    }

    fn new2() -> Result<Self, Error> {
        let data_dir = Self::data_dir().map_err(|kind| Error::critical(kind))?;
        let media: Vec<_> = Self::read_media(&data_dir)
            .map_err(|kind| Error::critical(kind))?
            .into_iter()
            .map(Rc::new)
            .collect();
        let main_window = Dialog::main(&media);
        Ok(Self {
            media,
            dialog: main_window,
            error_dialog: None,
            data_dir,
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
            Ok(zcinema) => zcinema,
            Err(error) => Self {
                error_dialog: Some(error.into()),
                ..Default::default()
            },
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
                self.error_screen(error);
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
