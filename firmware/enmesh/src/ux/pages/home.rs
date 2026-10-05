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

        // show the information
        Image::new(&generated::images::enmesh_logo_20x20(), Point::zero())
        .draw(draw_target).ok();

        // let _ = LinearLayout::vertical(
        //     Chain::new(
        //         LinearLayout::horizontal(
        //             Chain::new(Text::new("enmesh", Point::zero(), theme.text_style)).append(
        //                 Text::new(model.firmware_version, Point::zero(), theme.text_style),
        //             ),
        //         )
        //         .with_spacing(FixedMargin(5))
        //         .arrange(),
        //     )
        //     .append(Chain::new(
        //         LinearLayout::horizontal(
        //             Chain::new(Text::new("WiFi:", Point::zero(), theme.text_style)).append(
        //                 Text::new(
        //                     model.wifi_status.to_string().as_str(),
        //                     Point::zero(),
        //                     theme.text_style,
        //                 ),
        //             ),
        //         )
        //         .with_spacing(FixedMargin(5))
        //         .arrange(),
        //     ))
        //     .append(Chain::new(
        //         LinearLayout::horizontal(
        //             Chain::new(Text::new("BLE:", Point::zero(), theme.text_style)).append(
        //                 Text::new(
        //                     model.ble_status.to_string().as_str(),
        //                     Point::zero(),
        //                     theme.text_style,
        //                 ),
        //             ),
        //         )
        //         .with_spacing(FixedMargin(5))
        //         .arrange(),
        //     )),
        // )
        // .arrange()
        // .draw(draw_target);
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
