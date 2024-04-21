mod config;
mod error;
mod screen;
mod serial;
mod utils;
mod view_utils;

use std::{cell::RefCell, rc::Rc};

use error::ErrorKind;
use iced::{executor, window, Application, Command, Element, Settings, Theme};

use crate::{
    config::Config,
    error::Error,
    screen::{Dialog, ErrorScreen, ErrorScreenMessage, MainScreenMessage, SerialEditScreenMessage},
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
    media: Vec<Rc<RefCell<Serial>>>,
    dialog: Dialog,
    error_dialog: Option<ErrorScreen>,
    config: Rc<Config>,
}

impl ZCinema {
    fn change_serial_screen(&mut self, id: usize) {
        let serial = Rc::clone(&self.media[id]);
        self.dialog = Dialog::change_serial(serial, id)
    }

    fn main_screen(&mut self) {
        let media: Vec<_> = self.media.iter().map(Rc::clone).collect();
        self.dialog = Dialog::main(&media);
    }

    fn error_screen(&mut self, error: Error) {
        self.error_dialog = Some(error.into());
    }

    fn remove_serial(&mut self, id: usize) -> Result<(), Error> {
        {
            let serial = self.media[id].borrow();
            serial.remove_file(&self.config.data_dir)?;
        }
        self.media.remove(id);
        Ok(())
    }

    fn read_media(config: Rc<Config>) -> Result<Vec<Serial>, ErrorKind> {
        let media = if config.data_dir.exists() {
            utils::read_media(config)?
        } else {
            Vec::new()
        };
        Ok(media)
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
                    MainScreenMessage::AddSerial => {
                        let serial = Rc::new(RefCell::new(Serial::new(Rc::clone(&self.config))));
                        self.media.push(serial);
                        self.change_serial_screen(self.media.len() - 1);
                    }
                    MainScreenMessage::ChangeSerial(id) => self.change_serial_screen(id),
                }
                Ok(Command::none())
            }
            Message::SerialEditScreen(message) => {
                match message {
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
        let config = Config::read().map_err(|kind| Error::critical(kind))?;
        let config = Rc::new(config);
        let media: Vec<_> = Self::read_media(config.clone())
            .map_err(|kind| Error::critical(kind))?
            .into_iter()
            .map(RefCell::new)
            .map(Rc::new)
            .collect();
        let main_window = Dialog::main(&media);
        Ok(Self {
            media,
            dialog: main_window,
            error_dialog: None,
            config,
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
