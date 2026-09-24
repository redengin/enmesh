/// provide Page primitives
use crate::ux::pages::prelude::*;

pub struct TabBar {
    count: u8,
    pub current_tab: u8,
    needs_refresh: bool,
}
impl TabBar {
    pub fn new(count: u8) -> Self {
        Self {
            count,
            current_tab: 0,
            needs_refresh: true,
        }
    }
}

impl crate::ux::pages::View for TabBar {

    fn refresh(
        &mut self,
        draw_target: &mut impl common::embedded_graphics::prelude::DrawTarget<Color = common::embedded_graphics::pixelcolor::Rgb888>,
        theme: &crate::ux::themes::Theme,
        _model: &crate::State,
    ) {
        // draw the tab bar
        // FIXME only supports a specific number of tabs (3)
        const SELECTED: &str = "^";
        const NOT_SELECTED: &str = "-";
        LinearLayout::horizontal(
            Chain::new(Text::new(
                if self.current_tab == 0 {
                    SELECTED
                } else {
                    NOT_SELECTED
                },
                Point::zero(),
                theme.text_style,
            ))
            .append(Text::new(
                if self.current_tab == 1 {
                    SELECTED
                } else {
                    NOT_SELECTED
                },
                Point::zero(),
                theme.text_style,
            ))
            .append(Text::new(
                if self.current_tab == 2 {
                    SELECTED
                } else {
                    NOT_SELECTED
                },
                Point::zero(),
                theme.text_style,
            )),
        )
        .with_spacing(DistributeFill(draw_target.bounding_box().size.width))
        .arrange()
        .align_to(&draw_target.bounding_box(), horizontal::Left, vertical::Bottom)
        .draw(draw_target)
        .ok();


    }

    fn update(
        &mut self,
        draw_target: &mut impl common::embedded_graphics::prelude::DrawTarget<Color = common::embedded_graphics::pixelcolor::Rgb888>,
        theme: &crate::ux::themes::Theme,
        model: &crate::State,
    ) -> bool
    {
        if self.needs_refresh
        {
            self.refresh(draw_target, theme, model);
            return true;
        }

        return false;
    }

    fn handle_event(&mut self, event: &crate::ux::HidEvent) -> bool {
        match event {
            crate::ux::HidEvent::Next => {
                self.current_tab += 1;
                self.current_tab %= self.count;
                self.needs_refresh = true;
                return true;
            }
            crate::ux::HidEvent::Previous => {
                self.current_tab -= 1;
                self.current_tab %= self.count;
                self.needs_refresh = true;
                return true;
            }

            _ => return false
        };
    }
}
