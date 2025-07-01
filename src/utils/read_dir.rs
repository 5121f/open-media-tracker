/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::io;
use std::path::{Path, PathBuf};

use fs_err as fs;

pub async fn read_dir_with_filter<P>(path: P, filter: fn(&Path) -> bool) -> io::Result<Vec<PathBuf>>
where
    P: AsRef<Path>,
{
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

pub async fn read_dir(path: impl AsRef<Path>) -> io::Result<Vec<PathBuf>> {
    read_dir_with_filter(path, |_| true).await
}
