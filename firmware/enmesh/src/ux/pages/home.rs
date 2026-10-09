/// provide the page primitives
use crate::ux::pages::prelude::*;

#[derive(Default)]
pub struct Home {
    needs_refresh: bool,
    // memo'd model state
    last_wifi_status: crate::state::WiFiStatus,
    last_ble_status: crate::state::BleStatus,
    last_storage_free: usize,
}
impl Home {
    pub fn new() -> Self {
        Self {
            needs_refresh: true,
            ..Default::default()
        }
    }
}

impl View for Home {
    fn refresh(
        &mut self,
        draw_target: &mut impl DrawTarget<Color = common::embedded_graphics::pixelcolor::Rgb888>,
        theme: &crate::ux::themes::Theme,
        model: &crate::State,
    ) {
        // clear the area
        draw_target.clear(theme.background).ok();

        // create a horizontal spacer
        let left_margin = theme.text_style.line_height().div_ceil(2) as i32;

        // draw the header
        Image::new(&theme.enmesh_logo, Point::zero())
            .draw(draw_target)
            .ok();
        let enmesh_logo_size = theme.enmesh_logo.bounding_box().size();
        LinearLayout::horizontal(
            Chain::new(Text::new("enmesh", Point::zero(), theme.text_style)).append(Text::new(
                model.firmware_version,
                Point::zero(),
                theme.text_style,
            )),
        )
        .with_spacing(FixedMargin(left_margin))
        .arrange()
        .translate_mut(Point::new(enmesh_logo_size.width as i32 + left_margin, 0))
        .draw(draw_target)
        .ok();

        // draw the info
        LinearLayout::vertical(
            Chain::new(
                LinearLayout::horizontal(
                    Chain::new(Text::new("WiFi:", Point::zero(), theme.label_style)).append(
                        Text::new(
                            model.wifi_status.to_string().as_str(),
                            Point::zero(),
                            theme.text_style,
                        ),
                    ),
                )
                .with_spacing(FixedMargin(5))
                .arrange(),
            )
            .append(Chain::new(
                LinearLayout::horizontal(
                    Chain::new(Text::new("BLE:", Point::zero(), theme.label_style)).append(
                        Text::new(
                            model.ble_status.to_string().as_str(),
                            Point::zero(),
                            theme.text_style,
                        ),
                    ),
                )
                .with_spacing(FixedMargin(5))
                .arrange(),
            ))
            .append(Chain::new(
                LinearLayout::horizontal(
                    Chain::new(Text::new("Storage:", Point::zero(), theme.label_style)).append(
                        Text::new(
                            heapless::format!(20;"{}% Free", model.storage_status.free_percent())
                                .unwrap()
                                .as_str(),
                            Point::zero(),
                            theme.text_style,
                        ),
                    ),
                )
                .with_spacing(FixedMargin(5))
                .arrange(),
            )),
        )
        .arrange()
        .translate_mut(Point::new(0, enmesh_logo_size.height as i32))
        .draw(draw_target)
        .ok();

        // memo the current state
        self.last_wifi_status = model.wifi_status;
        self.last_ble_status = model.ble_status;
        self.last_storage_free = model.storage_status.free_percent();
        self.needs_refresh = false;
    }

    fn update(
        &mut self,
        draw_target: &mut impl DrawTarget<Color = common::embedded_graphics::pixelcolor::Rgb888>,
        theme: &crate::ux::themes::Theme,
        model: &crate::State,
    ) -> bool {
        if self.needs_refresh {
            self.refresh(draw_target, theme, model);
            return true;
        }
        // refresh if model data has changed
        else if (model.wifi_status != self.last_wifi_status)
            || (model.ble_status != self.last_ble_status)
            || (model.storage_status.free_percent() != self.last_storage_free)
        {
            self.refresh(draw_target, theme, model);
            return true;
        }
        else {
            return false;
        }
    }
}
