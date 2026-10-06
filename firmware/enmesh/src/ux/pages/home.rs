/// provide the page primitives
use crate::ux::pages::prelude::*;

pub struct Home {
    needs_refresh: bool,

}
impl Home {
    pub fn new() -> Self {
        Self {
            needs_refresh: true,
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
        .draw(draw_target).ok();
        let enmesh_logo_size= theme.enmesh_logo.bounding_box().size();
        LinearLayout::horizontal(
            Chain::new(Text::new("enmesh", Point::zero(), theme.text_style)).append(
                Text::new(model.firmware_version, Point::zero(), theme.text_style),
            ),
        )
        .with_spacing(FixedMargin(left_margin))
        .arrange()
        .translate_mut(Point::new(enmesh_logo_size.width as i32 + left_margin, 0))
        .draw(draw_target).ok();

        // draw the info
        LinearLayout::vertical(
        Chain::new(
                LinearLayout::horizontal(
                    Chain::new(Text::new("WiFi:", Point::zero(), theme.text_style)).append(
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
                    Chain::new(Text::new("BLE:", Point::zero(), theme.text_style)).append(
                        Text::new(
                            model.ble_status.to_string().as_str(),
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
        .draw(draw_target).ok();
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

        // let mut updated = false;

        // return updated;
        return false;
    }
}
