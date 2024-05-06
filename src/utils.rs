use std::{
    fs,
    path::{Path, PathBuf},
    rc::Rc,
};

use mime_guess::mime;

use crate::{config::Config, error::ErrorKind, series::Series};

pub fn read_media(config: Rc<Config>) -> Result<Vec<Series>, ErrorKind> {
    let media = read_dir(&config.data_dir)?
        .into_iter()
        .map(|m| Series::read_from_file(m, config.clone()))
        .collect::<Result<_, _>>()?;
    Ok(media)
}

pub fn watch(path: impl AsRef<Path>) -> Result<(), ErrorKind> {
    let path = path.as_ref();
    open::that(path).map_err(|source| ErrorKind::open_vido(&path, source.kind()))?;
    Ok(())
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

pub fn read_dir_sort(path: impl AsRef<Path>) -> Result<Vec<PathBuf>, ErrorKind> {
    let mut read_dir = read_dir(path)?;
    read_dir.sort();
    Ok(read_dir)
}

pub fn arr_rc_clone<T>(vec: &[Rc<T>]) -> Vec<Rc<T>> {
    vec.iter().map(Rc::clone).collect()
}

pub fn next_dir(path: impl AsRef<Path>) -> Result<PathBuf, ErrorKind> {
    let path = path.as_ref();
    let dir_name = path
        .file_name()
        .ok_or(ErrorKind::FailedToFindNextSeasonPath)?;
    let parent = path
        .parent()
        .ok_or(ErrorKind::parent_dir(&dir_name))?
        .to_owned();
    let paths = read_dir_sort(parent)?;
    let dirs: Vec<_> = paths.into_iter().filter(|path| path.is_dir()).collect();
    let mut current_dir_index = None;
    for (i, dir) in dirs.iter().enumerate() {
        let dir = dir
            .file_name()
            .ok_or(ErrorKind::FailedToFindNextSeasonPath)?
            .to_str()
            .ok_or(ErrorKind::FailedToFindNextSeasonPath)?;
        if dir_name == dir {
            current_dir_index = Some(i);
            break;
        }
    }
    let current_dir_index = current_dir_index.ok_or(ErrorKind::FailedToFindNextSeasonPath)?;
    let next_season_index = current_dir_index + 1;
    if next_season_index >= dirs.len() {
        return Err(ErrorKind::FailedToFindNextSeasonPath);
    }
    let next_dir = dirs[next_season_index].to_path_buf();
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
