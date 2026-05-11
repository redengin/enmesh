// provide the shared crates via re-export
use common::*;

#[derive(Default, Copy, Clone)]
pub struct EnmeshLoRaConfig {
    pub modulation_config: EnmeshLoRaModulationConfig,
    pub packet_config: EnmeshLoRaPacketConfig,
}

/// used to configure the LoRa radio modulation
#[derive(Copy, Clone)]
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

    /// each radio has a unique range (which enmesh firmware will adapt to)
    pub tx_power_dbm: i32,

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
            tx_power_dbm: 0,
            air_time: embassy_time::Duration::from_millis(100),
        }
    }
}

/// used to configure the LoRa packet recognition
#[derive(Default, Copy, Clone)]
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

/// provide rx-tx thread
pub mod thread;

// pub trait LoRa_Protocol_Loop {
//     fn cycle(
//         lora_radio: &mut impl lora_phy::mod_traits::RadioKind,
//         modulation_parmeters: &lora_phy::mod_params::ModulationParams,
//         packet_parmeters: &lora_phy::mod_params::PacketParams,
//     );
// }

/// determine if the LoRa channel is clear for transmission
pub async fn is_channel_clear(
    lora_radio: &mut impl lora_phy::mod_traits::RadioKind,
    modulation_parameters: &lora_phy::mod_params::ModulationParams,
) -> Result<bool, lora_phy::mod_params::RadioError> {
    return match lora_radio.do_cad(&modulation_parameters).await {
        Ok(_) => {
            // this is a gargabe API
            let mut is_active: bool = false;
            let lora_phy_cad: Option<&mut bool> = Some(&mut is_active);
            return match lora_radio
                .process_irq_event(
                    lora_phy::mod_params::RadioMode::ChannelActivityDetection,
                    lora_phy_cad,
                    true,
                )
                .await
            {
                Ok(_) => Ok(!is_active),
                Err(e) => Err(e),
            };
        }
        Err(e) => Err(e),
    };
}

/// set the transmit power, reducing toward radio max if the radio rejects the requested power level
pub async fn set_tx_power(
    lora_radio: &mut impl lora_phy::mod_traits::RadioKind,
    tx_power: i32,
) -> Result<i32 /* actual tx_power */, lora_phy::mod_params::RadioError> {
    const MIN_TX_POWER_DBM: i32 = -9;
    let mut try_tx_power = tx_power;
    loop {
        match lora_radio
            .set_tx_power_and_ramp_time(try_tx_power, None, true)
            .await {
                Ok(_) => return Ok(try_tx_power),
                Err(e) => {
                    if try_tx_power <= MIN_TX_POWER_DBM {
                        return Err(e);
                    }
                    // reduce power and retry
                    try_tx_power -= 1;
                }
            }
    }
}
