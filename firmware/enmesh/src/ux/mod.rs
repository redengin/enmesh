// provide the shared crates via re-export
use common::*;

/// UX designed for RGB888
/// * uses embedded_graphics::draw_target::ColorCoverted to support all screens
use embedded_graphics::prelude::*;  // provide common traits
use embedded_graphics::pixelcolor::Rgb888;

/// provide the implementation for user interface
mod ux;
pub use ux::Ux as Ux;
mod pages;


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
const HID_HELD_MILLIS: u64 = 500;
pub const HID_HELD_DURATION: core::time::Duration = core::time::Duration::from_millis(HID_HELD_MILLIS);

pub trait Page {
    /// repaint the whole display
    fn refresh(&mut self,
                display: &mut impl DrawTargetExt<Color=Rgb888>,
                model: &crate::State,
                theme: &Theme
    );

    /// handle HidEvent
    /// returns true if the event was handled and should not be managed by the UX
    fn handle_event(&mut self, event: &HidEvent) -> bool;

    /// update the display
    /// * only needs to update changed items
    fn update(&mut self, display: &mut impl DrawTargetExt<Color=Rgb888>, theme: &Theme);
}


pub mod ssd1306;