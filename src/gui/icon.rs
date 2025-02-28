use iced::{
    Alignment, Font,
    widget::{Button, Text},
};

const ICON_FONT: Font = Font::with_name("open_media_tracker");

pub struct Icon(char);

impl Icon {
    pub const fn triple_dot() -> Self {
        Self('\u{E800}')
    }

    pub const fn open_folder() -> Self {
        Self('\u{F115}')
    }

    pub fn text<'a, Message>(&self) -> Text<'a, Message>
    where
        Message: iced::widget::text::Catalog + 'a,
    {
        Text::new(self.0).font(ICON_FONT)
    }

    pub fn button<'a, Message>(&self) -> Button<'a, Message> {
        Button::new(self.text().align_x(Alignment::Center)).width(30)
    }
}
