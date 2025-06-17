/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use cosmic::Element;
use cosmic::iced_widget::center;
use cosmic::widget::{button, dialog};
use derive_more::From;

use crate::gui::Screen;
use crate::model::Error;

#[derive(Debug, Clone)]
pub enum Msg {
    Ok { critical: bool },
}

impl Msg {
    const fn ok(critical: bool) -> Self {
        Self::Ok { critical }
    }
}

#[derive(From)]
pub struct ErrorScrn {
    error: Error,
}

impl Screen for ErrorScrn {
    type Message = Msg;

    fn view(&self) -> Element<Msg> {
        center(
            dialog()
                .title("Error")
                .body(self.error.to_string())
                .primary_action(
                    if self.error.critical {
                        button::destructive("Ok")
                    } else {
                        button::suggested("Ok")
                    }
                    .on_press(Msg::ok(self.error.critical)),
                ),
        )
        .into()
    }
}
