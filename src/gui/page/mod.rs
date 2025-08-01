/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

pub mod confirm;
pub mod error;
pub mod loading;
pub mod main;
pub mod media_edit;
pub mod warning;

pub use confirm::{ConfirmDlg, Msg as ConfirmPageMsg};
pub use error::{ErrorPage, Msg as ErrorPageMsg};
pub use loading::{LoadingPage, Msg as LoadingPageMsg};
pub use main::{MainPage, Msg as MainPageMsg};
pub use media_edit::{MediaEditPage, Msg as MediaEditPageMsg};
pub use warning::{Msg as WarningPageMsg, WarningDlg};

use cosmic::Element;

pub trait Page {
    type Message;

    fn view(&self) -> Element<Self::Message>;

    fn view_map<'a, B: 'a, F>(&'a self, f: F) -> Element<'a, B>
    where
        F: Fn(Self::Message) -> B + 'a,
    {
        self.view().map(f)
    }

    fn view_into<'a, M>(&'a self) -> Element<'a, M>
    where
        M: From<Self::Message> + 'a,
    {
        self.view_map(Into::into)
    }
}
