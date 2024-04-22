pub mod confirm;
pub mod error;
pub mod main;
pub mod serial_edit;

pub use confirm::{ConfirmScreen, Message as ConfirmScreenMessage};
pub use error::{ErrorScreen, Message as ErrorScreenMessage};
pub use main::{MainScreen, Message as MainScreenMessage};
pub use serial_edit::{Message as SerialEditScreenMessage, SerialEditScreen};

// Optional Dialog
#[derive(Default)]
pub struct Dialog<T>(Option<T>);

impl<T> Dialog<T> {
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
