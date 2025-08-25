/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::path::{Path, PathBuf};

use expand_tilde::ExpandTilde;

use crate::model::{ErrorKind, Result};
use crate::utils;

pub async fn next_dir(path: impl AsRef<Path>) -> Result<PathBuf> {
    let path = path.as_ref().expand_tilde()?;

    let parent = path
        .parent()
        .ok_or_else(|| ErrorKind::find_parent(&*path))?;
    let mut paths = utils::read_dir_with_filter(parent, Path::is_dir).await?;
    let dir_name = path.file_name().unwrap_or_default();
    paths.sort();
    let (current_dir_index, _) = paths
        .iter()
        .filter_map(|path| path.file_name())
        .enumerate()
        .find(|(_, file_name)| *file_name == dir_name)
        .ok_or_else(|| ErrorKind::find_next_chapter(&*path))?;
    let next_chapter_index = current_dir_index + 1;
    if next_chapter_index >= paths.len() {
        return Err(ErrorKind::find_next_chapter(path));
    }
    let next_dir = paths.swap_remove(next_chapter_index);
    Ok(next_dir)
}
