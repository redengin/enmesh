/// provide page implementations
mod home;
// pub(crate) use home::Home;
mod meshcore;
// pub(crate) use meshcore::MeshCore;
mod meshtastic;
// pub(crate) use meshtastic::Meshtastic;


// #[derive(PartialEq, Eq)]
pub(crate) enum Pages {
    Home(home::Home),
//     MeshCore,
//     Meshtastic,
//     // Hibernate,
}
// impl Pages {
//     pub fn next(&self) -> Self {
//         match self {
//             Pages::Home => Pages::MeshCore,
//             Pages::MeshCore => Pages::Meshtastic,
//             Pages::Meshtastic => Pages::Home,
//         }
//     }

//     pub fn previous(&self) -> Self {
//         match self {
//             Pages::Home => Pages::Meshtastic,
//             Pages::Meshtastic => Pages::MeshCore,
//             Pages::MeshCore => Pages::Home,
//         }
//     }
// }

/// provide the necessary primitives for page implementation
pub mod prelude {
    // provide the shared crates via re-export
    pub use common::*;

    // provide embedded graphics primitives
    pub use embedded_graphics::prelude::*;
    pub use embedded_graphics::pixelcolor::Rgb888;
    pub use embedded_graphics::text::Text;
    pub use embedded_graphics::{
        mono_font::{MonoTextStyle, ascii::FONT_10X20},
        primitives::PrimitiveStyleBuilder,
    };
    pub use embedded_graphics::{primitives::Rectangle, text::renderer::TextRenderer};

    // provide embedded layout primitives
    pub use embedded_layout::layout::linear::spacing::*;
    pub use embedded_layout::layout::linear::*;
    pub use embedded_layout::prelude::*;

    // provide format without allocation
    pub use heapless::format;

    // provide enmesh ux primitives
    pub use crate::ux::HidEvent;
    pub use crate::ux::themes::Theme;
}


// use prelude::*;
// pub(crate) trait Page {
//     /// repaint the whole display
//     fn refresh(
//         &mut self,
//         display: &mut impl DrawTargetExt<Color = Rgb888>,
//         model: &crate::State,
//         theme: &Theme,
//     );

//     /// update the display
//     /// * only needs to update display changes
//     fn update(
//         &mut self,
//         display: &mut impl DrawTargetExt<Color = Rgb888>,
//         model: &crate::State,
//         theme: &Theme,
//     );

//     /// handle HidEvent
//     /// returns true if the event was handled and should not be bubbled up
//     fn handle_event(&mut self, event: &HidEvent) -> bool;
// }

