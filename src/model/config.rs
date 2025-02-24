/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::path::PathBuf;

use fs_err as fs;

use crate::model::error::{ErrorKind, Result};

use super::Placeholder;

const DATA_DIR_NAME: &str = "open_media_tracker";

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
            fs::create_dir(&data_dir)?;
        }
        Ok(Self { data_dir })
    }

    pub fn path_to_media(&self, file_name: &str) -> PathBuf {
        self.data_dir.join(file_name)
    }
}

impl Placeholder for Config {
    fn placeholder() -> Self {
        Self {
            data_dir: dirs::data_dir()
                .map(|d| d.join(DATA_DIR_NAME))
                .unwrap_or_default(),
        }
    }
}
