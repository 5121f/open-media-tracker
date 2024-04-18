mod dialogs;

use std::{fs, num::NonZeroU8, path::PathBuf, rc::Rc};

use iced::{Element, Sandbox, Settings, Theme};
use ron::ser::PrettyConfig;
use serde::{Deserialize, Serialize};

use crate::dialogs::{error_dialog, main_window, serial_edit_dialog, Dialog};

fn main() -> iced::Result {
    ZCinema::run(Settings::default())
}

#[derive(Debug, Clone)]
enum Message {
    MainWindow(main_window::Message),
    SerialChange(serial_edit_dialog::Message),
    ErrorDialog(error_dialog::Message),
}

struct ZCinema {
    media: Vec<Rc<Serial>>,
    dialog: Dialog,
    state_dir: PathBuf,
}

impl ZCinema {
    fn add_serial_dialog(&mut self) -> Result<(), Error> {
        self.dialog = Dialog::add_serial()?;
        Ok(())
    }

    fn change_serial_dialog(&mut self, id: usize) {
        let serial = &self.media[id];
        self.dialog = Dialog::change_serial(serial, id)
    }

    fn main_window(&mut self) {
        let media = clone_rc_vec(&self.media);
        self.dialog = Dialog::main_window(media);
    }

    fn error_dialog(&mut self, message: impl ToString) {
        self.dialog = Dialog::error(message.to_string());
    }

    fn handle_error<T, E>(&mut self, result: Result<T, E>) -> Option<T>
    where
        E: std::error::Error,
    {
        match result {
            Ok(value) => Some(value),
            Err(err) => {
                self.error_dialog(err);
                None
            }
        }
    }

    fn save_serial(&self, id: usize) {
        let serial = self.media[id].as_ref();
        let content = ron::ser::to_string_pretty(serial, PrettyConfig::new()).unwrap();
        if !self.state_dir.exists() {
            fs::create_dir(&self.state_dir).unwrap();
        }
        let file_name = serial_file_name(&serial.name);
        let path = self.state_dir.join(&file_name);
        fs::write(path, content).unwrap();
    }

    fn remove_serial(&mut self, id: usize) {
        let serial = self.media[id].as_ref();
        let file_name = serial_file_name(&serial.name);
        let path = self.state_dir.join(file_name);
        fs::remove_file(path).unwrap();
        self.media.remove(id);
    }
}

impl Sandbox for ZCinema {
    type Message = Message;

    fn new() -> Self {
        let state_dir = dirs::state_dir().unwrap().join("zcinema");
        let mut media = Vec::new();
        for entry in fs::read_dir(&state_dir).unwrap() {
            let entry = entry.unwrap().path();
            if entry.is_file() {
                let file_content = fs::read_to_string(entry).unwrap();
                let m: Serial = ron::from_str(&file_content).unwrap();
                media.push(Rc::new(m));
            }
        }
        let media2 = clone_rc_vec(&media);
        Self {
            media,
            dialog: Dialog::main_window(media2),
            state_dir,
        }
    }

    fn title(&self) -> String {
        String::from("ZCinema")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::MainWindow(main_window::Message::AddSerial) => {
                let res = self.add_serial_dialog();
                self.handle_error(res);
            }
            Message::MainWindow(main_window::Message::ChangeSerial(id)) => {
                self.change_serial_dialog(id)
            }
            Message::SerialChange(serial_edit_dialog::Message::Accept {
                kind,
                name,
                season,
                seria,
            }) => {
                if let serial_edit_dialog::Kind::Change { id } = kind {
                    let serial = Rc::get_mut(&mut self.media[id]).unwrap();
                    if serial.name != name {
                        let file_name = serial_file_name(&serial.name);
                        let path = self.state_dir.join(&file_name);
                        let new_name = serial_file_name(&name);
                        let new_path = self.state_dir.join(&new_name);
                        fs::rename(path, new_path).unwrap();
                        serial.name = name;
                    }
                    serial.current_season = season;
                    serial.current_seria = seria;
                    self.save_serial(id);
                } else {
                    let serial = Rc::new(Serial {
                        name,
                        current_season: season,
                        current_seria: seria,
                    });
                    self.media.push(serial);
                    self.save_serial(self.media.len() - 1);
                }
                self.main_window();
            }
            Message::SerialChange(serial_edit_dialog::Message::Delete(id)) => {
                self.remove_serial(id);
                self.main_window();
            }
            Message::SerialChange(serial_edit_dialog::Message::Back) => {
                self.main_window();
            }
            Message::SerialChange(dialog_message) => {
                if let Dialog::SerialChange(dialog) = &mut self.dialog {
                    let res = dialog.update(dialog_message);
                    self.handle_error(res);
                }
            }
            Message::ErrorDialog(error_dialog::Message::Ok) => self.main_window(),
        }
    }

    fn view(&self) -> Element<Message> {
        self.dialog.view()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

#[derive(Serialize, Deserialize)]
struct Serial {
    name: String,
    current_season: NonZeroU8,
    current_seria: NonZeroU8,
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("Season and seria number can not be zero")]
    SeasonAndSeriaCannotBeZero,
    #[error("Number overflow")]
    NumberOverflow,
}

fn clone_rc_vec<T>(v: &[Rc<T>]) -> Vec<Rc<T>> {
    v.iter().map(|m| Rc::clone(&m)).collect()
}

fn serial_file_name(name: &str) -> String {
    format!("{}.ron", name)
}
