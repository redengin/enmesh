// provide the shared crates via re-export
use common::*;


/// Listen for packets, and resend non-duplicates
/// if enmesh wifi is enabled
/// * forward packets to the enmesh endpoint(s)
/// * send non-duplicates coming from the enmesh endpoint(s)
pub async fn run<LoraRk, LoraDly>(mut lora_radio: lora_phy::LoRa<LoraRk, LoraDly>)
where
    LoraRk: lora_phy::mod_traits::RadioKind,
    LoraDly: lora_phy::DelayNs,
{
    let mut rx_buffer = [0u8; 1024];
    loop {
        // RX for each supported LoRa protocol

        // FIXME example RX
        let modulation_params = lora_radio.create_modulation_params(
            // spreading_factor,
            lora_phy::mod_params::SpreadingFactor::_7,
            // bandwidth,
            lora_phy::mod_params::Bandwidth::_62KHz,
            // coding_rate,
            lora_phy::mod_params::CodingRate::_4_5,
            // frequency_in_hz
            100
        ).unwrap();
        let packet_params = lora_radio.create_rx_packet_params(
            // preamble_length,
            1,
            // implicit_header,
            true,
            // max_payload_length,
            rx_buffer.len() as u8,
            // crc_on,
            true,
            // iq_inverted,
            true,
            &modulation_params
            ).unwrap();

        match lora_radio.rx(
            // packet_params,
            &packet_params,
            &mut rx_buffer
        ).await
        {
            Ok(_) => { /* TODO */},
            Err(_) => { /* TODO */}
        }

        // TX for each supported LoRa protocol
    }
}
