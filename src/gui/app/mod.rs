/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

mod confirm_kind;
mod loading_kind;
mod message;
mod screens;

use std::sync::Arc;

use cosmic::app::Task;
use cosmic::iced::{executor, window};
use cosmic::widget::Popover;
use cosmic::{Action, Application, Core, Element};

use crate::gui::screen::{
    ConfirmDlg, ConfirmScrnMsg, ErrorScrn, ErrorScrnMsg, MainScrn, MainScrnMsg, MediaEditScrnMsg,
};
use crate::gui::{Dialog, LoadingDialog, Screen};
use crate::model::{Config, Error, ErrorKind, MaybeError, MediaHandler, MediaList, Placeholder};
use crate::utils;
use confirm_kind::ConfirmKind;
use loading_kind::LoadingKind;
pub use message::Msg;
use screens::Screens;

pub struct OpenMediaTracker {
    core: Core,
    media_list: MediaList,
    screen: Screens,
    confirm: ConfirmDlg<ConfirmKind>,
    error: Dialog<ErrorScrn>,
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
            || self.confirm.as_ref().map(Screen::view_into),
            |screen| Some(screen.view_into()),
        );

        let screen_view = self.screen.view(&self.media_list);

        if let Some(dialog) = dialog {
            return Popover::new(screen_view).popup(dialog).into();
        }

        screen_view
    }

    fn update(&mut self, message: Self::Message) -> Task<Self::Message> {
        let res = self.update2(message);
        if let Some(error) = res.error {
            self.error_dialog(error);
        }
        res.value
    }
}

impl OpenMediaTracker {
    fn change_media_screen(&mut self, id: usize) -> Task<Msg> {
        let (screen, task) = Screens::change_media(&self.media_list, id);
        self.screen = screen;
        task.map(Action::App)
    }

    fn main_screen(&mut self) {
        self.screen = Screens::Main(MainScrn::new(&self.media_list));
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
                self.media_list.remove(*id)?;
                self.confirm.close();
                self.main_screen();
            }
        }
        Ok(())
    }

    fn media_edit_screen_update(
        &mut self,
        message: MediaEditScrnMsg,
    ) -> Result<Task<Msg>, ErrorKind> {
        match message {
            MediaEditScrnMsg::Delete(id) => {
                let media = &self.media_list[id];
                let name = media.name().to_string();
                self.confirm_dialog(ConfirmKind::DeleteMedia { id, name });
            }
            MediaEditScrnMsg::Back => self.main_screen(),
            MediaEditScrnMsg::Watch { path } => utils::open(path)?,
            _ => {
                if let Screens::MediaChange(dialog) = &mut self.screen {
                    let task = dialog.update(&mut self.media_list, message)?;
                    return Ok(task.map(|m| Action::App(Msg::MediaEditScreen(m))));
                }
            }
        }
        Ok(Task::none())
    }

    fn main_screen_update(&mut self, message: MainScrnMsg) -> Result<Task<Msg>, ErrorKind> {
        match message {
            MainScrnMsg::AddMedia => {
                let config = self.config.clone();
                let media = MediaHandler::with_default_name(config)?;
                let new_media_index = self.media_list.insert(media);
                return Ok(self.change_media_screen(new_media_index));
            }
            MainScrnMsg::MenuButton(entity) => {
                let Screens::Main(screen) = &self.screen else {
                    return Ok(Task::none());
                };
                let Some(selected_media_name) = screen.selected(entity) else {
                    return Ok(Task::none());
                };
                let selected_media_id = self
                    .media_list
                    .iter()
                    .enumerate()
                    .find(|(_id, media)| media.name() == selected_media_name)
                    .map(|(id, _media)| id);
                if let Some(id) = selected_media_id {
                    return Ok(self.change_media_screen(id));
                }
            }
            MainScrnMsg::SortButton | MainScrnMsg::SearchBarChanged(_) => {
                if let Screens::Main(screen) = &mut self.screen {
                    screen.update(message, &mut self.media_list);
                }
            }
        }
        Ok(Task::none())
    }

    fn update2(&mut self, message: Msg) -> MaybeError<Task<Msg>, Error> {
        let mut error = None;

        match message {
            Msg::MainScreen(message) => match self.main_screen_update(message) {
                Ok(task) => return MaybeError::success(task),
                Err(err) => error = Some(err.into()),
            },
            Msg::MediaEditScreen(message) => match self.media_edit_screen_update(message) {
                Ok(task) => return MaybeError::success(task),
                Err(err) => error = Some(err.into()),
            },
            Msg::ErrorScreen(ErrorScrnMsg::Ok { fatal: critical }) => {
                if critical {
                    return MaybeError::success(close_app());
                }
                self.error.close();
            }
            Msg::ConfirmScreen(message) => match self.confirm_screen_update(&message) {
                Ok(()) => {}
                Err(err) => error = Some(err.into()),
            },
            Msg::MediaLoaded(res) => {
                self.media_list = res.value;
                error = res.error.map(Into::into);
                self.loading.complete(&LoadingKind::ReadMedia);
                if let Screens::Main(screen) = &mut self.screen {
                    screen.update_media(&self.media_list);
                }
            }
            Msg::Loading => {}
        }

        MaybeError {
            value: Task::none(),
            error,
        }
    }
}

fn close_app() -> Task<Msg> {
    window::get_latest().and_then(window::close)
}
