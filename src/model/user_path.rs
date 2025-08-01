/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::path::PathBuf;

use derive_more::{Deref, From};
use serde::{Deserialize, Serialize};

#[cfg(unix)]
const HOME_PREFIX: &str = "~/";

#[cfg(windows)]
const HOME_PREFIX: &str = "~\\";

#[derive(Debug, Clone, Serialize, Deserialize, Default, Deref, From)]
pub struct UserPath(String);

impl UserPath {
    pub fn userify(path: impl Into<PathBuf>) -> Self {
        let path = path.into().to_string_lossy().to_string();
        let Some(home_dir) = std::env::home_dir() else {
            return Self(path);
        };
        let home_dir = home_dir.to_string_lossy().to_string();
        if let Some(value) = path.strip_prefix(&home_dir) {
            return Self(format!("~{value}"));
        }
        Self(path)
    }

    pub fn into_path_buf(self) -> PathBuf {
        if self.0.starts_with(HOME_PREFIX) {
            let Ok(home_dir) = etcetera::home_dir() else {
                return PathBuf::from(self.0);
            };
            let relative_path = &self.0[2..];
            return home_dir.join(relative_path);
        }
        PathBuf::from(self.0)
    }

    pub fn to_path_buf(&self) -> PathBuf {
        self.clone().into_path_buf()
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
