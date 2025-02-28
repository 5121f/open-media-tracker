/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::path::PathBuf;

use etcetera::BaseStrategy;
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
        let user_dirs =
            etcetera::choose_base_strategy().map_err(|_| ErrorKind::UserDataDirNotFound)?;
        let user_data_dir = user_dirs.data_dir();
        let data_dir = user_data_dir.join(DATA_DIR_NAME);
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
            data_dir: etcetera::choose_base_strategy()
                .map(|d| d.data_dir().join(DATA_DIR_NAME))
                .unwrap_or_default(),
        }
    }
}
