// SPDX-License-Identifier: MPL-2.0 OR GPL-2.0-or-later

use cosmic::Element;
use cosmic::widget::{button, dialog, icon};
use derive_more::From;

use crate::gui::Page;
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
pub struct ErrorPage {
    error: Error,
}

impl Page for ErrorPage {
    type Message = Msg;

    fn view(&self) -> Element<'_, Msg> {
        dialog()
            .icon(icon(crate::gui::icon::error()).size(30))
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
