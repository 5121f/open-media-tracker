/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

#![windows_subsystem = "windows"] // Do not open console window on startup on Windows
#![warn(clippy::pedantic)]

mod gui;
mod model;
mod utils;

use cosmic::{app::Settings, iced::Size};

use gui::app::OpenMediaTracker;

fn main() -> cosmic::iced::Result {
    cosmic::app::run::<OpenMediaTracker>(Settings::default().size(Size::new(600.0, 500.0)), ())
}
