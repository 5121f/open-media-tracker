/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::path::PathBuf;

use crate::gui::{screen::ConfirmScreenMessage, WarningMessage};

#[derive(Debug, Clone)]
pub enum Message {
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
    ConfirmScreen(ConfirmScreenMessage),
    Warning(WarningMessage),
}

impl From<ConfirmScreenMessage> for Message {
    fn from(value: ConfirmScreenMessage) -> Self {
        Self::ConfirmScreen(value)
    }
}

impl From<WarningMessage> for Message {
    fn from(value: WarningMessage) -> Self {
        Self::Warning(value)
    }
}
