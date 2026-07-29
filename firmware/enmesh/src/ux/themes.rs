// provide the shared crates via re-export
use common::*;

// provide the embedded graphics primitives
use embedded_graphics::pixelcolor::Rgb888;

pub struct Theme<'a> {
    /// default color for text and mono-icons
    pub color: Rgb888,
    pub background: Rgb888,
    /// regular text font
    pub text_style: embedded_graphics::mono_font::MonoTextStyle<'a, Rgb888>,
    /// label font
    pub label_style: embedded_graphics::mono_font::MonoTextStyle<'a, Rgb888>,
    /// large text font
    pub h1_style: embedded_graphics::mono_font::MonoTextStyle<'a, Rgb888>,
}
impl<'a> Theme<'a> {
    /// create a theme to match the display area
    pub fn new(screen_area: embedded_graphics::geometry::Size) -> Theme<'a> {
        use embedded_graphics::prelude::*;
        use embedded_graphics::mono_font::{MonoTextStyle, ascii::*};

        // default theme WHITE text on BLACK background
        let color = embedded_graphics::pixelcolor::Rgb888::WHITE;
        let background = embedded_graphics::pixelcolor::Rgb888::BLACK;

        // choose font based on display size
        return if screen_area.height <= 64 {
            // use low height font
            Theme {
                color,
                background,
                text_style: MonoTextStyle::new(&FONT_6X9, color),
                label_style: MonoTextStyle::new(&FONT_6X10, color),
                h1_style: MonoTextStyle::new(&FONT_10X20, color),
            }
        }
        else {
            // use the largest font with BOLD support
            Theme {
                color,
                background,
                // FIXME this font isn't very large
                text_style: MonoTextStyle::new(&FONT_9X18, color),
                label_style: MonoTextStyle::new(&FONT_9X18_BOLD, color),
                h1_style: MonoTextStyle::new(&FONT_10X20, color),
            }
        };
}

}

