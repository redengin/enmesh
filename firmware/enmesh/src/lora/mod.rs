// provide the shared crates via re-export
use common::*;

// provide logging primitives
use log::*;

use serde::ser::SerializeStruct;
// provide the serialization traits
use serde::{Deserialize, Serialize};

/// publish rx-tx thread
pub mod thread;

#[derive(Default, Copy, Clone, Serialize, Deserialize)]
pub struct EnmeshLoRaConfig {
    pub modulation_config: EnmeshLoRaModulationConfig,
    pub packet_config: EnmeshLoRaPacketConfig,
}

/// used to configure the LoRa radio modulation
#[derive(Copy, Clone, Serialize, Deserialize)]
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
    pub air_time_millis: u16,
}
impl Default for EnmeshLoRaModulationConfig {
    fn default() -> Self {
        Self {
            frequency_hz: 0,
            bandwidth: lora_modulation::Bandwidth::_250KHz,
            spreading_factor: lora_modulation::SpreadingFactor::_6,
            coding_rate: lora_modulation::CodingRate::_4_5,
            tx_power_dbm: 0,
            air_time_millis: 1000,
        }
    }
}

/// used to configure the LoRa packet recognition
#[derive(Default, Copy, Clone, Serialize, Deserialize)]
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

pub struct ReceivedLoRaPacket {
    /// record the signal quality (rssi/snr) to influence transmit power
    pub rssi: i16,
    pub snr: i16,
    /// packet size (in bytes) stored in buffer
    pub length: usize,
    pub buffer: [u8; 255],
}

use embassy_sync::blocking_mutex::raw::NoopRawMutex;
/// provide scheduling primitives
use embassy_sync::rwlock::RwLock;

/// logging tag
const TAG: &str = "LoRaRf";
pub trait LoRaRf {
    #![allow(async_fn_in_trait)]

    /// default implementation should be sufficient
    async fn cycle(
        &mut self,
        lora_radio: &mut impl lora_phy::mod_traits::RadioKind,
        global_state: &'static RwLock<NoopRawMutex, crate::State>,
        lora_config: &EnmeshLoRaConfig,
    ) {
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
            lora_radio
                .set_irq_params(Some(lora_phy::mod_params::RadioMode::Transmit))
                .await
                .unwrap();
            // make sure the channel is clear before transmitting
            match crate::lora::is_channel_clear(lora_radio, &modulation_params).await {
                Ok(is_clear) => {
                    if is_clear {
                        // transmit packets
                        let mut global_state_lock = global_state.write().await;
                        global_state_lock.current_radio_mode =
                            crate::state::LoRaRadioMode::Transmit;
                        drop(global_state_lock);

                        self.do_tx(lora_radio, lora_config).await;
                    }
                }
                Err(e) => {
                    warn!("{TAG} failed to detect channel activity (CAD): {:?}", e);
                    return;
                }
            }
        }

        // receive packets
        let mut global_state_lock = global_state.write().await;
        global_state_lock.current_radio_mode = crate::state::LoRaRadioMode::Receive;
        drop(global_state_lock);
        self.do_rx(lora_radio, &packet_params).await;

        // radio ends in standby mode
        let mut global_state_lock = global_state.write().await;
        global_state_lock.current_radio_mode = crate::state::LoRaRadioMode::Standby;
        drop(global_state_lock);
    }

    /// default implementation should be sufficient
    async fn do_tx(
        &mut self,
        lora_radio: &mut impl lora_phy::mod_traits::RadioKind,
        lora_config: &EnmeshLoRaConfig,
    ) {
        // transmit packets until airtime expires
        let tx_stop_time = embassy_time::Instant::now() +
            embassy_time::Duration::from_millis(lora_config.modulation_config.air_time_millis as u64);

        // send packets
        while self.has_transmit_packets() {
            let packet = self.tx_peek().unwrap();
            lora_radio
                .set_payload(&packet.buffer[0..packet.length])
                .await
                .unwrap();
            match lora_radio.do_tx().await {
                Ok(_) => {
                    debug!("{TAG} transmitted packet [size: {}", packet.length);
                    // take the packet off the queue
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

    /// default implementation should be sufficient
    async fn do_rx(
        &mut self,
        lora_radio: &mut impl lora_phy::mod_traits::RadioKind,
        packet_params: &lora_phy::mod_params::PacketParams,
    ) {
        // receive until radio timeout
        const RX_TIMEOUT: u32 = embassy_time::Duration::from_secs(1).as_secs() as u32;
        const RX_MODE: lora_phy::mod_params::RxMode =
            lora_phy::mod_params::RxMode::DutyCycle(lora_phy::mod_params::DutyCycleParams {
                rx_time: RX_TIMEOUT,
                sleep_time: 0,
            });
        // prepare radio for receive
        lora_radio
            .set_irq_params(Some(lora_phy::mod_params::RadioMode::Receive(RX_MODE)))
            .await
            .unwrap();
        loop {
            debug!("{TAG} waiting for packets...");
            match lora_radio.do_rx(RX_MODE).await {
                Ok(_) => {
                    match lora_radio.await_irq().await {
                        Ok(_) => {
                            match lora_radio
                                .process_irq_event(
                                    lora_phy::mod_params::RadioMode::Receive(RX_MODE),
                                    None,
                                    true,
                                )
                                .await
                            {
                                Ok(irq_state) => {
                                    match irq_state {
                                        Some(lora_phy::mod_traits::IrqState::Done) => {
                                            // get the packet signal quality
                                            let packet_status =
                                                lora_radio.get_rx_packet_status().await.unwrap();
                                            let mut packet: ReceivedLoRaPacket =
                                                ReceivedLoRaPacket {
                                                    rssi: packet_status.rssi,
                                                    snr: packet_status.snr,
                                                    length: 0,
                                                    buffer: [0; 255],
                                                };
                                            // get the payload
                                            match lora_radio
                                                .get_rx_payload(&packet_params, &mut packet.buffer)
                                                .await
                                            {
                                                Ok(buffer_length) => {
                                                    packet.length = buffer_length as usize;
                                                    // must complete before the next packet is received, else traffic will be lost
                                                    self.handle_received_packet(packet);
                                                }
                                                Err(e) => {
                                                    warn!(
                                                        "{TAG} failed to get rx payload (aborting): {:?}",
                                                        e
                                                    );
                                                    break;
                                                }
                                            }
                                        }
                                        // ignore other irq states
                                        _ => continue,
                                    }
                                }
                                Err(e) => {
                                    warn!(
                                        "{TAG} failed to process IRQ event for receive (aborting): {:?}",
                                        e
                                    );
                                    break;
                                }
                            }
                        }
                        Err(e) => {
                            debug!("{TAG} failed to await IRQ for receive (aborting): {:?}", e);
                            break;
                        }
                    }
                }
                Err(e) => {
                    warn!("{TAG} failed to start receive (aborting): {:?}", e);
                    break;
                }
            }
        }
    }

    /// handle a received packet
    /// * **must** complete before the next packet is received, else traffic will be lost
    fn handle_received_packet(&mut self, packet: ReceivedLoRaPacket);

    fn has_transmit_packets(&mut self) -> bool;

    /// allows implementions to dynamically scale tx power
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

/// set the transmit power, reducing if the radio rejects the requested power level
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
