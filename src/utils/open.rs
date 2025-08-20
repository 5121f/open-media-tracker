/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::ffi::OsStr;
use std::path::PathBuf;
use std::sync::Arc;

pub fn open(path: impl AsRef<OsStr>) -> Result<(), OpenError> {
    let path = path.as_ref();
    open::that_detached(path).map_err(|source| OpenError::new(source, path))
}

#[derive(Debug, Clone, thiserror::Error)]
#[error("{path}: Failed to open default program: {source}")]
pub struct OpenError {
    path: PathBuf,
    source: Arc<std::io::Error>,
}

impl OpenError {
    fn new(source: std::io::Error, path: impl Into<PathBuf>) -> Self {
        let source = source.into();
        let path = path.into();
        Self { path, source }
    }
}
