use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::{
    error::{Error, ErrorKind},
    serial::Serial,
};

pub fn read_media(dir: impl AsRef<Path>) -> Result<Vec<Serial>, ErrorKind> {
    let media = read_dir(dir)?
        .into_iter()
        .map(Serial::read_from_file)
        .collect::<Result<_, _>>()?;
    Ok(media)
}

pub fn watch(path: impl AsRef<Path>, seria_number: usize) -> Result<(), Error> {
    let files = read_dir(path)?;
    let seria = &files[seria_number];
    open::that(seria).map_err(|source| ErrorKind::open_vido(&seria, source.kind()))?;
    Ok(())
}

pub fn read_dir(path: impl AsRef<Path>) -> Result<Vec<PathBuf>, ErrorKind> {
    let read_dir = fs::read_dir(&path).map_err(|source| ErrorKind::fsio(&path, source))?;
    let mut files = Vec::new();
    for entry in read_dir {
        let entry = entry.map_err(|source| ErrorKind::fsio(&path, source))?;
        files.push(entry.path());
    }
    files.sort();
    Ok(files)
}
