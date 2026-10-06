// provide the Page primitives
use super::pages::prelude::*;

pub struct Theme<'a> {
    /// default color for text and mono-icons
    pub color: Rgb888,
    pub background: Rgb888,
    /// regular text font
    pub text_style: MonoTextStyle<'a, Rgb888>,
    /// label font
    pub label_style: MonoTextStyle<'a, Rgb888>,
    /// large text font
    pub h1_style: MonoTextStyle<'a, Rgb888>,
    /// small text font
    pub small_style: MonoTextStyle<'a, Rgb888>,

    // image assets
    pub enmesh_logo: tinyqoi::Qoi<'static>,
}

/// provide theme creator using display size and Color
pub trait ThemeForColor {
    fn theme(screen_size: Size) -> Theme<'static>;
}


/// provide monochrome support
impl ThemeForColor for BinaryColor {
    fn theme(screen_size: Size) -> Theme<'static>
    {
        // default theme WHITE text on BLACK background
        let color = Rgb888::WHITE;
        let background = Rgb888::BLACK;

        // choose font based on display size
        return if screen_size.height <= 64 {
            Theme {
                color,
                background,
                text_style: MonoTextStyle::new(&mono_font::ascii::FONT_6X10, color),
                label_style: MonoTextStyle::new(&mono_font::ascii::FONT_6X10, color),
                h1_style: MonoTextStyle::new(&profont::PROFONT_14_POINT, color),
                small_style: MonoTextStyle::new(&mono_font::ascii::FONT_5X8, color),
                enmesh_logo: generated::images::enmesh_logo_20x20(),
            }
        } else {
            Theme {
                color,
                background,
                text_style: MonoTextStyle::new(&mono_font::ascii::FONT_9X18_BOLD, color),
                label_style: MonoTextStyle::new(&mono_font::ascii::FONT_9X18, color),
                h1_style: MonoTextStyle::new(&profont::PROFONT_24_POINT, color),
                small_style: MonoTextStyle::new(&profont::PROFONT_9_POINT, color),
                enmesh_logo: generated::images::enmesh_logo_36x36(),
            }
        };
    }
}

/// provide monochrome support
impl ThemeForColor for Rgb888 {
    fn theme(screen_size: Size) -> Theme<'static>
    {
        // default theme WHITE text on BLACK background
        let color = Rgb888::WHITE;
        let background = Rgb888::BLACK;

        // choose font based on display size
        return if screen_size.height <= 64 {
            Theme {
                color,
                background,
                text_style: MonoTextStyle::new(&mono_font::ascii::FONT_8X13_BOLD, color),
                label_style: MonoTextStyle::new(&mono_font::ascii::FONT_8X13, color),
                h1_style: MonoTextStyle::new(&profont::PROFONT_14_POINT, color),
                small_style: MonoTextStyle::new(&mono_font::ascii::FONT_5X8, color),
                enmesh_logo: generated::images::enmesh_logo_20x20(),
            }
        } else {
            Theme {
                color,
                background,
                text_style: MonoTextStyle::new(&mono_font::ascii::FONT_9X18_BOLD, color),
                label_style: MonoTextStyle::new(&mono_font::ascii::FONT_9X18, color),
                h1_style: MonoTextStyle::new(&profont::PROFONT_24_POINT, color),
                small_style: MonoTextStyle::new(&profont::PROFONT_9_POINT, color),
                enmesh_logo: generated::images::enmesh_logo_36x36(),
            }
        };
    }
}
