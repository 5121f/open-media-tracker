/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::path::{Path, PathBuf};

use fs_err as fs;

use crate::model::Result;

pub fn read_dir_with_filter(
    path: impl AsRef<Path>,
    filter: fn(&Path) -> bool,
) -> Result<Vec<PathBuf>> {
    let path = path.as_ref();

    let read_dir = fs::read_dir(path)?;
    let mut paths = Vec::new();
    for entry in read_dir {
        let entry = entry?;
        let path = entry.path();
        if filter(&path) {
            paths.push(path);
        }
    }
    Ok(paths)
}

pub fn read_dir(path: impl AsRef<Path>) -> Result<Vec<PathBuf>> {
    read_dir_with_filter(path, |_| true)
}
