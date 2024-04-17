use std::{
    borrow::{Borrow, BorrowMut},
    rc::Rc,
};

use iced::{Element, Sandbox, Settings, Theme};

use crate::{main_window::MainWindow, serial_chamge_dialog::SerialChangeDialog};

fn main() -> iced::Result {
    ZCinema::run(Settings::default())
}

#[derive(Debug, Clone)]
enum Message {
    MainWindow(main_window::Message),
    SerialChange(serial_chamge_dialog::Message),
}

struct ZCinema {
    media: Vec<Rc<Serial>>,
    dialog: Dialog,
}

impl ZCinema {
    fn clone_media(&self) -> Vec<Rc<Serial>> {
        self.media.iter().map(|m| Rc::clone(&m)).collect()
    }
}

impl Sandbox for ZCinema {
    type Message = Message;

    fn new() -> Self {
        Self {
            media: Vec::new(),
            dialog: Dialog::main_window(Vec::new()),
        }
    }

    fn title(&self) -> String {
        String::from("ZCinema")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::MainWindow(main_window::Message::AddSerial) => {
                self.dialog = Dialog::add_serial();
            }
            Message::MainWindow(main_window::Message::ChangeSerial(id)) => {
                let serial = self.media[id].borrow();
                self.dialog = Dialog::change_serial(serial, id);
            }
            Message::SerialChange(serial_chamge_dialog::Message::Accept) => {
                if let Dialog::SerialChange(dialog) = self.dialog.borrow_mut() {
                    if let Some(id) = dialog.id {
                        let serial = Rc::get_mut(&mut self.media[id]).unwrap();
                        serial.name = dialog.name.clone();
                    } else {
                        let serial = Serial::new(dialog.name.clone());
                        self.media.push(Rc::new(serial));
                    }
                    self.dialog = Dialog::main_window(self.clone_media());
                }
            }
            Message::SerialChange(serial_chamge_dialog::Message::Back) => {
                self.dialog = Dialog::main_window(self.clone_media());
            }
            Message::SerialChange(dialog_message) => {
                if let Dialog::SerialChange(dialog) = self.dialog.borrow_mut() {
                    dialog.update(dialog_message);
                }
            }
        }
    }

    fn view(&self) -> Element<Message> {
        self.dialog.view()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

enum Dialog {
    MainWindow(MainWindow),
    SerialChange(SerialChangeDialog),
}

impl Dialog {
    fn main_window(media: Vec<Rc<Serial>>) -> Self {
        let dialog = MainWindow::new(media);
        Self::MainWindow(dialog)
    }

    fn add_serial() -> Self {
        let dialog = SerialChangeDialog::new();
        Self::SerialChange(dialog)
    }

    fn change_serial(serial: &Serial, id: usize) -> Self {
        let dialog = SerialChangeDialog::change(serial, id);
        Self::SerialChange(dialog)
    }
}

impl Dialog {
    fn view(&self) -> Element<Message> {
        match self {
            Dialog::MainWindow(dialog) => dialog.view().map(Message::MainWindow),
            Dialog::SerialChange(dialog) => dialog.view().map(Message::SerialChange),
        }
    }
}

mod main_window {
    use std::rc::Rc;

    use iced::{
        widget::{button, column, horizontal_space, row, text},
        Element,
    };

    use crate::Serial;

    #[derive(Debug, Clone)]
    pub enum Message {
        AddSerial,
        ChangeSerial(usize),
    }

    pub struct MainWindow {
        media: Vec<Rc<Serial>>,
    }

    impl MainWindow {
        pub fn new(media: Vec<Rc<Serial>>) -> Self {
            Self { media }
        }

        pub fn view(&self) -> Element<Message> {
            column![
                button("+").on_press(Message::AddSerial),
                column(
                    self.media
                        .iter()
                        .enumerate()
                        .map(|(id, m)| row![
                            text(&m.name),
                            horizontal_space(),
                            button("...").on_press(Message::ChangeSerial(id))
                        ]
                        .into())
                        .collect::<Vec<_>>()
                )
            ]
            .into()
        }
    }
}

mod serial_chamge_dialog {
    use iced::{
        widget::{button, column, horizontal_space, row, text, text_input},
        Element,
    };

    use crate::Serial;

    #[derive(Debug, Clone)]
    pub enum Message {
        Back,
        Accept,
        NameChanged(String),
    }

    pub struct SerialChangeDialog {
        pub id: Option<usize>,
        pub name: String,
    }

    impl SerialChangeDialog {
        pub fn new() -> Self {
            Self {
                id: None,
                name: String::new(),
            }
        }

        pub fn change(serial: &Serial, id: usize) -> Self {
            let name = serial.name.clone();
            Self { id: Some(id), name }
        }

        pub fn view(&self) -> Element<Message> {
            column![
                button("< Back").on_press(Message::Back),
                row![
                    text("Name"),
                    text_input("Name", &self.name).on_input(Message::NameChanged)
                ],
                row![
                    horizontal_space(),
                    button("Accept").on_press(Message::Accept)
                ]
            ]
            .into()
        }

        pub fn update(&mut self, message: Message) {
            match message {
                Message::Back | Message::Accept => {}
                Message::NameChanged(value) => self.name = value,
            }
        }
    }
}

struct Serial {
    name: String,
}

impl Serial {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
