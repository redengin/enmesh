// provide the shared crates via re-export
use common::*;

#[derive(Default)]
pub struct EnmeshLoRaConfig {
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
impl Default for EnmeshLoRaModulationConfig {
    fn default() -> Self {
        Self {
            frequency_hz: 0,
            bandwidth: lora_modulation::Bandwidth::_250KHz,
            spreading_factor: lora_modulation::SpreadingFactor::_6,
            coding_rate: lora_modulation::CodingRate::_4_5,
            air_time: embassy_time::Duration::from_millis(100),
        } 
    }
}

/// used to configure the LoRa packet recognition
#[derive(Default)]
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

pub async fn run<LoRaRk, LoRaDly>(_lora_radio: lora_phy::LoRa<LoRaRk, LoRaDly>)
where
    LoRaRk: lora_phy::mod_traits::RadioKind,
    LoRaDly: lora_phy::DelayNs,
{
    loop {
        // TODO support UX choice of protocol

    } 
}