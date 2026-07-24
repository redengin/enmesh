/// provide UX LED control
pub(crate) mod led;

/// provide UX themes and pages
pub(crate) mod themes;
// pub(crate) mod pages;


// FIXME
// pub mod ssd1306;

// // provide the shared crates via re-export
// use common::*;

// /// provide primitives necessary to use enmesh firmware
// use crate::prelude::*;

// /// provide embedded graphics primitives
// use embedded_graphics::prelude::*;


// pub async fn run<D: DrawTarget + embedded_graphics::geometry::OriginDimensions>(
//     global_state: &'static RwLock<NoopRawMutex, crate::State>,
//     mut screen: D,
//     mut button: impl button::ButtonState,
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

