/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::{fs, path::PathBuf};

use crate::model::error::{ErrorKind, FSIOError, Result};

const DATA_DIR_NAME: &str = "open media tracker";

#[derive(Debug)]
pub struct Config {
    pub data_dir: PathBuf,
}

impl Config {
    pub fn read() -> Result<Self> {
        let data_dir = dirs::data_dir()
            .ok_or(ErrorKind::UserDataDirNotFound)?
            .join(DATA_DIR_NAME);
        if !data_dir.exists() {
            fs::create_dir(&data_dir).map_err(|source| FSIOError::new(&data_dir, source))?;
        }
        Ok(Self { data_dir })
    }

    pub fn path_to_media(&self, file_name: &str) -> PathBuf {
        self.data_dir.join(file_name)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            data_dir: dirs::data_dir()
                .map(|d| d.join(DATA_DIR_NAME))
                .unwrap_or_default(),
        }
    }
}
