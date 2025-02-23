/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::path::PathBuf;

use derive_more::derive::From;

use crate::gui::screen::{ConfirmScrnMsg, WarningMsg};

#[derive(Debug, Clone, From)]
pub enum Msg {
    Back,
    Delete(usize),
    Watch {
        path: PathBuf,
    },
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
    #[from]
    ConfirmScreen(ConfirmScrnMsg),
    #[from]
    Warning(WarningMsg),
}

impl Msg {
    pub fn watch(path: impl Into<PathBuf>) -> Self {
        Self::Watch { path: path.into() }
    }
}
