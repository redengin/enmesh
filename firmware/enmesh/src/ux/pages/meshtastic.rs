/// provide the page primitives
use crate::ux::pages::prelude::*;

#[derive(Default)]
pub struct Meshtastic {
    needs_refresh: bool,
    // memo'd model state
}
impl Meshtastic {
    pub fn new() -> Self {
        Self {
            needs_refresh: true,
            ..Default::default()
        }
    }
}

impl View for Meshtastic {
    fn refresh(
        &mut self,
        draw_target: &mut impl DrawTarget<Color = common::embedded_graphics::pixelcolor::Rgb888>,
        theme: &crate::ux::themes::Theme,
        model: &crate::State,
    ) {
        // clear the area
        draw_target.clear(theme.background).ok();

        // show the information
        let _ = LinearLayout::vertical(
            Chain::new(Text::new("Meshtastic", Point::zero(), theme.text_style))
            .append(Chain::new(
                LinearLayout::horizontal(
                    Chain::new(Text::new("Freq:", Point::zero(), theme.text_style))
                    .append(Text::new(model.settings.meshtastic_settings.lora_config.modulation_config.frequency_hz.to_string().as_str(), Point::zero(), theme.text_style))
                )
                .with_spacing(FixedMargin(5))
                .arrange(),
            ))
            .append(Chain::new(
                LinearLayout::horizontal(
                    Chain::new(Text::new("Bandwidth:", Point::zero(), theme.text_style))
                    .append(Text::new(model.settings.meshtastic_settings.lora_config.modulation_config.bandwidth.hz().to_string().as_str(), Point::zero(), theme.text_style))
                )
                .with_spacing(FixedMargin(5))
                .arrange(),
            ))
            .append(Chain::new(
                LinearLayout::horizontal(
                    Chain::new(Text::new("Spreading:", Point::zero(), theme.text_style))
                    .append(Text::new(model.settings.meshtastic_settings.lora_config.modulation_config.spreading_factor.factor().to_string().as_str(), Point::zero(), theme.text_style))
                )
                .with_spacing(FixedMargin(5))
                .arrange(),
            ))
            .append(Chain::new(
                LinearLayout::horizontal(
                    Chain::new(Text::new("Coding Rate:", Point::zero(), theme.text_style))
                    .append(Text::new(model.settings.meshtastic_settings.lora_config.modulation_config.coding_rate.denom().to_string().as_str(), Point::zero(), theme.text_style))
                )
                .with_spacing(FixedMargin(5))
                .arrange(),
            ))
        )
        .arrange()
        .draw(draw_target);


        // memo the current state
        // self.last_wifi_status = model.wifi_status;
        // self.last_ble_status = model.ble_status;
        // self.last_storage_free = model.storage_status.free_percent();
        self.needs_refresh = false;
    }

    fn update(
        &mut self,
        draw_target: &mut impl DrawTarget<Color = common::embedded_graphics::pixelcolor::Rgb888>,
        theme: &crate::ux::themes::Theme,
        model: &crate::State,
    ) -> bool {
        if self.needs_refresh {
            self.refresh(draw_target, theme, model);
            return true;
        }
        // // refresh if model data has changed
        // else if (model.wifi_status != self.last_wifi_status)
        //     || (model.ble_status != self.last_ble_status)
        //     || (model.storage_status.free_percent() != self.last_storage_free)
        // {
        //     self.refresh(draw_target, theme, model);
        //     return true;
        // }
        else {
            return false;
        }
    }
}

