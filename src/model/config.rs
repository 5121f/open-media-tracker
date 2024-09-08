/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::{fs, path::PathBuf};

use crate::model::error::{ErrorKind, FSIOError, Result};

#[derive(Debug, Default)]
pub struct Config {
    pub data_dir: PathBuf,
}

impl Config {
    pub fn read() -> Result<Self> {
        let data_dir = dirs::data_dir()
            .ok_or(ErrorKind::UserDataDirNotFound)?
            .join("open media tracker");
        if !data_dir.exists() {
            fs::create_dir(&data_dir).map_err(|source| FSIOError::new(&data_dir, source))?;
        }
        Ok(Self { data_dir })
    }
}
