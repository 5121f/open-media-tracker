pub mod error_dialog;
pub mod main_window;
pub mod serial_edit_dialog;

use crate::{serial::viewmodel::Serial, Error, Message};
use iced::Element;
use main_window::MainWindow;
use serial_edit_dialog::SerialEditDialog;

pub enum Dialog {
    MainWindow(MainWindow),
    SerialChange(SerialEditDialog),
}

impl Dialog {
    pub fn main_window(media: &[Serial]) -> Self {
        let media = media.into_iter().map(|m| m.rc_clone()).collect();
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
}

impl Dialog {
    pub fn view(&self) -> Element<Message> {
        match self {
            Dialog::MainWindow(dialog) => dialog.view().map(Message::MainWindow),
            Dialog::SerialChange(dialog) => dialog.view().map(Message::SerialChange),
        }
    }
}
