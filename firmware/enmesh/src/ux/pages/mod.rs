/// provide the shared crates via re-export
use common::*;

pub mod prelude {
    /// provide the shared crates via re-export
    pub use common::*;

    /// provide embedded graphics primitives
    pub use embedded_graphics::prelude::*;
    pub use embedded_graphics::primitives::Rectangle;
    pub use embedded_graphics::text::Text;
    pub use embedded_graphics::text::renderer::TextRenderer;

    /// provide embedded layout primitives
    pub use embedded_layout::prelude::*;
    pub use embedded_layout::object_chain::Chain;
}

/// provide Page primitives
use prelude::*;


pub struct Theme<'a, COLOR> {
    pub color: COLOR,
    pub background: COLOR,
    pub header_style: embedded_graphics::mono_font::MonoTextStyle<'a, COLOR>,
    pub label_style: embedded_graphics::mono_font::MonoTextStyle<'a, COLOR>,
    pub text_style: embedded_graphics::mono_font::MonoTextStyle<'a, COLOR>,
}

pub struct PageController {
    // current_page: pages::Pages,
    needs_refresh: bool,
}
impl PageController {
    pub fn new() -> Self {
        Self {
            needs_refresh: true,
        }
    }

    /// returns true if display has been changed
    pub fn update(
        &mut self,
        display: &mut impl DrawTarget<Color = embedded_graphics::pixelcolor::Rgb888>,
        theme: &crate::ux::themes::Theme,
        model: &crate::State,
    ) -> bool {
        let mut has_changed = false;

        // FIXME need a tab bar widget
        let TAB_BAR_HEIGHT = theme.text_style.line_height();

        // create a screen region for the page contents
        let _page_area = display.cropped(&Rectangle {
            top_left: Point::zero(),
            size: Size::new(
                display.bounding_box().size.width,
                display.bounding_box().size.height - TAB_BAR_HEIGHT,
            ),
        });
        // update the page
        if self.needs_refresh {
            display.clear(theme.background).ok();
            // TODO refresh the current page
            self.needs_refresh = false;
            has_changed = true;
        } else {
            // TODO update the current page
            // has_changed = <page>.update();
        }

        // draw the tab bar
        // TODO

        // provide BLE pairing dialog overlay
        use crate::state::BleStatus;
        match model.ble_status {
            BleStatus::Pairing { passkey: _ } => {
                // TODO use BlePairingDialog widget
            }
            _ => { /* ignored */ }
        }

        return has_changed;
    }

    pub fn handle_event(&mut self, _event: &crate::ux::HidEvent) {
        // TODO
    }
}

pub trait View {
    /// repaint the whole view
    fn refresh (
        &mut self,
        draw_target: &mut impl DrawTarget<Color = embedded_graphics::pixelcolor::Rgb888>,
        theme: &crate::ux::themes::Theme,
        model: &crate::State,
    );

    /// update the view
    /// * only needs to update changes
    /// returns true if display changed
    fn update(
        &mut self,
        draw_target: &mut impl DrawTarget<Color = embedded_graphics::pixelcolor::Rgb888>,
        theme: &crate::ux::themes::Theme,
        model: &crate::State,
    ) -> bool;

    /// handle HidEvent
    /// returns true if the event was handled and should not be bubbled up
    fn handle_event(&mut self, _event: &crate::ux::HidEvent) -> bool
    {
        // default doesn't handle event
        false
    }
}


/// provide reusable Views
pub mod widgets;





// /// provide page implementations
// pub mod home;
// pub mod meshcore;
// pub mod meshtastic;

// pub(crate) enum Pages {
//     Page0(home::Home),
//     Page1(meshcore::MeshCore),
//     Page2(meshtastic::Meshtastic),
// }
// impl Pages {
//     // pub const COUNT:usize = 3;

//     pub fn index(&self) -> usize {
//         return match self {
//             Self::Page0(_) => 0,
//             Self::Page1(_page) => 1,
//             Self::Page2(_) => 2,
//         }
//     }

//     pub(crate) fn next(&self) -> Self {
//         return match self {
//             Self::Page0(_) => Self::Page1(meshcore::MeshCore::new()),
//             Self::Page1(_page) => Self::Page2(meshtastic::Meshtastic::new()),
//             Self::Page2(_) => Self::Page0(home::Home::new()),
//         }
//     }

//     pub(crate) fn previous(&self) -> Self {
//         return match self {
//             Self::Page0(_) => Self::Page2(meshtastic::Meshtastic::new()),
//             Self::Page1(_page) => Self::Page0(home::Home::new()),
//             Self::Page2(_) => Self::Page1(meshcore::MeshCore::new()),
//         }
//     }
// }
// impl crate::ux::View for Pages {
//     fn refresh(
//         &mut self,
//         display: &mut impl common::embedded_graphics::prelude::DrawTargetExt<Color = common::embedded_graphics::pixelcolor::Rgb888>,
//         model: &crate::State,
//         theme: &prelude::Theme,
//     ) {
//         match self {
//             Self::Page0(page) => page.refresh(display, model, theme),
//             Self::Page1(page) => page.refresh(display, model, theme),
//             Self::Page2(page) => page.refresh(display, model, theme),
//         }
//     }

//     fn handle_event(&mut self, event: &prelude::HidEvent) -> bool {
//         return match self {
//             Self::Page0(page) => page.handle_event(event),
//             Self::Page1(page) => page.handle_event(event),
//             Self::Page2(page) => page.handle_event(event),
//         }
//     }
// }

// /// provide the necessary primitives for page implementation
// pub mod prelude {
//     // provide the shared crates via re-export
//     pub use common::*;

//     // provide embedded graphics primitives
//     pub use embedded_graphics::prelude::*;
//     pub use embedded_graphics::pixelcolor::Rgb888;
//     pub use embedded_graphics::text::Text;
//     pub use embedded_graphics::primitives::PrimitiveStyleBuilder;
//     pub use embedded_graphics::{primitives::Rectangle, text::renderer::TextRenderer};

//     // provide embedded layout primitives
//     pub use embedded_layout::layout::linear::spacing::*;
//     pub use embedded_layout::layout::linear::*;
//     pub use embedded_layout::prelude::*;

//     // provide format without allocation
//     pub use heapless::format;

//     // provide enmesh ux primitives
//     pub use crate::ux::HidEvent;
//     pub use crate::ux::themes::Theme;
// }
