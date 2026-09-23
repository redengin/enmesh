use common::embedded_graphics::draw_target;

/// provide View primitives
use crate::ux::pages::prelude::*;

pub struct TabBar {

}
impl crate::ux::pages::View for TabBar {
    fn refresh(
        &mut self,
        draw_target: &mut impl common::embedded_graphics::prelude::DrawTarget<Color = common::embedded_graphics::pixelcolor::Rgb888>,
        theme: &crate::ux::themes::Theme,
        model: &crate::State,
    ) {
        // tab bar is always visible, so proxy to update()
        self.update(draw_target, theme, model);
    }

    fn update(
        &mut self,
        draw_target: &mut impl common::embedded_graphics::prelude::DrawTarget<Color = common::embedded_graphics::pixelcolor::Rgb888>,
        theme: &crate::ux::themes::Theme,
        model: &crate::State,
    ) -> bool
    {
        // FIXME
        false        
    }

    fn handle_event(&mut self, event: &crate::ux::HidEvent) -> bool {
       // FIXME
       false 
    }
}
