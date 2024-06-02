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
