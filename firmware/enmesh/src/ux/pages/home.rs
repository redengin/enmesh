// provide the view primitives
use crate::ux::pages::prelude::*;

// use alloc::string::ToString;

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


    }

    fn update(
        &mut self,
        draw_target: &mut impl DrawTarget<Color = common::embedded_graphics::pixelcolor::Rgb888>,
        theme: &crate::ux::themes::Theme,
        model: &crate::State,
    ) -> bool {
        if self.needs_refresh
        {
            self.refresh(draw_target, theme, model);
            return true;
        }

        let mut updated = false;

        return updated;
    }
}

// impl crate::ux::View for Home {
//     /// repaint the whole display
//     fn refresh(
//         &mut self,
//         display: &mut impl DrawTargetExt<Color = Rgb888>,
//         model: &crate::State,
//         theme: &Theme,
//     ) {
//         // clear the display
//         let _ = display.clear(theme.background.into());

//         // show the information
//         let _ = LinearLayout::vertical(
//             Chain::new(
//                 LinearLayout::horizontal(
//                     Chain::new(Text::new("enmesh", Point::zero(), theme.text_style)).append(
//                         Text::new(model.firmware_version, Point::zero(), theme.text_style),
//                     ),
//                 )
//                 .with_spacing(FixedMargin(5))
//                 .arrange(),
//             )
//             .append(Chain::new(
//                 LinearLayout::horizontal(
//                     Chain::new(Text::new("WiFi:", Point::zero(), theme.text_style)).append(
//                         Text::new(
//                             model.wifi_status.to_string().as_str(),
//                             Point::zero(),
//                             theme.text_style,
//                         ),
//                     ),
//                 )
//                 .with_spacing(FixedMargin(5))
//                 .arrange(),
//             ))
//             .append(Chain::new(
//                 LinearLayout::horizontal(
//                     Chain::new(Text::new("BLE:", Point::zero(), theme.text_style)).append(
//                         Text::new(
//                             model.ble_status.to_string().as_str(),
//                             Point::zero(),
//                             theme.text_style,
//                         ),
//                     ),
//                 )
//                 .with_spacing(FixedMargin(5))
//                 .arrange(),
//             )), // .append(Chain::new(
//                 //     LinearLayout::horizontal(
//                 //         Chain::new(Text::new(model.current_protocol.to_string().as_str(), Point::zero(), theme.text_style))
//                 //         .append(Text::new(model.current_radio_mode.to_string().as_str(), Point::zero(), theme.text_style))
//                 //     )
//                 //     .with_spacing(FixedMargin(5))
//                 //     .arrange(),
//                 // ))
//         )
//         .arrange()
//         .draw(display);
//     }

//     /// handle HidEvent
//     fn handle_event(&mut self, _event: &HidEvent) -> bool {
//         // no events handled
//         false
//     }
// }
