/// provide logging primitives
use log::*;

/// used to configure the LoRa radio
pub struct LoRaModulationConfig {
    /// [legal frequencies](https://meshtastic.org/docs/configuration/radio/lora/#region)
    pub frequency_hz: u32,

    /// see [Link Budget](https://meshtastic.org/docs/overview/radio-settings/#presets)
    /// for how the bandwidth impacts the range
    pub bandwidth: lora_modulation::Bandwidth,
    /// see [Link Budget](https://meshtastic.org/docs/overview/radio-settings/#presets)
    /// for how the spreading factor impacts the range
    pub spreading_factor: lora_modulation::SpreadingFactor,
    /// see [Link Budget](https://meshtastic.org/docs/overview/radio-settings/#presets)
    /// for how the coding rate impacts the range
    pub coding_rate: lora_modulation::CodingRate,

    /// maximum duration a transmitter can actively transmit
    pub air_time: core::time::Duration,
}

/// used to configure the LoRa send/receive
pub struct LoRaPacketConfig {
    /// smaller preambles minimize power usage
    pub preamble_length: u16,
    pub max_payload_length: u8,
    /// packet CRC will be appended/checked by the radio
    pub crc: bool,
    /// used if network uses statically sized packets (i.e. radio doesn't transmit a LoRa header)
    pub implicit_header: bool,
    /// packet will only be recognized by a receiver using the same iq mode
    /// * doesn't mitigate congestion
    pub iq_inverted: bool,
}

use core::fmt::Error;

pub trait LoRaTxRx {
    fn transmit(&self, raw_packet: &[u8]) -> Result<usize, Error>;
    fn receive(&self, raw_packet: &mut [u8]) -> Result<usize, Error>;
}

/// handle the exchange of LoRa traffic
pub async fn run<LoRaRk, LoRaDly>(lora_radio: lora_phy::LoRa<LoRaRk, LoRaDly>)
where
    LoRaRk: lora_phy::mod_traits::RadioKind,
    LoRaDly: lora_phy::DelayNs,
{
    // configured for Meshcore BOSTON
    let modulation_config = LoRaModulationConfig {
        frequency_hz: 910_525_000,
        bandwidth: lora_modulation::Bandwidth::_62KHz,
        spreading_factor: lora_modulation::SpreadingFactor::_7,
        coding_rate: lora_modulation::CodingRate::_4_5,
        // this is an enmesh extension
        air_time: core::time::Duration::from_millis(100),
    };
    let packet_config = LoRaPacketConfig {
        preamble_length: 8,
        max_payload_length: 255,
        crc: true,
        implicit_header: false,
        iq_inverted: false,
    };

    // perform a cycle
    do_rx(lora_radio, modulation_config, packet_config).await;

}


async fn do_rx<LoRaRk, LoRaDly>(
    mut lora_radio: lora_phy::LoRa<LoRaRk, LoRaDly>,
    modulation_config: LoRaModulationConfig,
    packet_config:  LoRaPacketConfig,
)
    where
        LoRaRk: lora_phy::mod_traits::RadioKind,
        LoRaDly: lora_phy::DelayNs,
{
    // configure the radio
    let modulation_params = lora_radio
        .create_modulation_params(
            modulation_config.spreading_factor,
            modulation_config.bandwidth,
            modulation_config.coding_rate,
            modulation_config.frequency_hz,
        )
        .unwrap();
    let packet_params = lora_radio
        .create_rx_packet_params(
            packet_config.preamble_length,
            packet_config.implicit_header,
            packet_config.max_payload_length,
            packet_config.crc,
            packet_config.iq_inverted,
            &modulation_params,
        )
        .unwrap();
    lora_radio
        .prepare_for_rx(
            lora_phy::RxMode::Continuous,
            &modulation_params,
            &packet_params,
        )
        .await
        .unwrap();



    let mut buffer = [0u8; 255];

    loop {
        info!("LoRa: awaiting packet...");
        match lora_radio.rx(&packet_params, &mut buffer).await {
            Ok((len, _status)) => {
                info!(
                    "LoRa: received a packet [length: {len}, status: {:?}]",
                    true
                );
            }
            Err(err) => error!("failed rx [{:?}]", err),
        }
    }

}
