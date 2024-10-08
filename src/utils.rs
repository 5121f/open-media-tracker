/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::{
    fs,
    path::{Path, PathBuf},
};

use mime_guess::mime;

use crate::model::FSIOError;

pub fn open(path: impl AsRef<Path>) -> Result<(), OpenError> {
    let path = path.as_ref();
    open::that(path).map_err(|source| OpenError::new(path, source.kind()))
}

pub fn read_dir(path: impl AsRef<Path>) -> Result<Vec<PathBuf>, FSIOError> {
    let read_dir = fs::read_dir(&path).map_err(|source| FSIOError::new(&path, source))?;
    let mut files = Vec::new();
    for entry in read_dir {
        let entry = entry.map_err(|source| FSIOError::new(&path, source))?;
        files.push(entry.path());
    }
    Ok(files)
}

pub fn is_media_file(path: impl AsRef<Path>) -> bool {
    let mime = mime_guess::from_path(path);
    let Some(mime) = mime.first() else {
        return false;
    };
    let mtype = mime.type_();
    mtype == mime::VIDEO || mtype == mime::AUDIO
}

#[derive(Debug, Clone, thiserror::Error)]
#[error("{path}: Failed to open default program: {kind}")]
pub struct OpenError {
    path: PathBuf,
    kind: std::io::ErrorKind,
}

impl OpenError {
    fn new(path: impl AsRef<Path>, kind: std::io::ErrorKind) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
            kind,
        }
    }
}
