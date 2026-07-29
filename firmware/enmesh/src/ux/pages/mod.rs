/// provide page implementations
pub mod home;
pub mod meshcore;
pub mod meshtastic;


pub(crate) enum Pages {
    Page0(home::Home),
    Page1(meshcore::MeshCore),
    Page2(meshtastic::Meshtastic),
}
impl Pages {
    pub const count:usize = 3;

    pub fn index(&self) -> usize {
        return match self {
            Self::Page0(_) => 0,
            Self::Page1(_page) => 1,
            Self::Page2(_) => 2,
        }
    }

    pub(crate) fn next(&self) -> Self {
        return match self {
            Self::Page0(_) => Self::Page1(meshcore::MeshCore::new()),
            Self::Page1(_page) => Self::Page2(meshtastic::Meshtastic::new()),
            Self::Page2(_) => Self::Page0(home::Home::new()),
        }
    }

    pub(crate) fn previous(&self) -> Self {
        return match self {
            Self::Page0(_) => Self::Page2(meshtastic::Meshtastic::new()),
            Self::Page1(_page) => Self::Page0(home::Home::new()),
            Self::Page2(_) => Self::Page1(meshcore::MeshCore::new()),
        }
    }
}
impl crate::ux::View for Pages {
    fn refresh(
        &mut self,
        display: &mut impl common::embedded_graphics::prelude::DrawTargetExt<Color = common::embedded_graphics::pixelcolor::Rgb888>,
        model: &crate::State,
        theme: &prelude::Theme,
    ) {
        match self {
            Self::Page0(page) => page.refresh(display, model, theme),
            Self::Page1(page) => page.refresh(display, model, theme),
            Self::Page2(page) => page.refresh(display, model, theme),
        }
    }

    fn handle_event(&mut self, event: &prelude::HidEvent) -> bool {
        return match self {
            Self::Page0(page) => page.handle_event(event),
            Self::Page1(page) => page.handle_event(event),
            Self::Page2(page) => page.handle_event(event),
        }
    }
}

/// provide the necessary primitives for page implementation
pub mod prelude {
    // provide the shared crates via re-export
    pub use common::*;

    // provide embedded graphics primitives
    pub use embedded_graphics::prelude::*;
    pub use embedded_graphics::pixelcolor::Rgb888;
    pub use embedded_graphics::text::Text;
    pub use embedded_graphics::primitives::PrimitiveStyleBuilder;
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