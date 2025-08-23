// SPDX-License-Identifier: MPL-2.0 OR GPL-2.0-or-later
//
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
