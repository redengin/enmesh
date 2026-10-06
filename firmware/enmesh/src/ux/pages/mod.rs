/// shared Page primitives
pub mod prelude {
    /// provide the shared crates via re-export
    pub use common::*;

    /// provide View trait
    pub use super::View;

    /// provide string creation
    pub use crate::alloc::string::ToString;

    /// provide embedded graphics primitives
    pub use embedded_graphics::prelude::*;
    pub use embedded_graphics::mono_font::{self, MonoTextStyle};
    pub use embedded_graphics::pixelcolor::*;
    pub use embedded_graphics::primitives::*;
    pub use embedded_graphics::text::Text;
    pub use embedded_graphics::text::renderer::TextRenderer;
    pub use embedded_graphics::image::Image;

    /// provide additional fonts
    pub use common::profont;

    /// provide embedded layout primitives
    pub use embedded_layout::prelude::*;
    pub use embedded_layout::layout::linear::FixedMargin;
    pub use embedded_layout::layout::linear::LinearLayout;
    pub use embedded_layout::layout::linear::spacing::DistributeFill;
    pub use embedded_layout::object_chain::Chain;
    pub use embedded_layout::View as LayoutView;
}

/// provide the shared crates via re-export
use common::*;

/// provide logging primitives
use log::*;
const TAG: &str = "[PageController]";

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
    // dialogs
    ble_pairing_dialog: bool,
}
impl PageController {
    pub fn new() -> Self {
        Self {
            tab_bar: TabBar::<PAGE_COUNT>::new(),
            battery_widget: BatteryWidget::new(),
            needs_refresh: true,
            // pages
            home: home::Home::new(),
            // dialogs
            ble_pairing_dialog: false,
        }
    }

    /// returns true if display has been changed
    pub fn update(
        &mut self,
        display: &mut impl DrawTarget<Color = embedded_graphics::pixelcolor::Rgb888>,
        theme: &crate::ux::themes::Theme,
        model: &crate::State,
    ) -> bool {
        let mut has_changed = self.needs_refresh;

        // show BLE pairing dialog overlay upon Pairing
        use crate::state::BleStatus;
        match model.ble_status {
            BleStatus::Pairing { passkey } => {
                if !self.ble_pairing_dialog {
                    BlePairingDialog{passkey}.draw(display).ok();
                    self.ble_pairing_dialog = true;
                    has_changed = true;
                }
                // while the dialog is active don't update the page
                return has_changed;
            }
            _ => {
                if self.ble_pairing_dialog {
                    self.ble_pairing_dialog = false;
                    has_changed = true;
                }
            }
        }

        // provide space for drawer
        let drawer_height = theme.text_style.line_height();

        // update the page
        let mut page_area = display.cropped(&Rectangle {
            top_left: Point::zero(),
            size: Size::new(
                display.bounding_box().size.width,
                display.bounding_box().size.height - drawer_height,
            ),
        });
        match self.tab_bar.current_tab {
            _ => {
                if self.needs_refresh {
                    self.home.refresh(&mut page_area, theme, model);
                } else {
                    has_changed |= self.home.update(&mut page_area, theme, model);
                }
            }
        }

        // partition drawer into region for tab bar and battery widget
        let battery_widget_size = Size::new(
            2 * theme.text_style.line_height(),
            (theme.text_style.line_height() as f32 * 0.8) as u32,
        );
        let spacer_width = theme.text_style.line_height() / 3;

        // update the tab bar
        let mut tab_bar_area = display.cropped(&Rectangle {
            top_left: Point::new(
                spacer_width as i32,
                (display.bounding_box().size.height - drawer_height)
                    .try_into()
                    .expect("should fit"),
            ),
            size: Size::new(
                display.bounding_box().size.width
                    - spacer_width
                    - spacer_width
                    - battery_widget_size.width,
                drawer_height,
            ),
        });
        if self.needs_refresh {
            self.tab_bar.refresh(&mut tab_bar_area, theme, model);
        } else {
            has_changed |= self.tab_bar.update(&mut tab_bar_area, theme, model);
        }

        // update the battery widget
        let mut battery_area = display.cropped(&Rectangle {
            top_left: Point::new(
                (display.bounding_box().size.width - battery_widget_size.width) as i32,
                (display.bounding_box().size.height - drawer_height) as i32,
            ),
            size: battery_widget_size,
        });
        if self.needs_refresh {
            self.battery_widget.refresh(&mut battery_area, theme, model);
        } else {
            has_changed |= self.battery_widget.update(&mut battery_area, theme, model);
        }

        // everything has been refreshed
        self.needs_refresh = false;
        return has_changed;
    }

    pub fn handle_event(&mut self, event: &crate::ux::HidEvent) {
        let mut handled = false;
        // pass the unhandled event to dialog
        // TODO

        // pass the unhandled event to the page
        if !handled {
            handled = match self.tab_bar.current_tab {
                _ => self.home.handle_event(event),
            };
        };

        // pass the unhandled event to the tab_bar
        if !handled {
            handled = self.tab_bar.handle_event(event);
        }

        if !handled {
            debug!("{TAG} unhandled event: {:?}", event);
        }
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
