// SPDX-License-Identifier: MPL-2.0 OR GPL-2.0-or-later

use std::path::{Path, PathBuf};

use expand_tilde::ExpandTilde;
use fs_err as fs;

use crate::model::Result;

pub async fn read_dir_with_filter<P>(path: P, filter: fn(&Path) -> bool) -> Result<Vec<PathBuf>>
where
    P: AsRef<Path>,
{
    let path = path.as_ref().expand_tilde()?;
    let mut read_dir = fs::tokio::read_dir(path).await?;
    let mut paths = Vec::new();
    while let Some(entry) = read_dir.next_entry().await? {
        let path = entry.path();
        if filter(&path) {
            paths.push(path);
        }
    }
    Ok(paths)
}

pub async fn read_dir(path: impl AsRef<Path>) -> Result<Vec<PathBuf>> {
    read_dir_with_filter(path, |_| true).await
}
