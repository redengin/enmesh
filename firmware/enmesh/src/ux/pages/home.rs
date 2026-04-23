// provide the shared crates via re-export
use crate::ux::*;

pub(crate) struct Home {}

impl Home {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::ux::Page for Home {
    /// repaint the whole display
    fn refresh(
        &mut self,
        display: &mut impl DrawTargetExt<Color = Rgb888>,
        model: &crate::State,
        theme: &Theme,
    ) {
        // clear the display
        let _ = display.clear(theme.background.into());

        let display_area = display.bounding_box();

        use embedded_graphics::text::Text;
        use embedded_layout::layout::linear::LinearLayout;
        use embedded_layout::prelude::*;

        // header
        // FIXME convert model.battery_percent to a string
        let battery_percent_str = "0";
        let _ = LinearLayout::horizontal(
            Chain::new(
                Text::new("enmesh ", Point::zero(), theme.text_style))
                .append(Text::new(model.firmware_version, Point::zero(), theme.text_style))
                // FIXME align right the battery %
                .append(Text::new("      ", Point::zero(), theme.text_style))
                .append(Text::new(battery_percent_str, Point::zero(), theme.text_style))
                .append(Text::new("%", Point::zero(), theme.text_style))
        )
        .arrange()
        .draw(display);

        let _ = LinearLayout::vertical(
            Chain::new(Text::new("A", Point::zero(), theme.text_style))
                .append(Text::new("B", Point::zero(), theme.text_style))
                .append(Text::new("C", Point::zero(), theme.text_style)),
        )
        .with_alignment(horizontal::Center)
        .arrange()
        .align_to(&display_area, horizontal::Center, vertical::Center)
        .draw(display);

    }

    /// handle HidEvent
    fn handle_event(&mut self, _event: &HidEvent) -> bool {
        // no events handled
        false
    }

    /// update the display
    /// * only needs to update changed items
    fn update(
        &mut self,
        display: &mut impl DrawTargetExt<Color = Rgb888>,
        model: &crate::State,
        theme: &Theme,
    ) {
        // FIXME for now just refresh the whole screen
        self.refresh(display, model, theme);
    }
}
