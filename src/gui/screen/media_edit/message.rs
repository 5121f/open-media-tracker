/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::path::{Path, PathBuf};

use crate::gui::screen::{ConfirmScrnMsg, WarningMsg};

#[derive(Debug, Clone)]
pub enum Msg {
    Back,
    Delete(usize),
    Watch { path: PathBuf },
    NameChanged(String),
    ChapterChanged(String),
    EpisodeChanged(String),
    ChapterPathChanged(String),
    ChapterPathSelect,
    OpenChapterDirectory,
    ChapterInc,
    ChapterDec,
    EpisodeInc,
    EpisodeDec,
    ConfirmScreen(ConfirmScrnMsg),
    Warning(WarningMsg),
}

impl Msg {
    pub fn watch(path: impl AsRef<Path>) -> Self {
        Self::Watch {
            path: path.as_ref().to_owned(),
        }
    }
}

impl From<ConfirmScrnMsg> for Msg {
    fn from(value: ConfirmScrnMsg) -> Self {
        Self::ConfirmScreen(value)
    }
}

impl From<WarningMsg> for Msg {
    fn from(value: WarningMsg) -> Self {
        Self::Warning(value)
    }
}
