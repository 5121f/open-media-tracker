use iced::{
    theme,
    widget::{button, column, container, horizontal_space, row, text, Space},
    Element, Length,
};
use iced_aw::card;

use crate::{error::Error, gui::utils::DEFAULT_INDENT};

#[derive(Debug, Clone)]
pub enum Message {
    Ok { critical: bool },
}

pub fn error_view<'a>(error: &Error) -> Element<'a, Message> {
    let ok_button_style = if error.critical {
        theme::Button::Destructive
    } else {
        theme::Button::Primary
    };

    container(row![
        Space::with_width(Length::FillPortion(1)),
        card(
            text(error_title()),
            column![
                text(error),
                row![
                    horizontal_space(),
                    button("Ok").style(ok_button_style).on_press(Message::Ok {
                        critical: error.critical
                    })
                ],
            ]
            .spacing(DEFAULT_INDENT)
        )
        .style(iced_aw::style::card::CardStyles::Danger)
        .width(Length::FillPortion(15)),
        Space::with_width(Length::FillPortion(1))
    ])
    .center_y()
    .into()
}

pub fn error_title() -> String {
    String::from("Error")
}
