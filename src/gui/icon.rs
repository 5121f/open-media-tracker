use iced::{
    alignment,
    widget::{Button, Text},
    Font,
};

const ICON_FONT: Font = Font::with_name("open_media_tracker");

pub struct Icon(char);

impl Icon {
    pub fn triple_dot() -> Self {
        Self('\u{E800}')
    }

    pub fn open_folder() -> Self {
        Self('\u{F115}')
    }
}

impl AsRef<char> for Icon {
    fn as_ref(&self) -> &char {
        &self.0
    }
}

pub fn text<'a, Message>(icon: Icon) -> Text<'a, Message>
where
    Message: iced::widget::text::Catalog + 'a,
{
    Text::new(icon.as_ref()).font(ICON_FONT)
}

pub fn button<'a, Message>(icon: Icon) -> Button<'a, Message> {
    Button::new(text(icon).align_x(alignment::Horizontal::Center)).width(30)
}
