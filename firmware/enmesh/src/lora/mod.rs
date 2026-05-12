// provide the shared crates via re-export
use common::*;

/// provide rx-tx thread
pub mod thread;

// provide logging primitives
use log::*;

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
    pub buffer_length: usize,
    pub buffer: [u8; 255],
}
impl Default for ReceivedLoRaPacket {
    fn default() -> Self {
        Self {
            rssi: 0,
            snr: 0,
            buffer_length: 0,
            buffer: [0; 255],
        }
    }
}

pub trait LoRaHandler {
    #![allow(async_fn_in_trait)]
    async fn cycle(
        &mut self,
        lora_radio: &mut impl lora_phy::mod_traits::RadioKind,
        lora_config: &EnmeshLoRaConfig,
    ) {
        // logging tag
        const TAG: &str = "LoRaProtocol";

        // prepare radio
        let modulation_params = lora_radio
            .create_modulation_params(
                lora_config.modulation_config.spreading_factor,
                lora_config.modulation_config.bandwidth,
                lora_config.modulation_config.coding_rate,
                lora_config.modulation_config.frequency_hz,
            )
            .unwrap();
        lora_radio
            .set_modulation_params(&modulation_params)
            .await
            .unwrap();
        let packet_params = lora_radio
            .create_packet_params(
                lora_config.packet_config.preamble_length,
                lora_config.packet_config.implicit_header,
                lora_config.packet_config.max_payload_length,
                lora_config.packet_config.crc,
                lora_config.packet_config.iq_inverted,
                &modulation_params,
            )
            .unwrap();
        lora_radio.set_packet_params(&packet_params).await.unwrap();

        // prioritize sending
        if self.has_transmit_packets() {
            // allow implemenations to dynamically scale tx power
            crate::lora::set_tx_power(lora_radio, self.get_tx_power(&lora_config))
                .await
                .unwrap();
            // prepare radio for transmit
            lora_radio.set_irq_params(Some(lora_phy::mod_params::RadioMode::Transmit)).await.unwrap();

            match crate::lora::is_channel_clear(lora_radio, &modulation_params).await {
                Ok(is_clear) => {
                    if is_clear {

                        // transmit packets until airtime expires
                        let tx_stop_time =
                            embassy_time::Instant::now() + lora_config.modulation_config.air_time;

                        // send packets
                        while self.has_transmit_packets() {
                            let packet = self.tx_peek().unwrap();
                            lora_radio.set_payload(&packet.buffer[0..packet.buffer_length]).await.unwrap();
                            match lora_radio.do_tx().await {
                                Ok(_) => {
                                    debug!("{TAG} transmitted packet [size: {}", packet.buffer_length);
                                    self.tx_pop();
                                }
                                Err(e) => {
                                    warn!("{TAG} failed to transmit packet: {:?}", e);
                                    // skip this packet and try again later
                                    break;
                                }
                            }

                            // stop if airtime has expired
                            if embassy_time::Instant::now() > tx_stop_time {
                                debug!("{TAG} air time expired, deferring remaining packets");
                                break;
                            }
                        }
                    }
                }
                Err(e) => {
                    warn!("{TAG} failed to detect channel activity (CAD): {:?}", e);
                    return;
                }
            }

            // remove IRQ params
            lora_radio.set_irq_params(None).await.unwrap();
        }

        // receive packets
        // prepare radio for receive
        const RX_MODE: lora_phy::mod_params::RxMode = lora_phy::mod_params::RxMode::DutyCycle(
            lora_phy::mod_params::DutyCycleParams {
                rx_time: embassy_time::Duration::from_secs(1).as_secs() as u32,
                sleep_time: 0,
             }
        );
        lora_radio.set_irq_params(Some(lora_phy::mod_params::RadioMode::Receive(RX_MODE))).await.unwrap();
        loop {
            debug!("{TAG} waiting for packets...");
            match lora_radio.do_rx(RX_MODE).await
            {
               Ok(_) => {
                    match lora_radio.await_irq().await {
                        Ok(_) => {
                            let mut packet: ReceivedLoRaPacket = ReceivedLoRaPacket::default();
                            // TODO check if there is a packet
                            // TODO collect the packet and it's PacketStatus

                            // must complete before the next packet is received, else traffic will be lost
                            self.handle_received_packet(packet);
                        }
                        Err(e) => {
                            debug!("{TAG} failed to receive packet: {:?}", e);
                            break;
                        }   
                    }
                }
                Err(e) => {
                    warn!("{TAG} failed to start receive: {:?}", e);
                    break;
                }
            }
        }
    }

    /// handle a received packet
    /// * **must** complete before the next packet is received, else traffic will be lost
    fn handle_received_packet(&mut self, packet: ReceivedLoRaPacket);

    fn has_transmit_packets(&mut self) -> bool;

    /// allows implements to dynamically scale tx power
    fn get_tx_power(&mut self, lora_config: &EnmeshLoRaConfig) -> i32 {
        // default to the protocol's configured tx power
        lora_config.modulation_config.tx_power_dbm
    }

    /// peek at the next packet to transmit (leaving it on the queue)
    fn tx_peek(&mut self) -> Option<ReceivedLoRaPacket>;

    /// pop the next packet to transmit (removing it from the queue)
    fn tx_pop(&mut self) -> Option<ReceivedLoRaPacket>;
}

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
            .await
        {
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
