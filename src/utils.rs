use std::{
    fs,
    path::{Path, PathBuf},
};

use mime_guess::mime;

use crate::error::ErrorKind;

pub fn open(path: impl AsRef<Path>) -> Result<(), ErrorKind> {
    let path = path.as_ref();
    open::that(path).map_err(|source| ErrorKind::open(&path, source.kind()))
}

pub fn read_dir(path: impl AsRef<Path>) -> Result<Vec<PathBuf>, ErrorKind> {
    let read_dir = fs::read_dir(&path).map_err(|source| ErrorKind::fsio(&path, source))?;
    let mut files = Vec::new();
    for entry in read_dir {
        let entry = entry.map_err(|source| ErrorKind::fsio(&path, source))?;
        files.push(entry.path());
    }
    Ok(files)
}

pub fn next_dir(path: impl AsRef<Path>) -> Result<PathBuf, ErrorKind> {
    let path = path.as_ref();
    let dir_name = path
        .file_name()
        .ok_or(ErrorKind::FailedToFindNextSeasonPath)?;
    let parent = path
        .parent()
        .ok_or(ErrorKind::FailedToFindNextSeasonPath)?
        .to_owned();
    let mut paths = read_dir(parent)?;
    paths.retain(|path| path.is_dir());
    paths.sort();
    let (current_dir_index, _) = paths
        .iter()
        .flat_map(|path| path.file_name())
        .flat_map(|name| name.to_str())
        .enumerate()
        .find(|(_, file_name)| *file_name == dir_name)
        .ok_or(ErrorKind::FailedToFindNextSeasonPath)?;
    let next_season_index = current_dir_index + 1;
    if next_season_index >= paths.len() {
        return Err(ErrorKind::FailedToFindNextSeasonPath);
    }
    let next_dir = paths[next_season_index].to_path_buf();
    Ok(next_dir)
}

pub fn is_media_file(path: impl AsRef<Path>) -> bool {
    let mime = mime_guess::from_path(path);
    let Some(mime) = mime.first() else {
        return false;
    };
    let mtype = mime.type_();
    mtype == mime::VIDEO || mtype == mime::AUDIO
}

pub fn episode_paths(series_path: impl AsRef<Path>) -> Result<Option<Vec<PathBuf>>, ErrorKind> {
    let series_path = series_path.as_ref();
    if !series_path.exists() {
        return Ok(None);
    }
    let mut episode_paths = read_dir(series_path)?;
    episode_paths.retain(|p| is_media_file(p));
    episode_paths.sort();
    Ok(Some(episode_paths))
}
