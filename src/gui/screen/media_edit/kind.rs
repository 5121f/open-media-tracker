/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::{
    fmt::{self, Display},
    path::PathBuf,
};

#[derive(Clone)]
pub enum ConfirmKind {
    SwitchToNextChapter { path: PathBuf },
    EpisodesOverflow { episodes_on_disk: usize },
}

impl ConfirmKind {
    pub fn switch_to_next_chapter(path: PathBuf) -> Self {
        Self::SwitchToNextChapter { path }
    }

    pub fn episode_overflow(episodes_on_disk: usize) -> Self {
        Self::EpisodesOverflow { episodes_on_disk }
    }
}

impl Display for ConfirmKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::SwitchToNextChapter {
                path: next_chapter_path,
            } => {
                write!(f, "Proposed path to next chapter: {:?}", next_chapter_path)
            }
            Self::EpisodesOverflow { episodes_on_disk } => write!(
                f,
                "Seems like {} episode is a last of it chapter. Switch to the next chapter?",
                episodes_on_disk
            ),
        }
    }
}

pub enum WarningKind {
    NameUsed,
    WrongChapterPath,
}

impl Display for WarningKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NameUsed => write!(f, "Name must be unique"),
            Self::WrongChapterPath => write!(f, "Wrong chapter path"),
        }
    }
}
