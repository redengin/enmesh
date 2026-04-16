



pub struct LoRaModulationConfig {
    pub frequency_hz: u32,
    pub bandwidth: lora_modulation::Bandwidth,
    pub spreading_factor: lora_modulation::SpreadingFactor,
    pub coding_rate: lora_modulation::CodingRate,
}

pub struct LoRaPacketConfig {
    pub preamble_length: u16,
    pub implicit_header: bool,
    pub payload_length: u8,
    pub crc_on: bool,
    pub iq_inverted: bool,
}