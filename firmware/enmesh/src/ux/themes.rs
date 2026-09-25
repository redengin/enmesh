// provide the shared crates via re-export
use common::{embedded_graphics::pixelcolor::RgbColor, *};

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
    /// small text font
    pub small_style: embedded_graphics::mono_font::MonoTextStyle<'a, Rgb888>,
}
impl<'a> Theme<'a> {
    /// create a theme to match the display area
    pub fn new(screen_area: embedded_graphics::geometry::Size) -> Theme<'a> {
        use embedded_graphics::mono_font::MonoTextStyle;

        // default theme WHITE text on BLACK background
        let color = Rgb888::WHITE;
        let background = Rgb888::BLACK;

        // choose font based on display size
        return if screen_area.height <= 64 {
            Theme {
                color,
                background,
                text_style: MonoTextStyle::new(&embedded_graphics::mono_font::ascii::FONT_8X13_BOLD, color),
                label_style: MonoTextStyle::new(&embedded_graphics::mono_font::ascii::FONT_8X13, color),
                h1_style: MonoTextStyle::new(&profont::PROFONT_14_POINT, color),
                small_style: MonoTextStyle::new(&embedded_graphics::mono_font::ascii::FONT_5X8, color),
            }
        }
        else {
            Theme {
                color,
                background,
                text_style: MonoTextStyle::new(&embedded_graphics::mono_font::ascii::FONT_9X18_BOLD, color),
                label_style: MonoTextStyle::new(&embedded_graphics::mono_font::ascii::FONT_9X18, color),
                h1_style: MonoTextStyle::new(&profont::PROFONT_24_POINT, color),
                small_style: MonoTextStyle::new(&profont::PROFONT_9_POINT, color),
            }
        };
}

}

