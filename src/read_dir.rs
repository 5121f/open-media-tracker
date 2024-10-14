/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::model::FSIOError;

pub fn read_dir(path: impl AsRef<Path>) -> Result<Vec<PathBuf>, FSIOError> {
    let read_dir = fs::read_dir(&path).map_err(|source| FSIOError::new(&path, source))?;
    let mut files = Vec::new();
    for entry in read_dir {
        let entry = entry.map_err(|source| FSIOError::new(&path, source))?;
        files.push(entry.path());
    }
    Ok(files)
}

pub fn read_dir_for_dirs(path: impl AsRef<Path>) -> Result<Vec<PathBuf>, FSIOError> {
    let read_dir = fs::read_dir(&path).map_err(|source| FSIOError::new(&path, source))?;
    let mut dirs = Vec::new();
    for entry in read_dir {
        let entry = entry.map_err(|source| FSIOError::new(&path, source))?;
        let path = entry.path();
        if path.is_dir() {
            dirs.push(path);
        }
    }
    Ok(dirs)
}