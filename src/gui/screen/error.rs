/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use cosmic::Element;
use cosmic::widget::{button, dialog};
use derive_more::From;

use crate::gui::Screen;
use crate::model::Error;

#[derive(Debug, Clone)]
pub enum Msg {
    Ok { fatal: bool },
}

impl Msg {
    const fn ok(fatal: bool) -> Self {
        Self::Ok { fatal }
    }
}

#[derive(From)]
pub struct ErrorScrn {
    error: Error,
}

impl Screen for ErrorScrn {
    type Message = Msg;

    fn view(&self) -> Element<Msg> {
        dialog()
            .title(if self.error.fatal {
                "Fatal error"
            } else {
                "Error"
            })
            .body(self.error.to_string())
            .primary_action(button::suggested("Ok").on_press(Msg::ok(self.error.fatal)))
            .into()
    }
}
