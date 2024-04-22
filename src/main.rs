mod config;
mod error;
mod screen;
mod serial;
mod utils;
mod view_utils;

use std::{cell::RefCell, rc::Rc};

use error::ErrorKind;
use iced::{executor, window, Application, Command, Element, Settings, Theme};
use screen::{MainScreen, SerialEditScreen};

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
    screen: Screens,
    error_dialog: Dialog<ErrorScreen>,
    config: Rc<Config>,
}

impl ZCinema {
    fn change_serial_screen(&mut self, id: usize) {
        let serial = Rc::clone(&self.media[id]);
        self.screen = Screens::change_serial(serial, id)
    }

    fn main_screen(&mut self) {
        let media: Vec<_> = self.media.iter().map(Rc::clone).collect();
        self.screen = Screens::main(&media);
    }

    fn error_screen(&mut self, error: Error) {
        self.error_dialog = Dialog::new(error.into());
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

    fn close_app(&self) -> Command<Message> {
        window::close(window::Id::MAIN)
    }

    fn update2(&mut self, message: Message) -> Result<Command<Message>, Error> {
        match message {
            Message::MainScreen(message) => {
                match message {
                    MainScreenMessage::AddSerial => {
                        let serial = Serial::new(Rc::clone(&self.config))?;
                        let serial = Rc::new(RefCell::new(serial));
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
                        if let Screens::SerialChange(dialog) = &mut self.screen {
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
                self.error_dialog.close();
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
        let main_window = Screens::main(&media);
        Ok(Self {
            media,
            screen: main_window,
            error_dialog: Dialog::closed(),
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
                error_dialog: Dialog::new(error.into()),
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
        if let Some(error_dialog) = &self.error_dialog.get() {
            error_dialog.view().map(Message::ErrorScreen)
        } else {
            self.screen.view()
        }
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

pub enum Screens {
    MainWindow(MainScreen),
    SerialChange(SerialEditScreen),
}

impl Screens {
    fn view(&self) -> Element<Message> {
        match self {
            Screens::MainWindow(dialog) => dialog.view().map(Message::MainScreen),
            Screens::SerialChange(dialog) => dialog.view().map(Message::SerialEditScreen),
        }
    }

    fn main(media: &[Rc<RefCell<Serial>>]) -> Self {
        let media = media.into_iter().map(Rc::clone).collect();
        let dialog = MainScreen::new(media);
        Self::MainWindow(dialog)
    }

    fn change_serial(serial: Rc<RefCell<Serial>>, id: usize) -> Self {
        let dialog = SerialEditScreen::new(serial, id);
        Self::SerialChange(dialog)
    }
}

impl Default for Screens {
    fn default() -> Self {
        Screens::MainWindow(MainScreen::default())
    }
}
