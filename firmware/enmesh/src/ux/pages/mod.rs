// provide the shared crates via re-export
// use common::*;

/// provide the necessary primitives for page creation
pub mod prelude {
    // provide enmesh ux
    pub use crate::ux::*;

    pub use embedded_graphics::prelude::*;
    pub use embedded_graphics::pixelcolor::{Rgb888, BinaryColor};
    pub use embedded_graphics::text::Text;
    pub use embedded_graphics::image::ImageRaw;

    pub use embedded_layout::prelude::*;
    pub use embedded_layout::layout::linear::*;
    pub use embedded_layout::layout::linear::spacing::*;
    pub use embedded_layout::align;
}
mod home;
pub(crate) use home::Home as Home;
