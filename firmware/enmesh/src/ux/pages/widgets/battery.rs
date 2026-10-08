/// provide Page primitives
use crate::ux::pages::prelude::*;

use crate::state::BatteryState;

pub struct BatteryWidget {
    needs_refresh: bool,
    last_battery_state: BatteryState,
}
impl BatteryWidget {
    pub fn new() -> Self {
        Self {
            needs_refresh: true,
            last_battery_state: BatteryState::NotAvailable,
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
        // clear the area
        draw_target.clear(theme.background).ok();

        // draw the main part of battery icon
        let body_width = (draw_target.bounding_box().size.width * 92).div_ceil(100); // 92%
        let body = Rectangle {
            top_left: Point::zero(),
            size: Size::new(body_width, draw_target.bounding_box().size.height),
        };
        body.draw_styled(
            &PrimitiveStyleBuilder::new()
                .stroke_width(1)
                .stroke_color(theme.color)
                .build(),
            draw_target,
        )
        .ok();

        // draw the battery state inside the body
        Text::new(
            &model.battery_state.to_string(),
            Point::zero(),
            theme.small_style,
        )
        .align_to(
            &body,
            horizontal::Center,
            vertical::Center,
        )
        .draw(draw_target)
        .ok();

        // draw the tip of the battery
        Rectangle {
            top_left: Point::zero(),
            size: Size {
                width: (draw_target.bounding_box().size.width - body_width),
                height: (draw_target.bounding_box().size.height * 55).div_ceil(100), // 55%
            },
        }
        .align_to(&body, horizontal::LeftToRight, vertical::Center)
        .draw_styled(
            &PrimitiveStyleBuilder::new().fill_color(theme.color).build(),
            draw_target,
        )
        .ok();

        // memo state and mark as refreshed
        self.last_battery_state = model.battery_state;
        self.needs_refresh = false;
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
