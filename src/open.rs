/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::path::{Path, PathBuf};

pub fn open(path: impl AsRef<Path>) -> Result<(), OpenError> {
    let path = path.as_ref();
    open::that(path).map_err(|source| OpenError::new(path, source.kind()))
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
