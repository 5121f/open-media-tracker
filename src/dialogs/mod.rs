pub mod error_dialog;
pub mod main_window;
pub mod serial_edit_dialog;

use std::rc::Rc;

use crate::{Error, Message, Serial};
use error_dialog::ErrorDialog;
use iced::Element;
use main_window::MainWindow;
use serial_edit_dialog::SerialEditDialog;

pub enum Dialog {
    MainWindow(MainWindow),
    SerialChange(SerialEditDialog),
    Error(ErrorDialog),
}

impl Dialog {
    pub fn main_window(media: Vec<Rc<Serial>>) -> Self {
        let dialog = MainWindow::new(media);
        Self::MainWindow(dialog)
    }

    pub fn add_serial() -> Result<Self, Error> {
        let dialog = SerialEditDialog::new()?;
        Ok(Self::SerialChange(dialog))
    }

    pub fn change_serial(serial: &Serial, id: usize) -> Self {
        let dialog = SerialEditDialog::change(serial, id);
        Self::SerialChange(dialog)
    }

    pub fn error(message: impl ToString) -> Self {
        let dialog = ErrorDialog::new(message);
        Self::Error(dialog)
    }
}

impl Dialog {
    pub fn view(&self) -> Element<Message> {
        match self {
            Dialog::MainWindow(dialog) => dialog.view().map(Message::MainWindow),
            Dialog::SerialChange(dialog) => dialog.view().map(Message::SerialChange),
            Dialog::Error(dialog) => dialog.view().map(Message::ErrorDialog),
        }
    }
}
