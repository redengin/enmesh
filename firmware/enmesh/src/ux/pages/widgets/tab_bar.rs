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
        // clear the region
        draw_target.clear(theme.background).ok();

        // draw the tab bar
        const SELECTED: &str = "^";
        const NOT_SELECTED: &str = "-";
        let mut tabs = [Text::new(NOT_SELECTED, Point::zero(), theme.text_style); TAB_COUNT];
        // mark the selected tab
        tabs[self.current_tab] = Text::new(SELECTED, Point::zero(), theme.text_style);

        LinearLayout::horizontal(Views::new(&mut tabs))
            .with_spacing(DistributeFill(draw_target.bounding_box().size.width))
            .arrange()
            .align_to(
                &draw_target.bounding_box(),
                horizontal::Left,
                vertical::Bottom,
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
                    self.current_tab %= TAB_COUNT;
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
