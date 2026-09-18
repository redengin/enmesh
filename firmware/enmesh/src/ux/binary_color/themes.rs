// provide the shared crates via re-export
use common::*;

// provide the embedded graphics primitives
use embedded_graphics::pixelcolor::BinaryColor;

pub struct Theme<'a> {
    /// default color for text and mono-icons
    pub color: BinaryColor,
    pub background: BinaryColor,
    /// regular text font
    pub text_style: embedded_graphics::mono_font::MonoTextStyle<'a, BinaryColor>,
    /// label font
    pub label_style: embedded_graphics::mono_font::MonoTextStyle<'a, BinaryColor>,
    /// large text font
    pub h1_style: embedded_graphics::mono_font::MonoTextStyle<'a, BinaryColor>,
}
impl<'a> Theme<'a> {
    /// create a theme to match the display area
    pub fn new(screen_area: embedded_graphics::geometry::Size) -> Theme<'a> {
        use embedded_graphics::mono_font::MonoTextStyle;

        // default theme BLACK text on WHITE background
        let color = embedded_graphics::pixelcolor::BinaryColor::Off;
        let background = embedded_graphics::pixelcolor::BinaryColor::On;

        // choose font based on display size
        return if screen_area.height <= 64 {
            Theme {
                color,
                background,
                text_style: MonoTextStyle::new(&embedded_graphics::mono_font::ascii::FONT_8X13_BOLD, color),
                label_style: MonoTextStyle::new(&embedded_graphics::mono_font::ascii::FONT_8X13, color),
                h1_style: MonoTextStyle::new(&profont::PROFONT_14_POINT, color),
            }
        }
        else {
            Theme {
                color,
                background,
                text_style: MonoTextStyle::new(&embedded_graphics::mono_font::ascii::FONT_9X18_BOLD, color),
                label_style: MonoTextStyle::new(&embedded_graphics::mono_font::ascii::FONT_9X18, color),
                h1_style: MonoTextStyle::new(&profont::PROFONT_24_POINT, color),
            }
        };
}

}

