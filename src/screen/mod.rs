pub mod confirm;
pub mod error;
pub mod main;
pub mod serial_edit;

use std::rc::Rc;

pub use error::{ErrorScreen, Message as ErrorScreenMessage};
pub use main::{MainScreen, Message as MainScreenMessage};
pub use serial_edit::{Message as SerialEditScreenMessage, SerialEditScreen};

use iced::Element;

use crate::{serial::Serial, Message};

pub enum Dialog {
    MainWindow(MainScreen),
    SerialChange(SerialEditScreen),
}

impl Dialog {
    pub fn view(&self) -> Element<Message> {
        match self {
            Dialog::MainWindow(dialog) => dialog.view().map(Message::MainScreen),
            Dialog::SerialChange(dialog) => dialog.view().map(Message::SerialEditScreen),
        }
    }

    pub fn main(media: &[Rc<Serial>]) -> Self {
        let media = media.into_iter().map(Rc::clone).collect();
        let dialog = MainScreen::new(media);
        Self::MainWindow(dialog)
    }

    pub fn add_serial() -> Self {
        let dialog = SerialEditScreen::new();
        Self::SerialChange(dialog)
    }

    pub fn change_serial(serial: &Serial, id: usize) -> Self {
        let dialog = SerialEditScreen::change(serial, id);
        Self::SerialChange(dialog)
    }
}

impl Default for Dialog {
    fn default() -> Self {
        Dialog::MainWindow(MainScreen::default())
    }
}
