/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::path::PathBuf;

use derive_more::Display;

#[derive(Clone, Display)]
pub enum ConfirmKind {
    #[display("Proposed path to next chapter: {path:?}")]
    SwitchToNextChapter { path: PathBuf },
    #[display(
        "Seems like {episodes_on_disk} episode is a last of it chapter. \
        Switch to the next chapter?"
    )]
    EpisodesOverflow { episodes_on_disk: usize },
}

impl ConfirmKind {
    pub fn switch_to_next_chapter(path: impl Into<PathBuf>) -> Self {
        let path = path.into();
        Self::SwitchToNextChapter { path }
    }

    pub const fn episode_overflow(episodes_on_disk: usize) -> Self {
        Self::EpisodesOverflow { episodes_on_disk }
    }
}

#[derive(Display)]
pub enum WarningKind {
    #[display("Name must be unique")]
    NameUsed,
    #[display("Wrong chapter path")]
    WrongChapterPath,
}
