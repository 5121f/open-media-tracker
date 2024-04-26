use std::{
    fs,
    path::{Path, PathBuf},
    rc::Rc,
};

use crate::{
    config::Config,
    error::{Error, ErrorKind},
    series::Series,
};

pub fn read_media(config: Rc<Config>) -> Result<Vec<Series>, ErrorKind> {
    let media = read_dir(&config.data_dir)?
        .into_iter()
        .map(|m| Series::read_from_file(m, config.clone()))
        .collect::<Result<_, _>>()?;
    Ok(media)
}

pub fn watch(path: impl AsRef<Path>, episode_number: usize) -> Result<(), Error> {
    let files = read_dir(path)?;
    let episode = &files[episode_number];
    open::that(episode).map_err(|source| ErrorKind::open_vido(&episode, source.kind()))?;
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
