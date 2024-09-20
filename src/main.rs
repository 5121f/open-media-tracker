/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

#![windows_subsystem = "windows"] // Do not open console window on startup on Windows

mod gui;
mod message;
mod model;
mod utils;

use gui::main::OpenMediaTracker;

fn main() -> iced::Result {
    iced::application(
        OpenMediaTracker::title,
        OpenMediaTracker::update,
        OpenMediaTracker::view,
    )
    .run_with(OpenMediaTracker::new)
}
