/// provide Page primitives
use crate::ux::pages::prelude::*;

pub struct TabBar<const TAB_COUNT: usize> {
    pub current_tab: usize,
    needs_refresh: bool,
}
impl<const TAB_COUNT: usize> TabBar<TAB_COUNT> {
    pub fn new() -> Self {
        Self {
            current_tab: 0,
            needs_refresh: true,
        }
    }
}

impl<const TAB_COUNT: usize> crate::ux::pages::View for TabBar<TAB_COUNT> {
    fn refresh(
        &mut self,
        draw_target: &mut impl common::embedded_graphics::prelude::DrawTarget<
            Color = common::embedded_graphics::pixelcolor::Rgb888,
        >,
        theme: &crate::ux::themes::Theme,
        _model: &crate::State,
    ) {
        // clear the area
        draw_target.clear(theme.background).ok();

        // draw the tab bar
        let tab_size = theme.text_style.line_height() >> 1;
        let unselected_style: PrimitiveStyle<Rgb888> = PrimitiveStyleBuilder::new()
            .stroke_width(1)
            .stroke_color(Rgb888::WHITE)
            .fill_color(Rgb888::WHITE)
            .build();
        let unselected_tab = Rectangle::new(Point::zero(), Size{width: tab_size, height: tab_size})
        .into_styled(unselected_style);
        let mut tabs = [unselected_tab; TAB_COUNT];

        // mark the selected tab
        let selected_style: PrimitiveStyle<Rgb888> = PrimitiveStyleBuilder::new()
            .stroke_width(1)
            .stroke_color(Rgb888::WHITE)
            .fill_color(Rgb888::BLACK)
            .build();
        tabs[self.current_tab] = Rectangle::new(Point::zero(), Size{width: tab_size, height: tab_size})
        .into_styled(selected_style);

        LinearLayout::horizontal(Views::new(&mut tabs))
            .with_spacing(DistributeFill(draw_target.bounding_box().size.width))
            .arrange()
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
        draw_target: &mut impl common::embedded_graphics::prelude::DrawTarget<
            Color = common::embedded_graphics::pixelcolor::Rgb888,
        >,
        theme: &crate::ux::themes::Theme,
        model: &crate::State,
    ) -> bool {
        if self.needs_refresh {
            self.refresh(draw_target, theme, model);
            self.needs_refresh = false;
            return true;
        }

        return false;
    }

    fn handle_event(&mut self, event: &crate::ux::HidEvent) -> bool {
        match event {
            crate::ux::HidEvent::Next => {
                self.current_tab += 1;
                self.current_tab %= TAB_COUNT;
                self.needs_refresh = true;
                return true;
            }
            crate::ux::HidEvent::Previous => {
                if self.current_tab > 0 {
                    self.current_tab -= 1;
                }
                else {
                    self.current_tab = TAB_COUNT - 1;
                }
                self.needs_refresh = true;
                return true;
            }
            _ => return false,
        };
    }
}
