/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::model::{FSIOError, FSIOErrorExtention};

fn _read_dir(path: &Path) -> Result<Vec<PathBuf>> {
    let read_dir = fs::read_dir(path).fs_err(path)?;
    let mut paths = Vec::new();
    for entry in read_dir {
        let entry = entry.fs_err(path)?;
        paths.push(entry.path());
    }
    Ok(paths)
}
pub fn read_dir(path: impl AsRef<Path>) -> Result<Vec<PathBuf>> {
    _read_dir(path.as_ref())
}

fn _read_dir_for_dirs(path: &Path) -> Result<Vec<PathBuf>> {
    let read_dir = fs::read_dir(path).fs_err(path)?;
    let mut dirs = Vec::new();
    for entry in read_dir {
        let entry = entry.fs_err(path)?;
        let path = entry.path();
        if path.is_dir() {
            dirs.push(path);
        }
    }
    Ok(dirs)
}
pub fn read_dir_for_dirs(path: impl AsRef<Path>) -> Result<Vec<PathBuf>> {
    _read_dir_for_dirs(path.as_ref())
}

type Result<T> = std::result::Result<T, FSIOError>;
