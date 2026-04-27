// provide the shared crates via re-export
use crate::ux::pages::prelude::*;

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

        // show the information
        let version = heapless::format!(16; "enmesh {}", model.firmware_version).unwrap();
        let lora_status =
            heapless::format!(16; "{} {}", model.current_protocol, model.lora_mode).unwrap();
        let _ = LinearLayout::vertical(
            Chain::new(Text::new(version.as_str(), Point::zero(), theme.text_style)).append(
                Text::new(lora_status.as_str(), Point::zero(), theme.text_style),
            ),
        )
        .with_alignment(horizontal::Left)
        .arrange()
        // .align_to(&display_area, horizontal::Center, vertical::Center)
        .draw(display);

        // show the battery status
        // let battery_percent = heapless::format!(4; "{}%", model.battery_percent).unwrap();
        // let _ = LinearLayout::vertical(Chain::new(Text::new(
        //     battery_percent.as_str(),
        //     Point::zero(),
        //     theme.text_style,
        // )))
        // .arrange()
        // .with_alignment(horizontal::Right)
        // // .align_to(&display_area, horizontal::Center, vertical::Center)
        // .draw(display);
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
