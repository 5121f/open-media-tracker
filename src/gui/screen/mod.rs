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

pub use confirm::{ConfirmDlg, Msg as ConfirmScrnMsg};
pub use error::{ErrorScrn, Msg as ErrorScrnMsg};
pub use loading::{LoadingScrn, Msg as LoadingMsg};
pub use main::{MainScrn, Msg as MainScrnMsg};
pub use media_edit::{MediaEditScrn, Msg as MediaEditScrnMsg};
pub use warning::{Message as WarningMsg, WarningDlg};

use cosmic::Element;

pub trait Screen {
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
