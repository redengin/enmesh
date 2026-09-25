use common::embedded_graphics::{
    pixelcolor::Rgb888,
    primitives::{PrimitiveStyleBuilder, StyledDrawable},
};

/// provide Page primitives
use crate::ux::pages::prelude::*;

pub struct BatteryWidget {
    last_battery_percent: u8,
}
impl BatteryWidget {
    pub fn new() -> Self {
        Self {
            last_battery_percent: 255,
        }
    }
}
impl crate::ux::pages::View for BatteryWidget {
    fn refresh(
        &mut self,
        draw_target: &mut impl DrawTarget<Color = embedded_graphics::pixelcolor::Rgb888>,
        theme: &crate::ux::themes::Theme,
        model: &crate::State,
    ) {
        // draw the main part of battery icon
        let main_width = (draw_target.bounding_box().size.width as f32 * 0.95) as u32;
        RoundedRectangle::with_equal_corners(
            Rectangle {
                top_left: Point::zero(),
                size: Size::new(main_width, draw_target.bounding_box().size.height),
            },
            Size::new(3, 2),
        )
        .draw_styled(
            // &PrimitiveStyleBuilder::new().fill_color(theme.color).build(),
            &PrimitiveStyleBuilder::new()
                .stroke_width(1)
                .stroke_color(theme.color)
                .build(),
            draw_target,
        )
        .ok();

        // draw the tip of the battery
        Rectangle {
            top_left: Point::new(main_width as i32, 0),
            size: Size {
                width: (draw_target.bounding_box().size.width - main_width),
                height: (draw_target.bounding_box().size.height as f32 * 0.5) as u32,
            },
        }
        .align_to(
            &draw_target.bounding_box(),
            horizontal::Right,
            vertical::Center,
        )
        .draw_styled(
            &PrimitiveStyleBuilder::new().fill_color(theme.color).build(),
            draw_target,
        )
        .ok();

        // draw the battery percent
        Text::new("100", Point::zero(), theme.small_style)
            .align_to(
                &draw_target.bounding_box(),
                horizontal::Center,
                vertical::Center,
            )
            .draw(draw_target)
            .ok();
    }

    fn update(
        &mut self,
        draw_target: &mut impl DrawTarget<Color = embedded_graphics::pixelcolor::Rgb888>,
        theme: &crate::ux::themes::Theme,
        model: &crate::State,
    ) -> bool {
        if model.battery_percent != self.last_battery_percent {
            self.refresh(draw_target, theme, model);
            return true;
        }
        return false;
    }

    fn handle_event(&mut self, _event: &crate::ux::HidEvent) -> bool {
        // default doesn't handle event
        false
    }
}
