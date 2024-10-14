/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::{
    fmt::{self, Display},
    sync::Arc,
};

use iced::{widget::stack, window, Element, Task, Theme};

use crate::{
    gui::{
        screen::{
            main_screen_view, ConfirmScreen, ConfirmScreenMessage, ErrorScreen, ErrorScreenMessage,
            MainScreenMessage, MediaEditScreen, MediaEditScreenMessage,
        },
        Closable, Dialog, ListMessage, LoadingDialog,
    },
    message::Message,
    model::{self, Config, Error, ErrorKind, MediaHandler, MediaList, Placeholder},
    utils,
};

pub struct OpenMediaTracker {
    media: MediaList,
    screen: Screens,
    confirm_dialog: Closable<ConfirmScreen<ConfirmKind>>,
    error: Closable<ErrorScreen>,
    loading: LoadingDialog<LoadingKind>,
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
        self.error = Closable::new(screen);
    }

    fn confirm_dialog(&mut self, kind: ConfirmKind) {
        let screen = ConfirmScreen::new(kind);
        self.confirm_dialog = Closable::new(screen);
    }

    fn close_app(&self) -> Task<Message> {
        window::get_latest().and_then(window::close)
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

    fn read_media(&mut self) -> Task<Message> {
        self.loading.insert(LoadingKind::ReadMedia);
        let config = self.config.clone();
        let read_media_future = MediaList::read(config);
        Task::perform(read_media_future, Message::MediaLoaded)
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
                let media = MediaHandler::with_default_name(config)?;
                self.media.push(media);
                self.change_media_screen(self.media.len() - 1);
            }
            MainScreenMessage::MenuButton(ListMessage::Enter(id)) => self.change_media_screen(id),
        }
        Ok(())
    }

    fn update2(&mut self, message: Message) -> Result<Task<Message>, Error> {
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
            Message::MediaLoaded(res) => {
                self.media = res.map_err(Into::<ErrorKind>::into)?;
                self.loading.complete(LoadingKind::ReadMedia);
            }
            Message::Loading => {}
        }
        Ok(Task::none())
    }

    fn new2() -> Result<(Self, Task<Message>), Error> {
        let config = Config::read().map(Arc::new).map_err(Error::critical)?;
        let mut omt = Self {
            media: MediaList::new(),
            screen: Screens::Main,
            confirm_dialog: Closable::closed(),
            error: Closable::closed(),
            loading: LoadingDialog::closed(),
            config,
        };
        let command = Task::batch(vec![omt.read_media()]);
        Ok((omt, command))
    }

    pub fn new() -> (Self, Task<Message>) {
        Self::new2().unwrap_or_else(|error| {
            let mut omt = Self::placeholder();
            omt.error_dialog(error);
            (omt, Task::none())
        })
    }

    pub fn title(&self) -> String {
        let program_name = "Open Media Tracker";
        self.sub_title()
            .map(|sub_title| format!("{program_name} - {sub_title}"))
            .unwrap_or_else(|| String::from(program_name))
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        self.update2(message).unwrap_or_else(|error| {
            self.error_dialog(error);
            Task::none()
        })
    }

    pub fn view(&self) -> Element<Message> {
        let dialog = self
            .error
            .view_into()
            .or_else(|| self.confirm_dialog.view_into());

        if let Some(dialog) = dialog {
            return stack!(self.screen.view(&self.media), dialog).into();
        }

        if let Some(loading_screen) = self.loading.as_ref() {
            return loading_screen.view_into();
        }

        self.screen.view(&self.media)
    }

    pub fn theme(&self) -> Theme {
        Theme::Dark
    }
}

impl Placeholder for OpenMediaTracker {
    fn placeholder() -> Self {
        Self {
            media: MediaList::placeholder(),
            screen: Screens::Main,
            confirm_dialog: Closable::closed(),
            error: Closable::closed(),
            loading: LoadingDialog::closed(),
            config: Config::placeholder().into(),
        }
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
    ReadMedia,
}

impl model::LoadingKind for LoadingKind {}

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
