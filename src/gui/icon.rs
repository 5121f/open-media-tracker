/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use cosmic::widget::icon;

#[cfg(all(unix, not(feature = "embed_icons")))]
macro_rules! cosmic_icon {
    ($name:expr) => {
        icon::from_name($name).handle()
    };
}

#[cfg(any(not(unix), feature = "embed_icons"))]
#[macro_use]
mod embed_icons {
    macro_rules! icon_path {
        ($path:expr) => {
            include_bytes!(concat!("../../assets/icons/", $path))
        };
    }

    macro_rules! cosmic_icon_path {
        ($path:expr) => {
            icon_path!(concat!("cosmic/", $path))
        };
    }

    macro_rules! svg_name {
        ($name:expr) => {
            concat!($name, ".svg")
        };
    }

    macro_rules! cosmic_icon {
        ($name:expr) => {
            icon::from_svg_bytes(cosmic_icon_path!(svg_name!($name))).symbolic(true)
        };
    }
}

pub fn sort_descending() -> icon::Handle {
    cosmic_icon!("view-sort-descending-symbolic")
}

pub fn sort_ascending() -> icon::Handle {
    cosmic_icon!("view-sort-ascending-symbolic")
}

pub fn search() -> icon::Handle {
    cosmic_icon!("system-search-symbolic")
}

pub fn back() -> icon::Handle {
    cosmic_icon!("go-previous-symbolic")
}

pub fn folder() -> icon::Handle {
    cosmic_icon!("folder-symbolic")
}

pub fn close() -> icon::Handle {
    cosmic_icon!("window-close-symbolic")
}

pub fn warning() -> icon::Handle {
    cosmic_icon!("dialog-warning-symbolic")
}

pub fn error() -> icon::Handle {
    cosmic_icon!("dialog-error-symbolic")
}
