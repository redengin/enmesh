/// provide logging primitives
use log::*;

pub struct EnmeshLoRaChannelConfig {
    pub modulation_config: EnmeshLoRaModulationConfig,
    pub packet_config: EnmeshLoRaPacketConfig,
}

/// used to configure the LoRa radio modulation
pub struct EnmeshLoRaModulationConfig {
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
    pub air_time: embassy_time::Duration,
}

/// used to configure the LoRa packet recognition
pub struct EnmeshLoRaPacketConfig {
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

/// record the signal quality (rssi/snr) to influence transmit power
pub struct ReceivedLoRaPacket {
    pub rssi: i16,
    pub snr: i16,
    pub buffer: [u8],
}

pub trait LoRaProtocol
{
    /// used to trigger radio calibration when the frequency changes
    fn get_lora_frequency_hz(&self) -> u32;

    /// handle LoRa traffic as a repeater
    /// RX -> process -> TX
    /// * process
    ///     * if 
    ///     * handle any repeater specific protocols
    ///     * create a sequence of packets to transmit
    ///         * ordered by repeater based priority
    ///             * generally, repeater-to-repeater traffic should be lower
    ///                 priority than user traffic
    /// * TX
    ///     * transmit packets by priority
    async fn do_cycle(&mut self, lora_radio: &mut impl lora_phy::mod_traits::RadioKind);

    // fn add_tranmit_packet(&mut self, buffer:[u8]);
}

// support Meshtastic
pub mod meshtastic;

// support MeshCore
pub mod meshcore;

// pub struct EnmeshLoRa {
// }
// impl 
//     pub async fn cycle(lora_protcol: &impl LoRaProtocol) {

//     }
// }

/// handle the exchange of LoRa traffic
pub async fn run<LoRaRk, LoRaDly>(lora_radio: lora_phy::LoRa<LoRaRk, LoRaDly>)
where
    LoRaRk: lora_phy::mod_traits::RadioKind,
    LoRaDly: lora_phy::DelayNs,
{
    // configured for Meshcore BOSTON
    let modulation_config = EnmeshLoRaModulationConfig {
        frequency_hz: 910_525_000,
        bandwidth: lora_modulation::Bandwidth::_62KHz,
        spreading_factor: lora_modulation::SpreadingFactor::_7,
        coding_rate: lora_modulation::CodingRate::_4_5,
        // this is an enmesh extension
        air_time: embassy_time::Duration::from_millis(100),
    };
    let packet_config = EnmeshLoRaPacketConfig {
        preamble_length: 8,
        max_payload_length: 255,
        crc: true,
        implicit_header: false,
        iq_inverted: false,
    };

    // perform a cycle
    do_cycle(lora_radio, modulation_config, packet_config).await;
}

async fn do_cycle<LoRaRk, LoRaDly>(
    mut lora_radio: lora_phy::LoRa<LoRaRk, LoRaDly>,
    modulation_config: EnmeshLoRaModulationConfig,
    packet_config: EnmeshLoRaPacketConfig,
) where
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
