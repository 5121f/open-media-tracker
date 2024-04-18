use std::num::NonZeroU8;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Serial {
    pub name: String,
    pub current_season: NonZeroU8,
    pub current_seria: NonZeroU8,
}
