use iced::{
    alignment, theme,
    widget::{button, row, text, text_input, Button, Row},
    Alignment, Color,
};

pub const DEFAULT_INDENT: u16 = 5;

pub fn square_button<M>(s: &str) -> Button<M> {
    button(
        text(s)
            .horizontal_alignment(alignment::Horizontal::Center)
            .line_height(1.0)
            .size(20),
    )
    .height(30)
    .width(30)
}

pub fn link<M>(s: &str) -> Button<M> {
    const CYAN: Color = Color::from_rgb(0.0, 255.0, 255.0);
    button(text(s).style(theme::Text::Color(CYAN))).style(theme::Button::Text)
}

pub fn signed_text_imput<'a, M, F>(s: &str, value: &str, on_input: F) -> Row<'a, M>
where
    M: Clone + 'a,
    F: 'a + Fn(String) -> M,
{
    row![text(s), text_input(s, value).on_input(on_input)]
        .spacing(DEFAULT_INDENT)
        .align_items(Alignment::Center)
}
