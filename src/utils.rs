use std::{
    fs,
    path::{Path, PathBuf},
};

use iced::{
    alignment, theme,
    widget::{button, text, Button},
    Color,
};

use crate::{
    error::{Error, ErrorKind},
    serial::viewmodel::Serial,
};

pub const DEFAULT_INDENT: u16 = 5;

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

pub fn square_button<M>(s: &str) -> Button<M> {
    button(
        text(s)
            .horizontal_alignment(alignment::Horizontal::Center)
            .line_height(1.0)
            .size(20),
    )
    .height(30)
    .width(30)
}

pub fn link<M>(s: &str) -> Button<M> {
    const CYAN: Color = Color::from_rgb(0.0, 255.0, 255.0);
    button(text(s).style(theme::Text::Color(CYAN))).style(theme::Button::Text)
}
