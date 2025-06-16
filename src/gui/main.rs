/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

mod confirm_kind;
mod loading_kind;
mod screens;

use std::sync::Arc;

use cosmic::app::Task;
use cosmic::iced::{executor, window};
use cosmic::widget::Popover;
use cosmic::{Action, Application, Core, Element};

use crate::gui::screen::ConfirmDlg;
use crate::gui::screen::ConfirmScrnMsg;
use crate::gui::screen::ErrorScrn;
use crate::gui::screen::ErrorScrnMsg;
use crate::gui::screen::MainScrn;
use crate::gui::screen::MainScrnMsg;
use crate::gui::screen::MediaEditScrnMsg;
use crate::gui::{Dialog, LoadingDialog, Screen};
use crate::message::Msg;
use crate::model::{Config, Error, ErrorKind, MaybeError, MediaHandler, MediaList, Placeholder};
use confirm_kind::ConfirmKind;
use loading_kind::LoadingKind;
use screens::Screens;

pub struct OpenMediaTracker {
    core: Core,
    media: MediaList,
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

    fn init(core: cosmic::Core, _flags: Self::Flags) -> (Self, Task<Self::Message>) {
        Self::new(core)
    }

    fn view(&self) -> Element<Self::Message> {
        self.view()
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
    fn change_media_screen(&mut self, id: usize) {
        self.screen = Screens::change_media(&self.media, id);
    }

    fn main_screen(&mut self) {
        self.screen = Screens::Main(MainScrn::new(&self.media));
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
        let read_media_future = MediaList::read(config);
        Task::perform(read_media_future, |res| Action::App(Msg::MediaLoaded(res)))
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
            MainScrnMsg::MenuButton(entity) => {
                let Screens::Main(screen) = &self.screen else {
                    return Ok(());
                };
                let Some(selected_media_name) = screen.selected(*entity) else {
                    return Ok(());
                };
                let selected_media_id = self
                    .media
                    .iter()
                    .enumerate()
                    .find(|(_id, media)| media.name() == selected_media_name)
                    .map(|(id, _media)| id);
                if let Some(id) = selected_media_id {
                    self.change_media_screen(id);
                }
            }
            MainScrnMsg::SortButton => {
                if let Screens::Main(screen) = &mut self.screen {
                    screen.sort(&mut self.media);
                }
            }
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
                if let Screens::Main(screen) = &mut self.screen {
                    screen.update_media(&self.media);
                }
            }
            Msg::Loading => {}
        }

        MaybeError {
            value: Task::none(),
            error,
        }
    }

    fn new2(core: Core) -> Result<(Self, Task<Msg>), Error> {
        let config = Config::read().map_err(Error::critical)?;
        let config = config.into();
        let mut omt = Self {
            core,
            media: MediaList::new(),
            screen: Screens::default(),
            confirm: ConfirmDlg::closed(),
            error: Dialog::closed(),
            loading: LoadingDialog::closed(),
            config,
        };
        let command = Task::batch(vec![omt.read_media()]);
        Ok((omt, command))
    }

    pub fn new(core: Core) -> (Self, Task<Msg>) {
        Self::new2(core).unwrap_or_else(|error| {
            let mut omt = Self::placeholder();
            omt.error_dialog(error);
            (omt, Task::none())
        })
    }

    pub fn view(&self) -> Element<Msg> {
        // Error -> Loading -> Confirm

        let dialog = match (
            self.error.as_ref(),
            self.loading.as_ref(),
            self.confirm.as_ref(),
        ) {
            (Some(error_screen), _, _) => Some(error_screen.view_into()),
            (None, Some(loading_screen), _) => return loading_screen.view_into(),
            (None, None, Some(confirm_screen)) => Some(confirm_screen.view_into()),
            (None, None, None) => None,
        };

        let screen_view = self.screen.view(&self.media);

        if let Some(dialog) = dialog {
            return Popover::new(screen_view).popup(dialog).into();
        }

        screen_view
    }
}

impl Placeholder for OpenMediaTracker {
    fn placeholder() -> Self {
        Self {
            core: Core::default(),
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
