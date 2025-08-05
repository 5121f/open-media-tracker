/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

mod kinds;
mod message;
mod screens;

use std::sync::Arc;

use cosmic::app::Task;
use cosmic::iced::{executor, window};
use cosmic::widget::Popover;
use cosmic::{Action, Application, Core, Element};

use crate::gui::page::{
    ConfirmDlg, ConfirmPageMsg, ErrorPage, ErrorPageMsg, MainPage, MainPageMsg, MediaEditPageMsg,
};
use crate::gui::{Dialog, LoadingDialog, Page};
use crate::model::{Config, Error, ErrorKind, MediaHandler, MediaList, Placeholder};
use crate::utils;
use kinds::{ConfirmKind, LoadingKind};
pub use message::Msg;
use screens::Screens;

pub struct OpenMediaTracker {
    core: Core,
    media_list: MediaList,
    screen: Screens,
    confirm: ConfirmDlg<ConfirmKind>,
    error: Dialog<ErrorPage>,
    loading: LoadingDialog<LoadingKind>,
    config: Arc<Config>,
}

impl Application for OpenMediaTracker {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Msg;
    const APP_ID: &'static str = "com.open_media_tracker.zeroten";

    fn core(&self) -> &cosmic::Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut cosmic::Core {
        &mut self.core
    }

    fn init(core: Core, _flags: Self::Flags) -> (Self, Task<Self::Message>) {
        let config;
        let screen;
        match Config::read() {
            Ok(c) => {
                config = c;
                screen = Screens::default();
            }
            Err(err) => {
                config = Config::placeholder();
                screen = Screens::error(Error::fatal(err));
            }
        }
        let config = config.into();
        // TODO: Uncoment when header bar will be fixed
        // core.window.header_title = String::from("Open Media Tracker");
        let mut omt = Self {
            core,
            media_list: MediaList::new(),
            screen,
            confirm: ConfirmDlg::closed(),
            error: Dialog::closed(),
            loading: LoadingDialog::closed(),
            config,
        };
        let task = omt.read_media();
        (omt, task)
    }

    fn view(&self) -> Element<Self::Message> {
        if let Some(loading_screen) = self.loading.as_ref() {
            return loading_screen.view_into();
        }

        let dialog = self.error.as_ref().map_or_else(
            || self.confirm.as_ref().map(Page::view_into),
            |screen| Some(screen.view_into()),
        );

        let screen_view = self.screen.view(&self.media_list);

        if let Some(dialog) = dialog {
            return Popover::new(screen_view).popup(dialog).into();
        }

        screen_view
    }

    fn update(&mut self, message: Self::Message) -> Task<Self::Message> {
        match self.update2(message) {
            Ok(task) => return task,
            Err(err) => self.error_dialog(err),
        }
        Task::none()
    }
}

impl OpenMediaTracker {
    fn change_media_screen(&mut self, id: usize) -> Task<Msg> {
        let (screen, task) = Screens::change_media(&self.media_list, id);
        self.screen = screen;
        task.map(Action::App)
    }

    fn main_screen(&mut self) {
        self.screen = Screens::Main(MainPage::new(&self.media_list));
    }

    fn error_dialog(&mut self, error: Error) {
        self.error = Dialog::new(error.into());
    }

    fn confirm_dialog(&mut self, kind: ConfirmKind) {
        self.confirm = ConfirmDlg::from_kind(kind);
    }

    fn read_media(&mut self) -> Task<Msg> {
        self.loading.insert(LoadingKind::ReadMedia);
        let config = self.config.clone();
        cosmic::task::future(async move { Msg::MediaLoaded(MediaList::read(config).await) })
    }

    fn confirm_screen_update(&mut self, message: &ConfirmPageMsg) -> Result<(), ErrorKind> {
        match message {
            ConfirmPageMsg::Confirm => {
                if let Some(kind) = self.confirm.kind() {
                    self.confirm_kind_update(&kind.clone())?;
                }
            }
            ConfirmPageMsg::Cancel => self.confirm.close(),
        }
        Ok(())
    }

    fn confirm_kind_update(&mut self, kind: &ConfirmKind) -> Result<(), ErrorKind> {
        match kind {
            ConfirmKind::DeleteMedia { id, .. } => {
                self.media_list.remove(*id)?;
                self.confirm.close();
                self.main_screen();
            }
        }
        Ok(())
    }

    fn media_edit_screen_update(
        &mut self,
        message: MediaEditPageMsg,
    ) -> Result<Task<Msg>, ErrorKind> {
        match message {
            MediaEditPageMsg::Delete(id) => {
                let media = &self.media_list[id];
                let name = media.name().to_string();
                self.confirm_dialog(ConfirmKind::DeleteMedia { id, name });
            }
            MediaEditPageMsg::Back => self.main_screen(),
            MediaEditPageMsg::Watch { path } => utils::open(path)?,
            _ => {
                if let Screens::MediaChange(dialog) = &mut self.screen {
                    let task = dialog.update(&mut self.media_list, message)?;
                    return Ok(task.map(|m| Action::App(Msg::MediaEditScreen(m))));
                }
            }
        }
        Ok(Task::none())
    }

    fn main_screen_update(&mut self, message: MainPageMsg) -> Result<Task<Msg>, ErrorKind> {
        match message {
            MainPageMsg::AddMedia => {
                let config = self.config.clone();
                let media = MediaHandler::with_default_name(config)?;
                let new_media_index = self.media_list.insert(media);
                return Ok(self.change_media_screen(new_media_index));
            }
            MainPageMsg::SortButton
            | MainPageMsg::SearchBarChanged(_)
            | MainPageMsg::MenuButton(_) => {
                if let Screens::Main(screen) = &mut self.screen {
                    return Ok(screen
                        .update(message, &mut self.media_list)
                        .map(Action::App));
                }
            }
        }
        Ok(Task::none())
    }

    fn update2(&mut self, message: Msg) -> Result<Task<Msg>, Error> {
        match message {
            Msg::MainScreen(message) => return Ok(self.main_screen_update(message)?),
            Msg::MediaEditScreen(message) => return Ok(self.media_edit_screen_update(message)?),
            Msg::ErrorScreen(ErrorPageMsg::Ok { fatal }) => {
                if fatal {
                    return Ok(close_app());
                }
                self.error.close();
            }
            Msg::ConfirmScreen(message) => self.confirm_screen_update(&message)?,
            Msg::MediaLoaded(res) => {
                self.media_list = res.value;
                if let Some(err) = res.error {
                    self.error_dialog(err.into());
                }
                self.loading.complete(&LoadingKind::ReadMedia);
                if let Screens::Main(screen) = &mut self.screen {
                    screen.update_media(&self.media_list);
                }
            }
            Msg::Loading => {}
            Msg::SelectMedia(media_name) => {
                let selected_media_id = self
                    .media_list
                    .iter()
                    .enumerate()
                    .find(|(_id, media)| media.name() == media_name)
                    .map(|(id, _media)| id);
                if let Some(id) = selected_media_id {
                    return Ok(self.change_media_screen(id));
                }
            }
        }
        Ok(Task::none())
    }
}

fn close_app() -> Task<Msg> {
    window::get_latest().and_then(window::close)
}
