/// provide the shared crates via re-export
use common::*;

/// provide logging primitives
use log::*;
const TAG: &str = "[UX BinaryColor]";

/// provide enmesh primitives
use crate::prelude::*;

pub mod themes;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::text::renderer::TextRenderer;

/// UX thread
pub async fn run(
    _global_state: &'static RwLock<NoopRawMutex, crate::State>,
    mut display: impl crate::ux::BufferedDisplay<Color = BinaryColor>,
    button: impl button::ButtonState,
    led: impl led::LedState,
) {
    // create the status LED
    let _status_led = crate::ux::status_led::StatusLed::new(led);

    // create the button monitor
    let mut button_monitor = crate::ux::ButtonMonitor::new(button);

    // create the UX theme
    let theme = themes::Theme::new(display.bounding_box().size);

    trace!("{TAG} powering on display....");
    display.power_on().await;

    // FIXME test-use-only
    let mut has_hid_event = false;

    loop {
        use embedded_graphics::prelude::*;
        use embedded_graphics::text::Text;
        let _ = display.clear(theme.background);

        let mut anchor = theme.h1_style.line_height() as i32;
        let _ = Text::new("Header Text", Point::new(0, anchor), theme.h1_style).draw(&mut display);
        anchor += theme.label_style.line_height() as i32;
        let _ = Text::new("Label Text", Point::new(0, anchor), theme.label_style).draw(&mut display);
        anchor += theme.text_style.line_height() as i32;
        let _ = Text::new("Normal Text", Point::new(0, anchor), theme.text_style).draw(&mut display);
        if has_hid_event {
            anchor += theme.text_style.line_height() as i32;
            let _ = Text::new("Button PRESSED", Point::new(0, anchor), theme.text_style).draw(&mut display);
        }

        let _ = display.flush().await;


        // check button for HID Events
        if let Some(_hid_event) = button_monitor.update().await
        {
            has_hid_event = true;
            // handle the event
            // ux.handle_event(hid_event);
        }
        else {
            has_hid_event = false;
        }
    }
    // let mut ux = Ux::new();
}

