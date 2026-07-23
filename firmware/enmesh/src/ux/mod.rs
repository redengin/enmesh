// provide the shared crates via re-export
use common::*;

/// provide primitives necessary to use enmesh firmware
use crate::prelude::*;

/// UX designed for RGB888
/// * uses embedded_graphics::draw_target::ColorCoverted to support all screens
use embedded_graphics::prelude::*; // provide common traits
use embedded_graphics::pixelcolor::Rgb888;

/// provide the implementation for user interface
mod ux;
pub use ux::Ux;
mod pages;
mod icons;

pub struct Theme<'a> {
    // pub font: embedded_graphics::mono_font::MonoFont<'static>,
    pub text_style: embedded_graphics::mono_font::MonoTextStyle<'a, Rgb888>,
    /// default color for text and mono-icons
    pub color: Rgb888,
    pub background: Rgb888,
}
/// provide themes
pub mod themes;

/// Respond to user interactions
pub enum HidEvent {
    /// move to next selectable item
    Next,
    /// move to the previous selectable item
    Previous,
    /// invokes the selected item's handler
    Select,
    /// finds the touched item and invokes a 'Select' event
    Touch { x: u32, y: u32 },
}
/// active HID input durations greater than this, should generate a HidEvent::Select
pub const HID_HELD_DURATION: core::time::Duration =
    core::time::Duration::from_millis(500);

pub trait Page {
    /// repaint the whole display
    fn refresh(
        &mut self,
        display: &mut impl DrawTargetExt<Color = Rgb888>,
        model: &crate::State,
        theme: &Theme,
    );

    /// handle HidEvent
    /// returns true if the event was handled and should not be managed by the UX
    fn handle_event(&mut self, event: &HidEvent) -> bool;

    /// update the display
    /// * only needs to update changed items
    fn update(
        &mut self,
        display: &mut impl DrawTargetExt<Color = Rgb888>,
        model: &crate::State,
        theme: &Theme,
    );
}

pub mod ssd1306;

// mod led;
// pub async fn run<D: DrawTarget + embedded_graphics::geometry::OriginDimensions>(
//     global_state: &'static RwLock<NoopRawMutex, crate::State>,
//     mut screen: D,
//     mut button: impl ButtonState,
//     mut led: impl embedded_hal::digital::OutputPin,
// )
// where <D as embedded_graphics::draw_target::DrawTarget>::Color: From<embedded_graphics::pixelcolor::Rgb888>
// {
//     let status_led = led::StatusLed::new(led);

//     // create the UX
//     let mut ux = crate::ux::Ux::new(); 
//     // create our ux theme
//     let screen_size = screen.size();
//     let theme = crate::ux::themes::DefaultTheme(screen_size);

//     // monitor the button state change
//     let mut button_active_frames = 0;

//    // provide threading primitives
//     use embassy_time::Duration;
//     const FRAME_RATE: u64 = 30; // frames per second
//     let mut frame_ticker = embassy_time::Ticker::every(Duration::from_hz(FRAME_RATE));
//     loop {
//         // monitor the button
//         if let Ok(active) = button.is_active() {
//             if active {
//                 button_active_frames += 1;
//             } else {
//                 const DEBOUNCE_FRAMES: u32 = 2;
//                 if button_active_frames >= DEBOUNCE_FRAMES {
//                     let button_down_duration = button_active_frames * Duration::from_hz(FRAME_RATE);
//                     // convert core::time::Duration -> embassy_time::Duration
//                     let hid_held_duration = embassy_time::Duration::from_millis(
//                         crate::ux::HID_HELD_DURATION.as_millis() as u64,
//                     );
//                     if button_down_duration >= hid_held_duration {
//                         ux.handle_event(&crate::ux::HidEvent::Select);
//                     } else {
//                         ux.handle_event(&crate::ux::HidEvent::Next);
//                     }
//                 }
//                 // reset the button monitor
//                 button_active_frames = 0;
//             }
//         }

//         // transmute the screen to support Rgb888
//         let mut rgb_screen = screen.color_converted();

//         // update the UX
//         use crate::ux::Page;
//         let model = global_state.read().await.clone();
//         ux.refresh(&mut rgb_screen, &model, &theme);
//         screen.flush().ok(); // must call flush to commit the changes to the screen

//         // await the next cycle
//         frame_ticker.next().await;
//     }
// }
