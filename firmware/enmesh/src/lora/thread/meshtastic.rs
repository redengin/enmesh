// provide the common crates via re-export
use common::*;

use crate::lora::EnmeshLoRaConfig;


pub async fn cycle(
    lora_radio: &mut impl lora_phy::mod_traits::RadioKind,
    lora_config: EnmeshLoRaConfig,
) {

    let modulation_params = lora_radio.create_modulation_params(
        lora_config.modulation_config.spreading_factor,
        lora_config.modulation_config.bandwidth,
        lora_config.modulation_config.coding_rate,
        lora_config.modulation_config.frequency_hz,
    ).unwrap();

    // prioritize sending
    let has_transmit_packets = false; // FIXME - check the packet queue
    if has_transmit_packets {
        if crate::lora::is_channel_clear(lora_radio, &modulation_params).await {
            // channel is clear, transmit packets
            // TODO - transmit packets
        }
    }

    // read packets
    // TODO

}