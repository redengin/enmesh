// provide the shared crates via re-export
use common::*;

/// provide embedded graphics primitives
use embedded_graphics::prelude::*;
use embedded_graphics::pixelcolor::Rgb888;

use crate::ux::{themes::Theme, HidEvent};

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

/// provide home page implementation
mod home; pub(crate) use home::Home as Home;

/// provide the necessary primitives for page creation
pub mod prelude {
    // provide enmesh ux primitives
    pub use crate::ux::HidEvent;
    pub use crate::ux::themes::Theme;

    // provide embedded graphics primitives
    pub use common::embedded_graphics::prelude::*;
    pub use common::embedded_graphics::pixelcolor::{Rgb888};
    pub use common::embedded_graphics::text::Text;

    // provide embedded layout primitives
    pub use common::embedded_layout::prelude::*;
    pub use common::embedded_layout::layout::linear::*;

//     // pub use embedded_graphics::pixelcolor::{Rgb888, BinaryColor};
//     pub use embedded_graphics::pixelcolor::{Rgb888};
//     pub use embedded_graphics::primitives::PrimitiveStyleBuilder;
//     pub use embedded_graphics::text::renderer::TextRenderer;
//     pub use embedded_graphics::mono_font::{MonoTextStyle, ascii::*};
//     // pub use embedded_graphics::{primitives::Rectangle, text::renderer::TextRenderer};
//     pub use embedded_graphics::primitives::Rectangle;


//     // pub use embedded_graphics::{mono_font::{MonoTextStyle, ascii::FONT_10X20}, primitives::PrimitiveStyleBuilder, text::{DecorationColor::TextColor, TextStyleBuilder}};
//     // pub use embedded_graphics::image::ImageRaw;

//     pub use embedded_layout::prelude::*;
//     pub use embedded_layout::layout::linear::*;
//     pub use embedded_layout::layout::linear::spacing::*;
//     // pub use embedded_layout::align;

//     pub use common::heapless::format;
}




// mod meshcore;
// pub(crate) use meshcore::MeshCore as MeshCore;

// mod meshtastic;
// pub(crate) use meshtastic::Meshtastic as Meshtastic;
