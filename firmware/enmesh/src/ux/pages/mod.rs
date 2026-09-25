/// provide the shared crates via re-export
use common::*;

pub mod prelude {
    /// provide the shared crates via re-export
    pub use common::*;

    /// provide embedded graphics primitives
    pub use embedded_graphics::prelude::*;
    pub use embedded_graphics::primitives::Rectangle;
    pub use embedded_graphics::primitives::RoundedRectangle;
    pub use embedded_graphics::text::Text;
    pub use embedded_graphics::text::renderer::TextRenderer;

    pub use embedded_layout::layout::linear::LinearLayout;
    pub use embedded_layout::layout::linear::spacing::DistributeFill;
    pub use embedded_layout::object_chain::Chain;
    /// provide embedded layout primitives
    pub use embedded_layout::prelude::*;
}

/// provide Page primitives
use prelude::*;

/// provide Widgets
use crate::ux::pages::widgets::prelude::*;

const PAGE_COUNT: usize = 4;
pub struct PageController {
    // current_page: pages::Pages,
    tab_bar: TabBar<PAGE_COUNT>,
    battery_widget: BatteryWidget,
    needs_refresh: bool,
}
impl PageController {
    pub fn new() -> Self {
        Self {
            tab_bar: TabBar::<PAGE_COUNT>::new(),
            battery_widget: BatteryWidget::new(),
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

        // clear the display if needs full refresh
        if self.needs_refresh {
            display.clear(theme.background).ok();
            has_changed = true;
        }

        // provide space for drawer
        let drawer_height = theme.text_style.line_height();

        // update the page
        let _page_area = display.cropped(&Rectangle {
            top_left: Point::zero(),
            size: Size::new(
                display.bounding_box().size.width,
                display.bounding_box().size.height - drawer_height,
            ),
        });
        // TODO choose screen to update/refresh

        // partition drawer into region for tab bar and battery widget
        let battery_widget_size = Size::new(
            (2 * theme.text_style.line_height()),
            (theme.text_style.line_height() as f32 * 0.8) as u32,
        );
        let battery_widget_width = 2 * theme.text_style.line_height();
        let tab_bar_width = (display.bounding_box().size.width - battery_widget_width) as i32;

        // update the tab bar
        let mut tab_bar_area = display.cropped(&Rectangle {
            top_left: Point::new(
                0,
                (display.bounding_box().size.height - drawer_height)
                    .try_into()
                    .expect("should fit"),
            ),
            size: Size::new(
                display.bounding_box().size.width - battery_widget_size.width,
                drawer_height,
            ),
        });
        has_changed = self.tab_bar.update(&mut tab_bar_area, theme, model) || has_changed;

        // update the battery widget
        let mut battery_area = display.cropped(&Rectangle {
            top_left: Point::new(
                tab_bar_width,
                (display.bounding_box().size.height - drawer_height) as i32,
            ),
            size: battery_widget_size,
        });
        has_changed = self.battery_widget.update(&mut battery_area, theme, model) || has_changed;

        // provide BLE pairing dialog overlay
        use crate::state::BleStatus;
        match model.ble_status {
            BleStatus::Pairing { passkey: _ } => {
                // TODO use BlePairingDialog widget
            }
            _ => { /* ignored */ }
        }

        // screen has been refreshed
        if self.needs_refresh {
            self.needs_refresh = false;
        }

        return has_changed;
    }

    pub fn handle_event(&mut self, event: &crate::ux::HidEvent) {
        // TODO pass event to page

        self.tab_bar.handle_event(event);
    }
}

pub trait View {
    /// repaint the whole view
    fn refresh(
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
    fn handle_event(&mut self, _event: &crate::ux::HidEvent) -> bool {
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
