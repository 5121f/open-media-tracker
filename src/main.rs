mod config;
mod dialog;
mod error;
mod screen;
mod serial;
mod utils;
mod view_utils;

use std::{cell::RefCell, fmt::Display, rc::Rc};

use error::ErrorKind;
use iced::{executor, font, window, Application, Command, Element, Settings, Theme};
use iced_aw::modal;
use screen::{ConfirmScreen, ConfirmScreenMessage, MainScreen, SerialEditScreen};
use utils::arr_rc_clone;

use crate::{
    config::Config,
    dialog::Dialog,
    error::Error,
    screen::{ErrorScreen, ErrorScreenMessage, MainScreenMessage, SerialEditScreenMessage},
    serial::Serial,
};

fn main() -> iced::Result {
    ZCinema::run(Settings::default())
}

#[derive(Debug, Clone)]
enum Message {
    MainScreen(MainScreenMessage),
    SerialEditScreen(SerialEditScreenMessage),
    ConfirmScreen(ConfirmScreenMessage),
    ErrorScreen(ErrorScreenMessage),
    FontLoaded(Result<(), font::Error>),
}

#[derive(Default)]
struct ZCinema {
    media: Vec<Rc<RefCell<Serial>>>,
    screen: Screens,
    confirm_dialog: Dialog<ConfirmScreen<ConfirmKind>>,
    error_dialog: Dialog<ErrorScreen>,
    config: Rc<Config>,
}

impl ZCinema {
    fn change_serial_screen(&mut self, id: usize) {
        let serials = arr_rc_clone(&self.media);
        self.screen = Screens::change_serial(serials, id)
    }

    fn main_screen(&mut self) {
        let media = arr_rc_clone(&self.media);
        self.screen = Screens::main(media);
    }

    fn error_screen(&mut self, error: Error) {
        self.error_dialog = Dialog::new(error.into());
    }

    fn confirm_dialog(&mut self, kind: ConfirmKind) {
        let screen = ConfirmScreen::new(kind);
        self.confirm_dialog = Dialog::new(screen);
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

    fn sub_title(&self) -> Option<String> {
        if let Some(dialog) = self.error_dialog.as_ref() {
            return Some(dialog.title());
        }
        if let Some(dialog) = self.confirm_dialog.as_ref() {
            return Some(dialog.title());
        }
        self.screen.title()
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
                        let name = {
                            let serial = self.media[id].borrow();
                            serial.name().to_string()
                        };
                        self.confirm_dialog(ConfirmKind::DeleteSerial { id, name });
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
            Message::ConfirmScreen(message) => {
                match message {
                    ConfirmScreenMessage::Confirm => {
                        let Some(dialog) = self.confirm_dialog.as_ref() else {
                            return Ok(Command::none());
                        };
                        match dialog.kind() {
                            ConfirmKind::DeleteSerial { id, .. } => {
                                self.remove_serial(*id)?;
                                self.confirm_dialog.close();
                                self.main_screen();
                            }
                        }
                    }
                    ConfirmScreenMessage::Cancel => self.confirm_dialog.close(),
                }
                Ok(Command::none())
            }
            Message::FontLoaded(res) => {
                if matches!(res, Err(_)) {
                    self.error_screen(ErrorKind::FontLoad.into());
                }
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
        let main_window = Screens::main(arr_rc_clone(&media));
        Ok(Self {
            media,
            screen: main_window,
            confirm_dialog: Dialog::closed(),
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
        match Self::new2() {
            Ok(zcinema) => (
                zcinema,
                font::load(iced_aw::BOOTSTRAP_FONT_BYTES).map(Message::FontLoaded),
            ),
            Err(error) => (
                Self {
                    error_dialog: Dialog::new(error.into()),
                    ..Default::default()
                },
                Command::none(),
            ),
        }
    }

    fn title(&self) -> String {
        let program_name = "ZCinema";
        if let Some(sub_title) = self.sub_title() {
            format!("{} - {}", program_name, sub_title)
        } else {
            String::from(program_name)
        }
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
        let dialog = {
            if let Some(error_dialog) = self.error_dialog.as_ref() {
                Some(error_dialog.view().map(Message::ErrorScreen))
            } else if let Some(confirm_dialog) = self.confirm_dialog.as_ref() {
                Some(confirm_dialog.view().map(Message::ConfirmScreen))
            } else {
                None
            }
        };
        modal(self.screen.view(), dialog).into()
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

    fn title(&self) -> Option<String> {
        match self {
            Screens::MainWindow(_) => None,
            Screens::SerialChange(dialog) => Some(dialog.title()),
        }
    }

    fn main(media: Vec<Rc<RefCell<Serial>>>) -> Self {
        let dialog = MainScreen::new(media);
        Self::MainWindow(dialog)
    }

    fn change_serial(serials: Vec<Rc<RefCell<Serial>>>, id: usize) -> Self {
        let dialog = SerialEditScreen::new(serials, id);
        Self::SerialChange(dialog)
    }
}

impl Default for Screens {
    fn default() -> Self {
        Screens::MainWindow(MainScreen::default())
    }
}

enum ConfirmKind {
    DeleteSerial { name: String, id: usize },
}

impl Display for ConfirmKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfirmKind::DeleteSerial { name, .. } => {
                write!(
                    f,
                    "You actually wont to delete serial \"{}\" from the list?",
                    name
                )
            }
        }
    }
}
