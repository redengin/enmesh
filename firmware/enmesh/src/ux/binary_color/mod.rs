/// provide the shared crates via re-export
use common::{embedded_graphics::text::renderer::TextRenderer, *};

/// provide logging primitives
use log::*;
const TAG: &str = "[UX BinaryColor]";

/// provide enmesh primitives
use crate::prelude::*;

mod themes;
use embedded_graphics::pixelcolor::BinaryColor;

/// UX thread
pub async fn run(
    _global_state: &'static RwLock<NoopRawMutex, crate::State>,
    mut display: impl crate::ux::BufferedDisplay<Color = BinaryColor>,
    _button: impl button::ButtonState,
    led: impl led::LedState,
) {
    // create the status LED
    let _status_led = crate::ux::status_led::StatusLed::new(led);

    // create the UX theme
    let theme = themes::Theme::new(display.bounding_box().size);

    trace!("{TAG} powering on display....");
    // FIXME TEST-USE-ONLY
    display.power_on().await;
    loop {
        use embedded_graphics::prelude::*;
        use embedded_graphics::text::Text;
        use embedded_graphics::pixelcolor::BinaryColor;
        let _ = display.clear(theme.background);

        let mut anchor = theme.h1_style.line_height() as i32;
        let _ = Text::new("Header Text", Point::new(0, anchor), theme.h1_style).draw(&mut display);
        anchor += theme.label_style.line_height() as i32;
        let _ = Text::new("Label Text", Point::new(0, anchor), theme.label_style).draw(&mut display);
        anchor += theme.text_style.line_height() as i32;
        let _ = Text::new("Normal Text", Point::new(0, anchor), theme.text_style).draw(&mut display);

        let _ = display.flush().await;

        Timer::after_secs(1).await;
    }
    // let mut ux = Ux::new();
}

