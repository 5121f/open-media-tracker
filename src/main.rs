/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

mod gui;
mod model;
mod utils;

use cosmic::app::Settings;
use cosmic::iced::Size;

use gui::app::OpenMediaTracker;
use log::LevelFilter;

fn main() -> cosmic::iced::Result {
    env_logger::builder().filter_level(LevelFilter::Warn).init();

    cosmic::app::run::<OpenMediaTracker>(Settings::default().size(Size::new(600.0, 500.0)), ())
}
