#![windows_subsystem = "windows"] // Do not open console window on windows

mod config;
mod error;
mod gui;
mod series;
mod utils;
mod view_utils;

use std::{cell::RefCell, fmt::Display, rc::Rc};

use error::ErrorKind;
use iced::{executor, font, window, Application, Command, Element, Settings, Theme};
use iced_aw::modal;

use crate::{
    config::Config,
    error::Error,
    gui::{
        ConfirmScreen, ConfirmScreenMessage, Dialog, ErrorScreen, ErrorScreenMessage, MainScreen,
        MainScreenMessage, SeriesEditScreen, SeriesEditScreenMessage,
    },
    series::Series,
    utils::arr_rc_clone,
};

fn main() -> iced::Result {
    ZCinema::run(Settings::default())
}

#[derive(Debug, Clone)]
enum Message {
    MainScreen(MainScreenMessage),
    SeriesEditScreen(SeriesEditScreenMessage),
    ConfirmScreen(ConfirmScreenMessage),
    ErrorScreen(ErrorScreenMessage),
    FontLoaded(Result<(), font::Error>),
}

#[derive(Default)]
struct ZCinema {
    media: Vec<Rc<RefCell<Series>>>,
    screen: Screens,
    confirm_dialog: Dialog<ConfirmScreen<ConfirmKind>>,
    error_dialog: Dialog<ErrorScreen<Error>>,
    config: Rc<Config>,
}

impl ZCinema {
    fn change_series_screen(&mut self, id: usize) -> Result<(), ErrorKind> {
        let media = arr_rc_clone(&self.media);
        self.screen = Screens::change_series(media, id)?;
        Ok(())
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

    fn remove_series(&mut self, id: usize) -> Result<(), Error> {
        let series = &self.media[id];
        series.borrow().remove_file(&self.config.data_dir)?;
        self.media.remove(id);
        Ok(())
    }

    fn read_media(config: Rc<Config>) -> Result<Vec<Series>, ErrorKind> {
        let media = config
            .data_dir
            .exists()
            .then(|| utils::read_media(config))
            .transpose()?
            .unwrap_or_default();
        Ok(media)
    }

    fn close_app(&self) -> Command<Message> {
        window::close(window::Id::MAIN)
    }

    fn sub_title(&self) -> Option<String> {
        self.error_dialog
            .title()
            .or_else(|| self.confirm_dialog.title())
            .or_else(|| self.screen.title())
    }

    fn update2(&mut self, message: Message) -> Result<Command<Message>, Error> {
        match message {
            Message::MainScreen(message) => {
                match message {
                    MainScreenMessage::AddSeries => {
                        let series = Series::new(Rc::clone(&self.config))?;
                        let series = Rc::new(RefCell::new(series));
                        self.media.push(series);
                        self.change_series_screen(self.media.len() - 1)?;
                    }
                    MainScreenMessage::ChangeSeries(id) => self.change_series_screen(id)?,
                }
                Ok(Command::none())
            }
            Message::SeriesEditScreen(message) => {
                match message {
                    SeriesEditScreenMessage::Delete(id) => {
                        let series = &self.media[id];
                        let name = series.borrow().name().to_string();
                        self.confirm_dialog(ConfirmKind::DeleteSeries { id, name });
                    }
                    SeriesEditScreenMessage::Back => self.main_screen(),
                    SeriesEditScreenMessage::Watch { path } => utils::watch(path)?,
                    _ => {
                        if let Screens::SeriesChange(dialog) = &mut self.screen {
                            dialog.update(message)?;
                        }
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
                            ConfirmKind::DeleteSeries { id, .. } => {
                                self.remove_series(*id)?;
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
                    return Err(ErrorKind::FontLoad.into());
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
        let main_screen = Screens::main(arr_rc_clone(&media));
        Ok(Self {
            media,
            screen: main_screen,
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
        let program_name = "zCinema";
        self.sub_title()
            .map(|sub_title| format!("{} - {}", program_name, sub_title))
            .unwrap_or_else(|| String::from(program_name))
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        self.update2(message).unwrap_or_else(|error| {
            self.error_screen(error);
            Command::none()
        })
    }

    fn view(&self) -> Element<Message> {
        let dialog = self
            .error_dialog
            .view_into()
            .or_else(|| self.confirm_dialog.view_into());
        modal(self.screen.view(), dialog).into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

pub enum Screens {
    Main(MainScreen),
    SeriesChange(SeriesEditScreen),
}

impl Screens {
    fn view(&self) -> Element<Message> {
        match self {
            Screens::Main(dialog) => dialog.view().map(Into::into),
            Screens::SeriesChange(dialog) => dialog.view().map(Into::into),
        }
    }

    fn title(&self) -> Option<String> {
        match self {
            Screens::Main(_) => None,
            Screens::SeriesChange(dialog) => Some(dialog.title()),
        }
    }

    fn main(media: Vec<Rc<RefCell<Series>>>) -> Self {
        let dialog = MainScreen::new(media);
        Self::Main(dialog)
    }

    fn change_series(media: Vec<Rc<RefCell<Series>>>, id: usize) -> Result<Self, ErrorKind> {
        let dialog = SeriesEditScreen::new(media, id)?;
        Ok(Self::SeriesChange(dialog))
    }
}

impl Default for Screens {
    fn default() -> Self {
        Screens::Main(MainScreen::default())
    }
}

enum ConfirmKind {
    DeleteSeries { name: String, id: usize },
}

impl Display for ConfirmKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfirmKind::DeleteSeries { name, .. } => {
                write!(
                    f,
                    "You actually wont to delete series \"{}\" from the list?",
                    name
                )
            }
        }
    }
}

impl From<ConfirmScreenMessage> for Message {
    fn from(value: ConfirmScreenMessage) -> Self {
        Self::ConfirmScreen(value)
    }
}

impl From<ErrorScreenMessage> for Message {
    fn from(value: ErrorScreenMessage) -> Self {
        Self::ErrorScreen(value)
    }
}

impl From<SeriesEditScreenMessage> for Message {
    fn from(value: SeriesEditScreenMessage) -> Self {
        Self::SeriesEditScreen(value)
    }
}

impl From<MainScreenMessage> for Message {
    fn from(value: MainScreenMessage) -> Self {
        Self::MainScreen(value)
    }
}
