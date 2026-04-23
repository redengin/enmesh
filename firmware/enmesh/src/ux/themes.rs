use crate::ux::*;

/// Creates a default theme for the UX/Pages
#[allow(non_snake_case)]
pub fn DefaultTheme<'a>(size: Size) -> Theme<'a> {
    use embedded_graphics::mono_font::{MonoTextStyle, ascii::*};

    // default theme WHITE on BLACK background
    let color = embedded_graphics::pixelcolor::Rgb888::WHITE;
    let background = embedded_graphics::pixelcolor::Rgb888::BLACK;

    // choose font based on display size
    let text_style = if size.height > 64 {
        MonoTextStyle::new(&FONT_6X10, color)
    } else {
        MonoTextStyle::new(&FONT_8X13, color)
    };

    Theme {
        text_style,
        color: color,
        background,
    }
}
