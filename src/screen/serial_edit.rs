use std::{
    cell::RefCell,
    fmt::Display,
    num::NonZeroU8,
    path::{Path, PathBuf},
    rc::Rc,
};

use iced::{
    theme,
    widget::{button, column, horizontal_space, row, text, Column, Space},
    Element, Length,
};
use iced_aw::card;

use crate::{
    dialog::Dialog,
    error::{Error, ErrorKind},
    screen::{ConfirmScreen, ConfirmScreenMessage},
    serial::Serial,
    utils::{self, read_dir},
    view_utils::{link, signed_text_imput, square_button, DEFAULT_INDENT},
};

#[derive(Debug, Clone)]
pub enum Message {
    Back,
    Delete(usize),
    Watch { path: String, seria: usize },
    NameChanged(String),
    SeasonChanged(String),
    SeriaChanged(String),
    SeasonPathChanged(String),
    SeasonPathSelect,
    SeasonInc,
    SeasonDec,
    SeriaInc,
    SeriaDec,
    ConfirmScreen(ConfirmScreenMessage),
    WarningClose,
}

pub struct SerialEditScreen {
    serials: Vec<Rc<RefCell<Serial>>>,
    confirm_screen: Dialog<ConfirmScreen<ConfirmKind>>,
    warning: Dialog<WarningKind>,
    seies_on_disk: Option<usize>,
    editable_serial_id: usize,
    buffer_name: String,
}

impl SerialEditScreen {
    pub fn new(serials: Vec<Rc<RefCell<Serial>>>, editable_serial_id: usize) -> Self {
        let editable_serial_name = {
            let editable_serial = serials[editable_serial_id].borrow();
            editable_serial.name().to_string()
        };
        Self {
            confirm_screen: Dialog::closed(),
            seies_on_disk: None,
            serials,
            editable_serial_id,
            warning: Dialog::closed(),
            buffer_name: editable_serial_name,
        }
    }

    pub fn view(&self) -> Element<Message> {
        let confirm_screen = self.confirm_screen.view_into();
        if let Some(confirm_screen) = confirm_screen {
            return confirm_screen;
        }
        let serial = self.editable_serial().borrow();
        let season_path = serial.season_path().display().to_string();
        let top = row![
            link("< Back").on_press(Message::Back),
            horizontal_space(),
            text(serial.name()),
            horizontal_space(),
            button("Delete")
                .style(theme::Button::Destructive)
                .on_press(Message::Delete(self.editable_serial_id)),
        ];
        let body = column![
            signed_text_imput("Name", &self.buffer_name, Message::NameChanged),
            row![
                signed_text_imput(
                    "Season",
                    &serial.season().to_string(),
                    Message::SeasonChanged
                ),
                square_button("-").on_press(Message::SeasonDec),
                square_button("+").on_press(Message::SeasonInc)
            ]
            .spacing(DEFAULT_INDENT),
            row![
                signed_text_imput("Seria", &serial.seria().to_string(), Message::SeriaChanged),
                square_button("-").on_press(Message::SeriaDec),
                square_button("+").on_press(Message::SeriaInc)
            ]
            .spacing(DEFAULT_INDENT),
            row![
                signed_text_imput("Season path", &season_path, Message::SeasonPathChanged),
                square_button("...").on_press(Message::SeasonPathSelect),
                square_button(">").on_press(Message::Watch {
                    path: season_path,
                    seria: serial.seria().get() as usize - 1
                })
            ]
            .spacing(DEFAULT_INDENT)
        ]
        .spacing(DEFAULT_INDENT);
        let space = Space::with_height(Length::Fixed(15.0));
        let mut layout = Column::new()
            .padding(DEFAULT_INDENT)
            .spacing(DEFAULT_INDENT);

        layout = layout.push(top);
        layout = layout.push(space);
        layout = layout.push_maybe(self.warning.as_ref().map(|w| w.view()));
        layout = layout.push(body);

        layout.into()
    }

    pub fn update(&mut self, message: Message) -> Result<(), Error> {
        match message {
            Message::Back | Message::Delete(_) | Message::Watch { .. } => {}
            Message::NameChanged(value) => {
                self.buffer_name = value.clone();
                let name_used = self.serials.iter().any(|s| s.borrow().name() == &value);
                if name_used {
                    self.warning(WarningKind::NameUsed);
                    return Ok(());
                }
                if let Some(WarningKind::NameUsed) = self.warning.as_ref() {
                    self.warning.close();
                }
                let serial = self.editable_serial();
                serial.borrow_mut().rename(value)?;
            }
            Message::SeasonChanged(value) => match value.parse() {
                Ok(number) => self.editable_serial().borrow_mut().set_season(number)?,
                Err(_) => self.warning(WarningKind::ParseNum),
            },
            Message::SeriaChanged(value) => match value.parse() {
                Ok(number) => self.set_seria(number)?,
                Err(_) => self.warning(WarningKind::ParseNum),
            },
            Message::SeasonInc => self.increase_season()?,
            Message::SeasonDec => {
                let serial = self.editable_serial();
                let new_value = serial.borrow().season().get() - 1;
                let new_value = NonZeroU8::new(new_value);
                match new_value {
                    Some(number) => serial.borrow_mut().set_season(number)?,
                    None => self.warning(WarningKind::SeasonCanNotBeZero),
                }
            }
            Message::SeriaInc => self.increase_seria()?,
            Message::SeriaDec => {
                let serial = self.editable_serial();
                let new_value = serial.borrow().seria().get() - 1;
                let new_value = NonZeroU8::new(new_value);
                match new_value {
                    Some(number) => self.editable_serial().borrow_mut().set_seria(number)?,
                    None => self.warning(WarningKind::SeriaCanNotBeZero),
                }
            }
            Message::SeasonPathChanged(value) => self
                .editable_serial()
                .borrow_mut()
                .set_season_path(PathBuf::from(value))?,
            Message::ConfirmScreen(message) => match message {
                ConfirmScreenMessage::Confirm => {
                    let Some(confirm) = self.confirm_screen.take() else {
                        return Ok(());
                    };
                    match confirm.take() {
                        ConfirmKind::TrySwitchToNewSeason { season_path } => {
                            self.editable_serial()
                                .borrow_mut()
                                .set_season_path(season_path)?;
                            self.confirm_screen.close();
                        }
                        ConfirmKind::SeriaOverflow { .. } => self.increase_season()?,
                    }
                }
                ConfirmScreenMessage::Cancel => self.confirm_screen.close(),
            },
            Message::SeasonPathSelect => {
                if let Some(folder) = rfd::FileDialog::new().pick_folder() {
                    self.editable_serial()
                        .borrow_mut()
                        .set_season_path(folder)?;
                }
            }
            Message::WarningClose => self.warning.close(),
        }
        Ok(())
    }

    pub fn title(&self) -> String {
        self.editable_serial().borrow().name().to_string()
    }

    fn editable_serial(&self) -> &Rc<RefCell<Serial>> {
        &self.serials[self.editable_serial_id]
    }

    fn warning(&mut self, kind: WarningKind) {
        self.warning = Dialog::new(kind);
    }

    fn set_series_on_disk(&mut self, series: usize) {
        self.seies_on_disk = Some(series);
    }

    fn increase_seria(&mut self) -> Result<(), Error> {
        let serial = self.editable_serial();
        let next_seria = serial.borrow().seria().saturating_add(1);
        self.set_seria(next_seria)
    }

    fn set_seria(&mut self, value: NonZeroU8) -> Result<(), Error> {
        let serial = self.editable_serial();
        if !serial.borrow().season_path_is_present() || value <= serial.borrow().seria() {
            serial.borrow_mut().set_seria(value)?;
            return Ok(());
        }
        let seies_on_disk = match self.seies_on_disk {
            Some(seies_on_disk) => seies_on_disk,
            None => self.read_series_on_disk()?,
        };
        if seies_on_disk < value.get() as usize {
            self.confirm(ConfirmKind::SeriaOverflow { seies_on_disk });
            return Ok(());
        }
        let serial = self.editable_serial();
        serial.borrow_mut().set_seria(value)?;
        Ok(())
    }

    fn read_series_on_disk(&mut self) -> Result<usize, ErrorKind> {
        let serial = self.editable_serial();
        let series_on_disk = read_dir(&serial.borrow().season_path())?.len();
        self.set_series_on_disk(series_on_disk);
        Ok(series_on_disk)
    }

    fn set_seria_to_one(&mut self) -> Result<(), ErrorKind> {
        let serial = self.editable_serial();
        serial.borrow_mut().set_seria(NonZeroU8::MIN)
    }

    fn increase_season(&mut self) -> Result<(), ErrorKind> {
        self.set_seria_to_one()?;
        let serial = self.editable_serial();
        let next_season = serial.borrow().season().saturating_add(1);
        serial.borrow_mut().set_season(next_season)?;
        if serial.borrow().season_path_is_present() {
            let season_path = next_dir(&serial.borrow().season_path())?
                .ok_or(ErrorKind::FailedToFindNextSeasonPath)?;
            self.confirm(ConfirmKind::TrySwitchToNewSeason { season_path });
        }
        Ok(())
    }

    fn confirm(&mut self, kind: ConfirmKind) {
        let confirm = ConfirmScreen::new(kind);
        self.confirm_screen = Dialog::new(confirm);
    }
}

fn next_dir(path: impl AsRef<Path>) -> Result<Option<PathBuf>, ErrorKind> {
    let path = path.as_ref();
    let dir_name = path
        .file_name()
        .ok_or(ErrorKind::FailedToFindNextSeasonPath)?;
    let parent = path
        .parent()
        .ok_or(ErrorKind::parent_dir(&dir_name))?
        .to_owned();
    let paths = utils::read_dir_sort(parent)?;
    let dirs: Vec<_> = paths.into_iter().filter(|path| path.is_dir()).collect();
    let mut season_dir_index = None;
    for (i, dir) in dirs.iter().enumerate() {
        let dir = dir
            .file_name()
            .ok_or(ErrorKind::FailedToFindNextSeasonPath)?
            .to_str()
            .ok_or(ErrorKind::FailedToFindNextSeasonPath)?;
        if dir_name == dir {
            season_dir_index = Some(i);
        }
    }
    let Some(season_dir_index) = season_dir_index else {
        return Ok(None);
    };
    let next_season_index = season_dir_index + 1;
    if next_season_index >= dirs.len() {
        return Ok(None);
    }
    Ok(Some(dirs[next_season_index].to_path_buf()))
}

enum ConfirmKind {
    TrySwitchToNewSeason { season_path: PathBuf },
    SeriaOverflow { seies_on_disk: usize },
}

impl Display for ConfirmKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfirmKind::TrySwitchToNewSeason { season_path } => {
                write!(f, "Proposed path to next season: {}", season_path.display())
            }
            ConfirmKind::SeriaOverflow { seies_on_disk } => write!(
                f,
                "Seems like {} serias is a last of it season. Switch to the next season?",
                seies_on_disk
            ),
        }
    }
}

enum WarningKind {
    SeasonCanNotBeZero,
    SeriaCanNotBeZero,
    NameUsed,
    ParseNum,
}

impl WarningKind {
    fn view(&self) -> Element<Message> {
        card("Warning", text(self.to_string()))
            .close_size(25.)
            .style(iced_aw::style::CardStyles::Warning)
            .on_close(Message::WarningClose)
            .into()
    }
}

impl Display for WarningKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WarningKind::SeasonCanNotBeZero => write!(f, "Season can not be zero"),
            WarningKind::SeriaCanNotBeZero => write!(f, "Seria can not be zero"),
            WarningKind::NameUsed => write!(f, "Name must be unic"),
            WarningKind::ParseNum => write!(f, "Failed to parse number. Maybe number is too big."),
        }
    }
}

impl From<ConfirmScreenMessage> for Message {
    fn from(value: ConfirmScreenMessage) -> Self {
        Self::ConfirmScreen(value)
    }
}
