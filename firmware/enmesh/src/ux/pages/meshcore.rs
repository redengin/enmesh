// provide the page common crates
use crate::ux::pages::prelude::*;

use alloc::string::ToString;

pub(crate) struct MeshCore{}

impl MeshCore {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::ux::Page for MeshCore {
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
            Chain::new(Text::new("MeshCore", Point::zero(), theme.text_style))
            .append(Chain::new(
                LinearLayout::horizontal(
                    Chain::new(Text::new("Freq:", Point::zero(), theme.text_style))
                    .append(Text::new(model.settings.meshcore_settings.lora_config.modulation_config.frequency_hz.to_string().as_str(), Point::zero(), theme.text_style))
                )
                .with_spacing(FixedMargin(5))
                .arrange(),
            ))
            .append(Chain::new(
                LinearLayout::horizontal(
                    Chain::new(Text::new("Bandwidth:", Point::zero(), theme.text_style))
                    .append(Text::new(model.settings.meshcore_settings.lora_config.modulation_config.bandwidth.hz().to_string().as_str(), Point::zero(), theme.text_style))
                )
                .with_spacing(FixedMargin(5))
                .arrange(),
            ))
            .append(Chain::new(
                LinearLayout::horizontal(
                    Chain::new(Text::new("Spreading:", Point::zero(), theme.text_style))
                    .append(Text::new(model.settings.meshcore_settings.lora_config.modulation_config.spreading_factor.factor().to_string().as_str(), Point::zero(), theme.text_style))
                )
                .with_spacing(FixedMargin(5))
                .arrange(),
            ))
            .append(Chain::new(
                LinearLayout::horizontal(
                    Chain::new(Text::new("Coding Rate:", Point::zero(), theme.text_style))
                    .append(Text::new(model.settings.meshcore_settings.lora_config.modulation_config.coding_rate.denom().to_string().as_str(), Point::zero(), theme.text_style))
                )
                .with_spacing(FixedMargin(5))
                .arrange(),
            ))
        )
        .arrange()
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
