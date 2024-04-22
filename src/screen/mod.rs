pub mod confirm;
pub mod error;
pub mod main;
pub mod serial_edit;

use std::{cell::RefCell, rc::Rc};

pub use confirm::{ConfirmScreen, Message as ConfirmScreenMessage};
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

    pub fn main(media: &[Rc<RefCell<Serial>>]) -> Self {
        let media = media.into_iter().map(Rc::clone).collect();
        let dialog = MainScreen::new(media);
        Self::MainWindow(dialog)
    }

    pub fn change_serial(serial: Rc<RefCell<Serial>>, id: usize) -> Self {
        let dialog = SerialEditScreen::new(serial, id);
        Self::SerialChange(dialog)
    }
}

impl Default for Dialog {
    fn default() -> Self {
        Dialog::MainWindow(MainScreen::default())
    }
}

// Optional Dialog
#[derive(Default)]
pub struct Od<T>(Option<T>);

impl<T> Od<T> {
    pub fn new(dialog: T) -> Self {
        Self(Some(dialog))
    }

    pub fn closed() -> Self {
        Self(None)
    }

    pub fn close(&mut self) {
        self.0 = None;
    }

    pub fn get(&self) -> Option<&T> {
        self.0.as_ref()
    }
}
