/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::path::PathBuf;
use std::sync::Arc;

use cosmic::dialog::file_chooser;
use derive_more::From;
use url::Url;

use crate::gui::page::{ConfirmPageMsg, WarningPageMsg};
use crate::model::{Episode, Result};

#[derive(Debug, Clone, From)]
pub enum Msg {
    Back,
    Delete(usize),
    Watch {
        path: PathBuf,
    },
    NameChanged(String),
    ChapterChanged(u8),
    EpisodeChanged(u8),
    ChapterPathChanged(String),
    ChapterPathSelect,
    ChapterPathSelected(Url),
    OpenChapterDirectory,
    #[from]
    ConfirmScreen(ConfirmPageMsg),
    #[from]
    Warning(WarningPageMsg),
    OpenDialogCanceled,
    OpenDialogError(Arc<file_chooser::Error>),
    NextChapterPath(Result<PathBuf>),
    EpisodeListLoaded(Result<Arc<Vec<Episode>>>),
    CheckOverflow {
        episode_list_read_res: Result<Arc<Vec<Episode>>>,
        if_not_then: Box<Msg>,
    },
}

impl Msg {
    pub fn watch(path: impl Into<PathBuf>) -> Self {
        Self::Watch { path: path.into() }
    }
}
