/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::{
    fmt::{self, Display},
    sync::Arc,
};

use iced::{executor, font, window, Application, Command, Element, Theme};
use iced_aw::modal;

use crate::{
    gui::{
        screen::{
            self, main_screen_view, ConfirmScreenMessage, ErrorScreen, ErrorScreenMessage,
            MainScreenMessage, MediaEditScreen, MediaEditScreenMessage,
        },
        Dialog, IDialog, ListMessage,
    },
    message::Message,
    model::{self, Config, Error, ErrorKind, MediaHandler, MediaList},
    utils,
};

#[derive(Default)]
pub struct OpenMediaTracker {
    media: MediaList,
    screen: Screens,
    confirm_dialog: Dialog<ConfirmScreen>,
    error: Dialog<ErrorScreen>,
    loading: LoadingDialog,
    config: Arc<Config>,
}

impl OpenMediaTracker {
    fn change_media_screen(&mut self, id: usize) {
        self.screen = Screens::change_media(&self.media, id);
    }

    fn main_screen(&mut self) {
        self.screen = Screens::Main;
    }

    fn error_dialog(&mut self, error: Error) {
        let screen = ErrorScreen::new(error);
        self.error = Dialog::new(screen);
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
            .title()
            .or_else(|| self.confirm_dialog.title())
            .or_else(|| self.loading.title())
            .or_else(|| self.screen_title())
    }

    fn screen_title(&self) -> Option<String> {
        match &self.screen {
            Screens::Main => None,
            Screens::MediaChange(screen) => Some(screen.title(&self.media)),
        }
    }

    fn load_font(&mut self) -> Command<Message> {
        self.loading.insert(LoadingKind::Font);
        font::load(iced_aw::BOOTSTRAP_FONT_BYTES).map(Message::FontLoaded)
    }

    fn read_media(&mut self) -> Command<Message> {
        self.loading.insert(LoadingKind::ReadMedia);
        let config = self.config.clone();
        let read_media_future = MediaList::read(config);
        Command::perform(read_media_future, Message::MediaLoaded)
    }

    fn confirm_screen_update(&mut self, message: ConfirmScreenMessage) -> Result<(), ErrorKind> {
        match message {
            ConfirmScreenMessage::Confirm => {
                if let Some(kind) = self.confirm_dialog.kind() {
                    self.confirm_kind_update(kind.clone())?;
                }
            }
            ConfirmScreenMessage::Cancel => self.confirm_dialog.close(),
        }
        Ok(())
    }

    fn confirm_kind_update(&mut self, kind: ConfirmKind) -> Result<(), ErrorKind> {
        match kind {
            ConfirmKind::DeleteMedia { id, .. } => {
                self.media.remove(id)?;
                self.confirm_dialog.close();
                self.main_screen();
            }
        }
        Ok(())
    }

    fn media_edit_screen_update(
        &mut self,
        message: MediaEditScreenMessage,
    ) -> Result<(), ErrorKind> {
        match message {
            MediaEditScreenMessage::Delete(id) => {
                let media = &self.media[id];
                let name = media.name().to_string();
                self.confirm_dialog(ConfirmKind::DeleteMedia { id, name });
            }
            MediaEditScreenMessage::Back => self.main_screen(),
            MediaEditScreenMessage::Watch { path } => utils::open(path)?,
            _ => {
                if let Screens::MediaChange(dialog) = &mut self.screen {
                    dialog.update(&mut self.media, message)?;
                }
            }
        }
        Ok(())
    }

    fn main_screen_update(&mut self, message: MainScreenMessage) -> Result<(), ErrorKind> {
        match message {
            MainScreenMessage::AddMedia => {
                let config = self.config.clone();
                let media = MediaHandler::with_default_name(config);
                self.media.push(media);
                self.change_media_screen(self.media.len() - 1);
            }
            MainScreenMessage::MenuButton(ListMessage::Enter(id)) => self.change_media_screen(id),
        }
        Ok(())
    }

    fn update2(&mut self, message: Message) -> Result<Command<Message>, Error> {
        match message {
            Message::MainScreen(message) => self.main_screen_update(message)?,
            Message::MediaEditScreen(message) => self.media_edit_screen_update(message)?,
            Message::ErrorScreen(ErrorScreenMessage::Ok { critical }) => {
                if critical {
                    return Ok(self.close_app());
                }
                self.error.close();
            }
            Message::ConfirmScreen(message) => self.confirm_screen_update(message)?,
            Message::FontLoaded(res) => {
                res.map_err(|_| ErrorKind::FontLoad)?;
                self.loading.complete(LoadingKind::Font);
            }
            Message::MediaLoaded(res) => {
                self.media = res.map_err(Into::<ErrorKind>::into)?;
                self.loading.complete(LoadingKind::ReadMedia);
            }
            Message::Loading => {}
        }
        Ok(Command::none())
    }

    fn new2() -> Result<(Self, Command<Message>), Error> {
        let config = Config::read().map(Arc::new).map_err(Error::critical)?;
        let mut omt = Self {
            media: MediaList::new(),
            screen: Screens::Main,
            confirm_dialog: Dialog::closed(),
            error: Dialog::closed(),
            loading: LoadingDialog::closed(),
            config,
        };
        let command = Command::batch(vec![omt.load_font(), omt.read_media()]);
        Ok((omt, command))
    }
}

impl Application for OpenMediaTracker {
    type Executor = executor::Default;
    type Theme = Theme;
    type Flags = ();
    type Message = Message;

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        Self::new2().unwrap_or_else(|error| {
            let mut omt = Self::default();
            omt.error_dialog(error);
            (omt, Command::none())
        })
    }

    fn title(&self) -> String {
        let program_name = "Open Media Tracker";
        self.sub_title()
            .map(|sub_title| format!("{program_name} - {sub_title}"))
            .unwrap_or_else(|| String::from(program_name))
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        self.update2(message).unwrap_or_else(|error| {
            self.error_dialog(error);
            Command::none()
        })
    }

    fn view(&self) -> Element<Message> {
        let dialog = self
            .error
            .view_into()
            .or_else(|| self.confirm_dialog.view_into());

        if dialog.is_some() {
            return modal(self.screen.view(&self.media), dialog).into();
        }

        if let Some(loading_screen) = self.loading.as_ref() {
            return loading_screen.view_into();
        }

        self.screen.view(&self.media)
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

pub enum Screens {
    Main,
    MediaChange(MediaEditScreen),
}

impl Screens {
    fn view<'a>(&'a self, media: &'a MediaList) -> Element<Message> {
        match self {
            Self::Main => main_screen_view(media).map(Into::into),
            Self::MediaChange(screen) => screen.view(media).map(Into::into),
        }
    }

    fn change_media(media: &[MediaHandler], id: usize) -> Self {
        let screen = MediaEditScreen::new(media, id);
        Self::MediaChange(screen)
    }
}

impl Default for Screens {
    fn default() -> Self {
        Self::Main
    }
}

#[derive(PartialEq, Eq, Hash)]
pub enum LoadingKind {
    Font,
    ReadMedia,
}

impl model::LoadingKind for LoadingKind {}

type LoadingDialog = super::loading::LoadingDialog<LoadingKind>;

#[derive(Clone)]
enum ConfirmKind {
    DeleteMedia { name: String, id: usize },
}

impl Display for ConfirmKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConfirmKind::DeleteMedia { name, .. } => {
                write!(
                    f,
                    "You actually want to delete media \"{name}\" from the list?",
                )
            }
        }
    }
}

type ConfirmScreen = screen::ConfirmScreen<ConfirmKind>;
