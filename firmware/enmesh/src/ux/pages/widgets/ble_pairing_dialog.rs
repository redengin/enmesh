use core::ops::Mul;

/// provide page primitives
use crate::ux::pages::prelude::*;

pub struct BlePairingDialog;

impl View for BlePairingDialog {
    fn refresh(
        &mut self,
        draw_target: &mut impl DrawTarget<Color = embedded_graphics::pixelcolor::Rgb888>,
        theme: &crate::ux::themes::Theme,
        model: &crate::State,
    ) {
        if let crate::state::BleStatus::Pairing { passkey } = model.ble_status {
            // draw a framing rectangle
            let frame_width = draw_target.bounding_box().size.width.div_ceil(10).mul(9); // 90%
            let frame_height = draw_target.bounding_box().size.height.div_ceil(3).mul(2); // 66%
            let frame = Rectangle::new(Point::zero(), Size::new(frame_width, frame_height))
                .into_styled(
                    PrimitiveStyleBuilder::new()
                        .stroke_color(theme.color)
                        .stroke_width(1)
                        .fill_color(theme.background)
                        .build(),
                )
                .align_to(
                    &draw_target.bounding_box(),
                    horizontal::Center,
                    vertical::Center,
                );
            frame.draw(draw_target).ok();

            // draw the dialog text
            LinearLayout::vertical(
                Chain::new(Text::new("BLE Pairing", Point::zero(), theme.text_style)).append(
                    Text::new(
                        heapless::format!(6; "{:06}", passkey).unwrap().as_str(),
                        Point::zero(),
                        theme.h1_style,
                    ),
                ),
            )
            .with_alignment(horizontal::Center)
            .arrange()
            .align_to(&frame, horizontal::Center, vertical::Center)
            .draw(draw_target)
            .ok();
        }
    }

    fn update(
        &mut self,
        draw_target: &mut impl DrawTarget<Color = embedded_graphics::pixelcolor::Rgb888>,
        theme: &crate::ux::themes::Theme,
        model: &crate::State,
    ) -> bool {
        // client should only call refresh()
        self.refresh(draw_target, theme, model);
        true
    }
}
