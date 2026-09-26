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
}

/// provide monochrome support
pub mod binary_color {
    // provide the Page primitives
    use super::super::pages::prelude::*;

    pub fn new<'a>(screen_size: Size) -> super::Theme<'a>
    {
        // default theme WHITE text on BLACK background
        let color = Rgb888::WHITE;
        let background = Rgb888::BLACK;

        // choose font based on display size
        return if screen_size.height <= 64 {
            super::Theme {
                color,
                background,
                text_style: MonoTextStyle::new(&mono_font::ascii::FONT_8X13_BOLD, color),
                label_style: MonoTextStyle::new(&mono_font::ascii::FONT_8X13, color),
                h1_style: MonoTextStyle::new(&profont::PROFONT_14_POINT, color),
                small_style: MonoTextStyle::new(&mono_font::ascii::FONT_5X8, color),
            }
        } else {
            super::Theme {
                color,
                background,
                text_style: MonoTextStyle::new(&mono_font::ascii::FONT_9X18_BOLD, color),
                label_style: MonoTextStyle::new(&mono_font::ascii::FONT_9X18, color),
                h1_style: MonoTextStyle::new(&profont::PROFONT_24_POINT, color),
                small_style: MonoTextStyle::new(&profont::PROFONT_9_POINT, color),
            }
        };
    }
}

/// provide full color support
pub mod color {
    // provide the Page primitives
    use super::super::pages::prelude::*;

    pub fn new<'a>(screen_size: Size) -> super::Theme<'a>
    {
        // default theme WHITE text on BLACK background
        let color = Rgb888::WHITE;
        let background = Rgb888::BLACK;

        // choose font based on display size
        return if screen_size.height <= 64 {
            super::Theme {
                color,
                background,
                text_style: MonoTextStyle::new(&mono_font::ascii::FONT_8X13_BOLD, color),
                label_style: MonoTextStyle::new(&mono_font::ascii::FONT_8X13, color),
                h1_style: MonoTextStyle::new(&profont::PROFONT_14_POINT, color),
                small_style: MonoTextStyle::new(&mono_font::ascii::FONT_5X8, color),
            }
        } else {
            super::Theme {
                color,
                background,
                text_style: MonoTextStyle::new(&mono_font::ascii::FONT_9X18_BOLD, color),
                label_style: MonoTextStyle::new(&mono_font::ascii::FONT_9X18, color),
                h1_style: MonoTextStyle::new(&profont::PROFONT_24_POINT, color),
                small_style: MonoTextStyle::new(&profont::PROFONT_9_POINT, color),
            }
        };
    }
}

