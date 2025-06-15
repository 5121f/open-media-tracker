/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

#![windows_subsystem = "windows"] // Do not open console window on startup on Windows
#![warn(clippy::pedantic)]

mod gui;
mod message;
mod model;
mod open;
mod read_dir;

use cosmic::app::Settings;

use gui::main::OpenMediaTracker;
use open::open;
use read_dir::read_dir;

fn main() -> cosmic::iced::Result {
    cosmic::app::run::<OpenMediaTracker>(Settings::default(), ())
}
