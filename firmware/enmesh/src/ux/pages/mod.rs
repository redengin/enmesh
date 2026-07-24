// provide the shared crates via re-export
use common::*;


/// provide the necessary primitives for page creation
pub mod prelude {
    // provide enmesh ux
    pub use crate::ux::*;

    pub use embedded_graphics::prelude::*;
    // pub use embedded_graphics::pixelcolor::{Rgb888, BinaryColor};
    pub use embedded_graphics::pixelcolor::{Rgb888};
    pub use embedded_graphics::primitives::PrimitiveStyleBuilder;
    pub use embedded_graphics::text::Text;
    pub use embedded_graphics::text::renderer::TextRenderer;
    pub use embedded_graphics::mono_font::{MonoTextStyle, ascii::*};
    // pub use embedded_graphics::{primitives::Rectangle, text::renderer::TextRenderer};
    pub use embedded_graphics::primitives::Rectangle;


    // pub use embedded_graphics::{mono_font::{MonoTextStyle, ascii::FONT_10X20}, primitives::PrimitiveStyleBuilder, text::{DecorationColor::TextColor, TextStyleBuilder}};
    // pub use embedded_graphics::image::ImageRaw;

    pub use embedded_layout::prelude::*;
    pub use embedded_layout::layout::linear::*;
    pub use embedded_layout::layout::linear::spacing::*;
    // pub use embedded_layout::align;

    pub use common::heapless::format;
}



mod home;
pub(crate) use home::Home as Home;

mod meshcore;
pub(crate) use meshcore::MeshCore as MeshCore;

mod meshtastic;
pub(crate) use meshtastic::Meshtastic as Meshtastic;
