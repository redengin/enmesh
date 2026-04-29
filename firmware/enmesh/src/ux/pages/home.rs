// provide the page common crates
use crate::ux::pages::prelude::*;

use alloc::string::ToString;

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
        let _ = LinearLayout::vertical(
            Chain::new(
                LinearLayout::horizontal(
                    Chain::new(Text::new("enmesh", Point::zero(), theme.text_style)).append(
                        Text::new(model.firmware_version, Point::zero(), theme.text_style)
                    ),
                )
                .with_spacing(FixedMargin(5))
                .arrange(),
            )
            .append(Chain::new(
                LinearLayout::horizontal(
                    Chain::new(Text::new("WiFi:", Point::zero(), theme.text_style))
                    .append(Text::new(model.wifi_status.to_string().as_str(), Point::zero(), theme.text_style))
                )
                .with_spacing(FixedMargin(5))
                .arrange(),
            ))
            .append(Chain::new(
                LinearLayout::horizontal(
                    Chain::new(Text::new("BLE:", Point::zero(), theme.text_style))
                        .append(Text::new(model.ble_status.to_string().as_str(), Point::zero(), theme.text_style))
                )
                    .with_spacing(FixedMargin(5))
                    .arrange(),
            ))
            // .append(Chain::new(
            //     LinearLayout::horizontal(
            //         Chain::new(Text::new(model.current_protocol.to_string().as_str(), Point::zero(), theme.text_style))
            //         .append(Text::new(model.current_radio_mode.to_string().as_str(), Point::zero(), theme.text_style))
            //     )
            //     .with_spacing(FixedMargin(5))
            //     .arrange(),
            // ))
        )
        .arrange()
        .draw(display);

        // TODO show the battery status
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
