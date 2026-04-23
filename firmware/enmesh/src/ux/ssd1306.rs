// provide the shared crates via re-export
use common::*;

// provide logging methods
use log::*;

use embedded_graphics::prelude::{DrawTargetExt, OriginDimensions};
use ssd1306::Ssd1306;
use ssd1306::mode::BufferedGraphicsMode;

/// provide screens and interaction via button
pub async fn run<ScreenInterface, ScreenSize>(
    mut screen: Ssd1306<ScreenInterface, ScreenSize, BufferedGraphicsMode<ScreenSize>>,
    mut power_control: impl crate::PowerControl,
    mut button: impl ButtonState,
) where
    ScreenInterface: display_interface::WriteOnlyDataCommand,
    ScreenSize: ssd1306::size::DisplaySize,
{
    // FIXME needs a state:State parameter
    let state = crate::State::new();


    // power on the screen
    power_control.power_on().await;

    use ssd1306::mode::DisplayConfig; // enable screen.init()
    match screen.init() {
        Ok(_) => debug!("screen initialized"),
        Err(e) => {
            warn!("screen not initialized [{:?}]", e);
            // abort the screen handler thread
            return;
        }
    }

    // create the UX
    let mut ux = crate::ux::Ux::new();
    // create our ux theme
    let screen_size = screen.size();
    let theme = crate::ux::themes::DefaultTheme(screen_size);

    // create a button monitor
    let mut button_active_frames = 0;

    // provide threading primitives
    use embassy_time::Duration;
    const FRAME_RATE: u64 = 30; // frames per second
    let mut frame_ticker = embassy_time::Ticker::every(Duration::from_hz(FRAME_RATE));
    loop {
        // monitor the button
        if let Ok(active) = button.is_active() {
            if active {
                button_active_frames += 1;
            } else {
                const DEBOUNCE_FRAMES: u32 = 2;
                if button_active_frames > DEBOUNCE_FRAMES {
                    let button_down_duration = button_active_frames * Duration::from_hz(FRAME_RATE);
                    // convert core::time::Duration -> embassy_time::Duration
                    let hid_held_duration = embassy_time::Duration::from_millis(
                        crate::ux::HID_HELD_DURATION.as_millis() as u64,
                    );
                    if button_down_duration >= hid_held_duration {
                        ux.handle_event(&crate::ux::HidEvent::Select);
                    } else {
                        ux.handle_event(&crate::ux::HidEvent::Next);
                    }
                }
                // reset the button monitor
                button_active_frames = 0;
            }
        }

        // transmute the screen to support Rgb888
        let mut rgb_screen = screen.color_converted();

        // update the UX
        use crate::ux::Page;
        ux.refresh(&mut rgb_screen, &state, &theme);
        screen.flush().ok(); // must call flush to commit the changes to the screen

        // await the next cycle
        frame_ticker.next().await;
    }
}
