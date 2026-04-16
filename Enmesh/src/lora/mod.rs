


/// used to configure the LoRa radio
pub struct LoRaModulationConfig {
    pub frequency_hz: u32,
    pub bandwidth: lora_modulation::Bandwidth,
    pub spreading_factor: lora_modulation::SpreadingFactor,
    pub coding_rate: lora_modulation::CodingRate,
}

/// used to configure the LoRa send/receive
pub struct LoRaPacketConfig {
    /// smaller preambles minimize power usage
    pub preamble_length: u16,
    pub payload_length: u8,
    /// packet will only be recognized by a receiver using the same iq mode
    pub iq_inverted: bool,
}
