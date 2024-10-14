/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::path::{Path, PathBuf};

use mime_guess::mime;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Episode {
    path: PathBuf,
}

impl Episode {
    pub fn new(path: PathBuf) -> Result<Self> {
        if !is_media_file(&path) {
            return Err(Error::MustBeAMediaFile);
        }
        Ok(Self { path })
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn name(&self) -> String {
        self.path
            .file_stem()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string()
    }
}

fn is_media_file(path: impl AsRef<Path>) -> bool {
    let mime = mime_guess::from_path(path);
    let Some(mime) = mime.first() else {
        return false;
    };
    let mtype = mime.type_();
    mtype == mime::VIDEO || mtype == mime::AUDIO
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum Error {
    #[error("Episode must be media file")]
    MustBeAMediaFile,
}

type Result<T> = std::result::Result<T, Error>;
