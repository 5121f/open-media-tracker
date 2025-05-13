/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::path::PathBuf;

use derive_more::{Deref, From};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default, Deref, From)]
pub struct UserPath(String);

impl UserPath {
    pub fn into_path_buf(self) -> PathBuf {
        if self.0.starts_with('~') {
            let Ok(home_dir) = etcetera::home_dir() else {
                return PathBuf::from(self.0);
            };
            let relative_path = if self.0.starts_with("~/") {
                &self.0[2..]
            } else {
                &self.0[1..]
            };
            dbg!(&relative_path);
            return home_dir.join(relative_path);
        }
        PathBuf::from(self.0)
    }
}

impl From<PathBuf> for UserPath {
    fn from(value: PathBuf) -> Self {
        Self(value.to_string_lossy().to_string())
    }
}

impl AsRef<str> for UserPath {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
