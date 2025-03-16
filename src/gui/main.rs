/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

mod confirm_kind;
mod loading_kind;
mod screens;

use std::sync::Arc;

use iced::widget::Stack;
use iced::{Element, Task, Theme, window};

use crate::gui::screen::ConfirmDlg;
use crate::gui::screen::{ConfirmScrnMsg, ErrorScrn, ErrorScrnMsg, MainScrnMsg, MediaEditScrnMsg};
use crate::gui::{Dialog, ListMsg, LoadingDialog, Screen};
use crate::message::Msg;
use crate::model::{Config, Error, ErrorKind, MaybeError, MediaHandler, MediaList, Placeholder};
use confirm_kind::ConfirmKind;
use loading_kind::LoadingKind;
use screens::Screens;

const PROGRAM_NAME: &str = "Open Media Tracker";

pub struct OpenMediaTracker {
    media: MediaList,
    screen: Screens,
    confirm: ConfirmDlg<ConfirmKind>,
    error: Dialog<ErrorScrn>,
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
        self.error = Dialog::new(error.into());
    }

    fn confirm_dialog(&mut self, kind: ConfirmKind) {
        self.confirm = ConfirmDlg::from_kind(kind);
    }

    fn sub_title(&self) -> Option<String> {
        self.error
            .title()
            .or_else(|| self.confirm.title())
            .or_else(|| self.loading.title())
            .or_else(|| self.screen.title(&self.media))
    }

    fn read_media(&mut self) -> Task<Msg> {
        self.loading.insert(LoadingKind::ReadMedia);
        let config = self.config.clone();
        let read_media_future = MediaList::read(config);
        Task::perform(read_media_future, Msg::MediaLoaded)
    }

    fn confirm_screen_update(&mut self, message: &ConfirmScrnMsg) -> Result<(), ErrorKind> {
        match message {
            ConfirmScrnMsg::Confirm => {
                if let Some(kind) = self.confirm.kind() {
                    self.confirm_kind_update(&kind.clone())?;
                }
            }
            ConfirmScrnMsg::Cancel => self.confirm.close(),
        }
        Ok(())
    }

    fn confirm_kind_update(&mut self, kind: &ConfirmKind) -> Result<(), ErrorKind> {
        match kind {
            ConfirmKind::DeleteMedia { id, .. } => {
                self.media.remove(*id)?;
                self.confirm.close();
                self.main_screen();
            }
        }
        Ok(())
    }

    fn media_edit_screen_update(&mut self, message: MediaEditScrnMsg) -> Result<(), ErrorKind> {
        match message {
            MediaEditScrnMsg::Delete(id) => {
                let media = &self.media[id];
                let name = media.name().to_string();
                self.confirm_dialog(ConfirmKind::DeleteMedia { id, name });
            }
            MediaEditScrnMsg::Back => self.main_screen(),
            MediaEditScrnMsg::Watch { path } => crate::open(path)?,
            _ => {
                if let Screens::MediaChange(dialog) = &mut self.screen {
                    dialog.update(&mut self.media, message)?;
                }
            }
        }
        Ok(())
    }

    fn main_screen_update(&mut self, message: &MainScrnMsg) -> Result<(), ErrorKind> {
        match message {
            MainScrnMsg::AddMedia => {
                let config = self.config.clone();
                let media = MediaHandler::with_default_name(config)?;
                let new_media_index = self.media.insert(media);
                self.change_media_screen(new_media_index);
            }
            MainScrnMsg::MenuButton(ListMsg::Enter(id)) => self.change_media_screen(*id),
        }
        Ok(())
    }

    fn update2(&mut self, message: Msg) -> MaybeError<Task<Msg>, Error> {
        let mut error = None;

        match message {
            Msg::MainScreen(message) => {
                if let Err(err) = self.main_screen_update(&message) {
                    error = Some(err.into());
                }
            }
            Msg::MediaEditScreen(message) => {
                if let Err(err) = self.media_edit_screen_update(message) {
                    error = Some(err.into());
                }
            }
            Msg::ErrorScreen(ErrorScrnMsg::Ok { critical }) => {
                if critical {
                    return MaybeError::success(close_app());
                }
                self.error.close();
            }
            Msg::ConfirmScreen(message) => {
                if let Err(err) = self.confirm_screen_update(&message) {
                    error = Some(err.into());
                }
            }
            Msg::MediaLoaded(res) => {
                self.media = res.value;
                error = res.error.map(Into::into);
                self.loading.complete(&LoadingKind::ReadMedia);
            }
            Msg::Loading => {}
        }

        MaybeError {
            value: Task::none(),
            error,
        }
    }

    fn new2() -> Result<(Self, Task<Msg>), Error> {
        let config = Config::read().map_err(Error::critical)?;
        let config = config.into();
        let mut omt = Self {
            media: MediaList::new(),
            screen: Screens::Main,
            confirm: ConfirmDlg::closed(),
            error: Dialog::closed(),
            loading: LoadingDialog::closed(),
            config,
        };
        let command = Task::batch(vec![omt.read_media()]);
        Ok((omt, command))
    }

    pub fn new() -> (Self, Task<Msg>) {
        Self::new2().unwrap_or_else(|error| {
            let mut omt = Self::placeholder();
            omt.error_dialog(error);
            (omt, Task::none())
        })
    }

    pub fn title(&self) -> String {
        self.sub_title().map_or_else(
            || String::from(PROGRAM_NAME),
            |sub_title| format!("{PROGRAM_NAME} - {sub_title}"),
        )
    }

    pub fn update(&mut self, message: Msg) -> Task<Msg> {
        let res = self.update2(message);
        if let Some(error) = res.error {
            self.error_dialog(error);
        }
        res.value
    }

    pub fn view(&self) -> Element<Msg> {
        if let Some(loading_screen) = self.loading.as_ref() {
            return loading_screen.view_into();
        }

        let dialog = self.error.view_into().or_else(|| self.confirm.view_into());

        Stack::new()
            .push(self.screen.view(&self.media))
            .push_maybe(dialog)
            .into()
    }

    pub const fn theme(_: &Self) -> Theme {
        Theme::Dark
    }
}

impl Placeholder for OpenMediaTracker {
    fn placeholder() -> Self {
        Self {
            media: MediaList::placeholder(),
            screen: Screens::placeholder(),
            confirm: ConfirmDlg::placeholder(),
            error: Dialog::placeholder(),
            loading: LoadingDialog::placeholder(),
            config: Config::placeholder().into(),
        }
    }
}

fn close_app() -> Task<Msg> {
    window::get_latest().and_then(window::close)
}
