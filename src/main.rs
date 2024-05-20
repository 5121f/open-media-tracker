#![windows_subsystem = "windows"] // Do not open console window on startup on Windows

mod config;
mod error;
mod gui;
mod media;
mod message;
mod series;
mod utils;

use std::{
    fmt::{self, Display},
    sync::Arc,
};

use error::ErrorKind;
use gui::screen::error_title;
use iced::{executor, font, window, Application, Command, Element, Settings, Size, Theme};
use iced_aw::modal;

use crate::{
    config::Config,
    error::Error,
    gui::{
        screen::{
            error_view, main_screen_view, ConfirmScreen, ConfirmScreenMessage, ErrorScreenMessage,
            LoadingScreen, MainScreenMessage, SeriesEditScreen, SeriesEditScreenMessage,
        },
        Dialog,
    },
    media::Media,
    message::Message,
    series::Series,
};

fn main() -> iced::Result {
    ZCinema::run(Settings {
        window: window::Settings {
            size: Size::new(550., 400.),
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}

#[derive(Default)]
struct ZCinema {
    media: Media,
    screen: Screens,
    confirm_dialog: Dialog<ConfirmScreen<ConfirmKind>>,
    error: Option<Error>,
    loading_dialog: Dialog<LoadingScreen<LoadingKind>>,
    config: Arc<Config>,
}

impl ZCinema {
    fn change_series_screen(&mut self, id: usize) {
        self.screen = Screens::change_series(&self.media, id);
    }

    fn main_screen(&mut self) {
        self.screen = Screens::Main;
    }

    fn error_screen(&mut self, error: Error) {
        self.error = Some(error);
    }

    fn confirm_dialog(&mut self, kind: ConfirmKind) {
        let screen = ConfirmScreen::new(kind);
        self.confirm_dialog = Dialog::new(screen);
    }

    fn close_app(&self) -> Command<Message> {
        window::close(window::Id::MAIN)
    }

    fn sub_title(&self) -> Option<String> {
        self.error
            .as_ref()
            .map(|_| error_title())
            .or_else(|| self.confirm_dialog.title())
            .or_else(|| self.loading_dialog.as_ref().map(|d| d.title()))
            .or_else(|| self.title())
    }

    fn title(&self) -> Option<String> {
        match &self.screen {
            Screens::Main => None,
            Screens::SeriesChange(screen) => Some(screen.title(&self.media)),
        }
    }

    fn load_font(&mut self) -> Command<Message> {
        self.add_loading_process(LoadingKind::Font);
        font::load(iced_aw::BOOTSTRAP_FONT_BYTES).map(Message::FontLoaded)
    }

    fn read_media(&mut self) -> Command<Message> {
        self.add_loading_process(LoadingKind::ReadMedia);
        Command::perform(Media::read(Arc::clone(&self.config)), Message::MediaLoaded)
    }

    fn loading_complete(&mut self, kind: LoadingKind) {
        if let Some(loadnig_screen) = self.loading_dialog.as_mut() {
            loadnig_screen.complete(kind);
            if loadnig_screen.all_complete() {
                self.loading_dialog.close();
            }
        }
    }

    fn add_loading_process(&mut self, kind: LoadingKind) {
        match self.loading_dialog.as_mut() {
            Some(dialog) => dialog.insert(kind),
            None => {
                let mut screen = LoadingScreen::new();
                screen.insert(kind);
                self.loading_dialog = Dialog::new(screen);
            }
        }
    }

    fn confirm_screen_update(&mut self, message: ConfirmScreenMessage) -> Result<(), ErrorKind> {
        match message {
            ConfirmScreenMessage::Confirm => {
                if let Some(dialog) = self.confirm_dialog.take() {
                    self.confirm_kind_update(dialog.take())?;
                }
            }
            ConfirmScreenMessage::Cancel => self.confirm_dialog.close(),
        }
        Ok(())
    }

    fn confirm_kind_update(&mut self, kind: ConfirmKind) -> Result<(), ErrorKind> {
        match kind {
            ConfirmKind::DeleteSeries { id, .. } => {
                self.media.remove(id)?;
                self.confirm_dialog.close();
                self.main_screen();
            }
        }
        Ok(())
    }

    fn series_edit_screen_update(
        &mut self,
        message: SeriesEditScreenMessage,
    ) -> Result<(), ErrorKind> {
        match message {
            SeriesEditScreenMessage::Delete(id) => {
                let series = &self.media[id];
                let name = series.name().to_string();
                self.confirm_dialog(ConfirmKind::DeleteSeries { id, name });
            }
            SeriesEditScreenMessage::Back => self.main_screen(),
            SeriesEditScreenMessage::Watch { path } => utils::open(path)?,
            _ => {
                if let Screens::SeriesChange(dialog) = &mut self.screen {
                    dialog.update(&mut self.media, message)?;
                }
            }
        }
        Ok(())
    }

    fn main_screen_update(&mut self, message: MainScreenMessage) -> Result<(), ErrorKind> {
        match message {
            MainScreenMessage::AddSeries => {
                let series = Series::new(Arc::clone(&self.config))?;
                self.media.push(series);
                self.change_series_screen(self.media.len() - 1);
            }
            MainScreenMessage::MenuButton(gui::ListMessage::Enter(id)) => {
                self.change_series_screen(id)
            }
        }
        Ok(())
    }

    fn update2(&mut self, message: Message) -> Result<Command<Message>, Error> {
        match message {
            Message::MainScreen(message) => self.main_screen_update(message)?,
            Message::SeriesEditScreen(message) => self.series_edit_screen_update(message)?,
            Message::ErrorScreen(ErrorScreenMessage::Ok { critical }) => {
                if critical {
                    return Ok(self.close_app());
                }
                self.error = None;
            }
            Message::ConfirmScreen(message) => self.confirm_screen_update(message)?,
            Message::FontLoaded(res) => {
                res.map_err(|_| ErrorKind::FontLoad)?;
                self.loading_complete(LoadingKind::Font);
            }
            Message::MediaLoaded(res) => {
                self.media = res?.into();
                self.loading_complete(LoadingKind::ReadMedia)
            }
        }
        Ok(Command::none())
    }

    fn new2() -> Result<(Self, Command<Message>), Error> {
        let config = Config::read().map_err(|kind| Error::critical(kind))?;
        let config = Arc::new(config);
        let mut zcinema = Self {
            media: Media::new(),
            screen: Screens::Main,
            confirm_dialog: Dialog::closed(),
            error: None,
            loading_dialog: Dialog::closed(),
            config,
        };
        let command = Command::batch(vec![zcinema.load_font(), zcinema.read_media()]);
        Ok((zcinema, command))
    }
}

impl Application for ZCinema {
    type Executor = executor::Default;
    type Theme = Theme;
    type Flags = ();
    type Message = Message;

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        Self::new2().unwrap_or_else(|error| {
            (
                Self {
                    error: Some(error),
                    ..Default::default()
                },
                Command::none(),
            )
        })
    }

    fn title(&self) -> String {
        let program_name = "zCinema";
        self.sub_title()
            .map(|sub_title| format!("{program_name} - {sub_title}"))
            .unwrap_or_else(|| String::from(program_name))
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        self.update2(message).unwrap_or_else(|error| {
            self.error_screen(error);
            Command::none()
        })
    }

    fn view(&self) -> Element<Message> {
        if let Some(loading_screen) = self.loading_dialog.as_ref() {
            return loading_screen.view();
        }

        let dialog = self
            .error
            .as_ref()
            .map(|error| error_view(error).map(Into::into))
            .or_else(|| self.confirm_dialog.view_into());

        let screen = match &self.screen {
            Screens::Main => main_screen_view(&self.media).map(Into::into),
            Screens::SeriesChange(screen) => screen.view(&self.media).map(Into::into),
        };

        modal(screen, dialog).into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

pub enum Screens {
    Main,
    SeriesChange(SeriesEditScreen),
}

impl Screens {
    fn change_series(media: &[Series], id: usize) -> Self {
        let screen = SeriesEditScreen::new(media, id);
        Self::SeriesChange(screen)
    }
}

#[derive(PartialEq, Eq, Hash)]
pub enum LoadingKind {
    Font,
    ReadMedia,
}

impl Default for Screens {
    fn default() -> Self {
        Self::Main
    }
}

enum ConfirmKind {
    DeleteSeries { name: String, id: usize },
}

impl Display for ConfirmKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConfirmKind::DeleteSeries { name, .. } => {
                write!(
                    f,
                    "You actually want to delete series \"{name}\" from the list?",
                )
            }
        }
    }
}
