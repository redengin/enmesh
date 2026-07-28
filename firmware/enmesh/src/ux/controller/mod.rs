/// provide status led controller
mod status_led;


// provide the shared crates via re-export
use common::*;

// provide logging methods
use log::*;

/// provide scheduling primitives
use crate::prelude::*;

/// provide color conversion
use common::embedded_graphics::draw_target::DrawTargetExt;
/// provide introspection of screen size
use common::embedded_graphics::geometry::OriginDimensions;

use crate::ux::{self, HidEvent, View};
const FRAME_RATE_HZ: u64 = 30; // frames per second

/// provide screens and interaction via button
pub async fn run_ssd1306<ScreenInterface, ScreenSize>(
    global_state: &'static RwLock<NoopRawMutex, crate::State>,
    mut screen: ssd1306::Ssd1306<
        ScreenInterface,
        ScreenSize,
        ssd1306::mode::BufferedGraphicsMode<ScreenSize>,
    >,
    mut power_control: impl crate::PowerControl,
    mut button: impl button::ButtonState,
    led: impl led::LedState,
) where
    ScreenInterface: display_interface::WriteOnlyDataCommand,
    ScreenSize: ssd1306::size::DisplaySize,
{
    // power on the screen
    power_control.power_on().await;
    Timer::after(Duration::from_secs(1)).await;

    // enable screen.init()
    use ssd1306::mode::DisplayConfig;
    match screen.init() {
        Ok(_) => debug!("screen initialized"),
        Err(e) => {
            warn!("screen not initialized [{:?}]", e);
            return;
        }
    }

    // create the UX (aka set of views)
    let mut ux = crate::ux::Ux::new();
    // create our ux theme
    let theme = crate::ux::themes::Theme::new(screen.size());

    // transmute the simple LED to a StatusLed
    let mut status_led = status_led::StatusLed::new(led);
    status_led.set_mode(status_led::LedStatusMode::OFF);
    status_led.update();

    // create a button monitor
    let mut hid_button  = HidButton::new();

    let mut frame_ticker = Ticker::every(Duration::from_hz(FRAME_RATE_HZ));
    loop {
        // monitor the button
        if let Ok(active) = button.is_active() {
            match hid_button.update(active){
                Some(hid_event) => {
                    ux.handle_event(&hid_event);
                }
                None => { }
            }
        }

        // transmute the screen to support Rgb888
        let mut rgb_screen = screen.color_converted();

        // update the UX
        let model = global_state.read().await.clone();
        ux.refresh(&mut rgb_screen, &model, &theme);
        screen.flush().ok(); // must call flush to commit the changes to the screen

        // await the next cycle
        frame_ticker.next().await;
    }
}

struct HidButton {
    active_frames: u32,
}
impl HidButton {
    pub fn new() -> Self {
        Self { active_frames: 0 }
    }
    pub fn update(&mut self, is_active: bool) -> Option<HidEvent> {
        if is_active {
            self.active_frames += 1;
            return None;
        }

        const DEBOUNCE_FRAMES: u32 = 2;
        if self.active_frames >= DEBOUNCE_FRAMES {
            let button_down_duration = self.active_frames * Duration::from_hz(FRAME_RATE_HZ);
            // convert core::time::Duration -> embassy_time::Duration
            let hid_held_duration = embassy_time::Duration::from_millis(
                crate::ux::HID_HELD_DURATION.as_millis() as u64,
            );
            if button_down_duration >= hid_held_duration {
                return Some(HidEvent::Select);
            } else {
                return Some(HidEvent::Next);
            }
        }
        None
    }
}
