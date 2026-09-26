/// provide Page primitives
use crate::ux::pages::prelude::*;

pub struct BatteryWidget {
    needs_refresh: bool,
    last_battery_state: crate::state::BatteryState,
}
impl BatteryWidget {
    pub fn new() -> Self {
        Self {
            needs_refresh: true,
            last_battery_state: crate::state::BatteryState::NotAvailable,
        }
    }
}
impl crate::ux::pages::View for BatteryWidget {
    fn refresh(
        &mut self,
        draw_target: &mut impl DrawTarget<Color = Rgb888>,
        theme: &crate::ux::themes::Theme,
        model: &crate::State,
    ) {
        // draw the main part of battery icon
        let main_width = (draw_target.bounding_box().size.width as f32 * 0.98) as u32;
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
                height: (draw_target.bounding_box().size.height as f32 * 0.4) as u32,
            },
        }
        .align_to(
            &draw_target.bounding_box(),
            horizontal::Right,
            vertical::Center,
        )
        .draw_styled(
            &PrimitiveStyleBuilder::new()
                .stroke_color(theme.background)
                .fill_color(theme.color)
                .build(),
            draw_target,
        )
        .ok();

        // draw the battery state
        Text::new(&model.battery_state.to_string(), Point::zero(), theme.small_style)
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
        draw_target: &mut impl DrawTarget<Color = Rgb888>,
        theme: &crate::ux::themes::Theme,
        model: &crate::State,
    ) -> bool {
        if self.needs_refresh || (model.battery_state != self.last_battery_state) {
            self.refresh(draw_target, theme, model);
            return true;
        }
        return false;
    }
}
