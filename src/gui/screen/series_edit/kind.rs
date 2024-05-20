use std::{
    fmt::{self, Display},
    path::PathBuf,
};

#[derive(Clone)]
pub enum ConfirmKind {
    SwitchToNextSeason { next_season_path: PathBuf },
    EpisodesOverflow { series_on_disk: usize },
}

impl ConfirmKind {
    pub fn switch_to_next_season(next_season_path: PathBuf) -> Self {
        Self::SwitchToNextSeason { next_season_path }
    }

    pub fn episode_overflow(series_on_disk: usize) -> Self {
        Self::EpisodesOverflow { series_on_disk }
    }
}

impl Display for ConfirmKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConfirmKind::SwitchToNextSeason { next_season_path } => {
                write!(f, "Proposed path to next season: {:?}", next_season_path)
            }
            ConfirmKind::EpisodesOverflow { series_on_disk } => write!(
                f,
                "Seems like {} episode is a last of it season. Switch to the next season?",
                series_on_disk
            ),
        }
    }
}

pub enum WarningKind {
    SeasonCanNotBeZero,
    EpisodeCanNotBeZero,
    NameUsed,
    WrongSeasonPath,
}

impl Display for WarningKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WarningKind::SeasonCanNotBeZero => write!(f, "Season can not be zero"),
            WarningKind::EpisodeCanNotBeZero => write!(f, "Episode can not be zero"),
            WarningKind::NameUsed => write!(f, "Name must be unique"),
            WarningKind::WrongSeasonPath => write!(f, "Wrong season path"),
        }
    }
}
