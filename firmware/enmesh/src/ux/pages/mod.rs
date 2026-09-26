/// shared Page primitives
pub mod prelude {
    /// provide the shared crates via re-export
    use common::*;

    /// provide View trait
    pub use super::View;

    /// provide string creation
    pub use crate::alloc::string::ToString;

    /// provide embedded graphics primitives
    pub use embedded_graphics::prelude::*;
    pub use embedded_graphics::pixelcolor::*;
    pub use embedded_graphics::mono_font::{self, MonoTextStyle};
    pub use embedded_graphics::primitives::*;
    pub use embedded_graphics::text::Text;
    pub use embedded_graphics::text::renderer::TextRenderer;

    /// provide additional fonts
    pub use common::profont;

    /// provide embedded layout primitives
    pub use embedded_layout::prelude::*;
    pub use embedded_layout::layout::linear::LinearLayout;
    pub use embedded_layout::layout::linear::spacing::DistributeFill;
    pub use embedded_layout::object_chain::Chain;
}

/// provide the shared crates via re-export
use common::*;

use postcard::ser_flavors::HVec;
/// provide Page primitives
use prelude::*;

/// provide Widgets
mod widgets;
use crate::ux::pages::widgets::prelude::*;

/// provide pages
mod home;

const PAGE_COUNT: usize = 4;
pub struct PageController {
    tab_bar: TabBar<PAGE_COUNT>,
    battery_widget: BatteryWidget,
    needs_refresh: bool,
    // pages
    home: home::Home,
}
impl PageController {
    pub fn new() -> Self {
        Self {
            tab_bar: TabBar::<PAGE_COUNT>::new(),
            battery_widget: BatteryWidget::new(),
            needs_refresh: true,
            // pages
            home: home::Home::new(),
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
            2 * theme.text_style.line_height(),
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

