use crate::lora::*;
const TAG: &str = "MeshCoreLoRa";

const MAX_PACKETS: usize = 255;
const MAX_PACKET_SIZE: usize = 255;
pub struct LoRaPacket {
    pub len : u8,
    buffer: [u8; MAX_PACKET_SIZE],
}

pub struct MeshCoreLora {
    pub lora_channel_config: EnmeshLoRaConfig,

    tx_queue: heapless::deque::Deque<LoRaPacket, MAX_PACKETS>,

    /// has lora radio been cold started?
    radio_has_cold_started: bool,
    /// has the lora radio been calibrated?
    radio_is_calibrated : bool,
}
impl MeshCoreLora {
    pub fn new(
        lora_channel_config: EnmeshLoRaConfig,
    ) -> Self
    {
        Self {
            lora_channel_config,
            tx_queue: heapless::Deque::new(),
            radio_has_cold_started: false,
            radio_is_calibrated: false,
        }
    }

}

impl LoRaProtocol for MeshCoreLora {

    fn get_lora_frequency_hz(&self) -> u32 {
        self.lora_channel_config.modulation_config.frequency_hz
    }

    async fn do_cycle(&mut self, lora_radio: &mut impl lora_phy::mod_traits::RadioKind) {

        // calibrate the radio
        // TODO only needs to be down if frequency changes - perhaps move to management class
        match lora_radio.calibrate_image(self.get_lora_frequency_hz()).await
        {
            Ok(_) => {},
            Err(err) => {
                error!("{TAG} unable to calibrate radio: {:?}", err);
                return;
            }
        }

        let modulation_params = lora_radio.create_modulation_params(
            self.lora_channel_config.modulation_config.spreading_factor,
            self.lora_channel_config.modulation_config.bandwidth,
            self.lora_channel_config.modulation_config.coding_rate,
            self.lora_channel_config.modulation_config.frequency_hz,
        ).unwrap();

        if self.tx_queue.is_empty() {
            // nothing to send, so just listen
            // self.listen(lora_radio);
        }
        else {
            // see if the channel is available for transmit
            lora_radio.do_cad(&modulation_params).await.unwrap();
            // TODO this API for cad activity is garbage
            let mut is_channel_active: bool = true;
            let lora_phy_cad: Option<&mut bool> = Some(&mut is_channel_active);
            lora_radio.process_irq_event(
                lora_phy::mod_params::RadioMode::ChannelActivityDetection,
                lora_phy_cad,
                true
            ).await.unwrap();

            if is_channel_active {
                // listen to the current transmitter
                // self.listen(lora_radio);
            }
            else {
                // send our packets
                let tx_start = embassy_time::Instant::now();

                // prepare for tx
                lora_radio.ensure_ready(lora_phy::mod_params::RadioMode::Transmit).await.unwrap();
                lora_radio.set_standby().await.unwrap();
                if !self.radio_has_cold_started {
                    lora_radio.init_lora(true).await.unwrap();
                    lora_radio.set_tx_power_and_ramp_time(0, None, false).await.unwrap();
                    lora_radio.set_irq_params(None).await.unwrap();
                    self.radio_has_cold_started = true;
                }
                if !self.radio_is_calibrated {
                    lora_radio.calibrate_image(self.lora_channel_config.modulation_config.frequency_hz).await.unwrap();
                    self.radio_is_calibrated = true;
                }
                lora_radio.set_modulation_params(&modulation_params).await.unwrap();
                // TODO set transmit power (dynamically scaled to meet recievers)
                const TX_POWER: i32 = 28;
                lora_radio.set_tx_power_and_ramp_time(
                    TX_POWER,
                    Some(&modulation_params),
                    true).await.unwrap();
                lora_radio.ensure_ready(lora_phy::mod_params::RadioMode::Transmit).await.unwrap();
                lora_radio.set_standby().await.unwrap();
                // FIXME lora_radio.set_packet_params(pkt_params);
                lora_radio.set_channel(self.lora_channel_config.modulation_config.frequency_hz).await.unwrap();
                // lora_radio.set_payload(buffer).await.unwrap();
                lora_radio.set_irq_params(Some(lora_phy::mod_params::RadioMode::Transmit)).await.unwrap();


                while self.tx_queue.len() > 0 {
                    let packet = self.tx_queue.get(0).unwrap();
                    // prepare for transmit
                    lora_radio.create_packet_params(
                        self.lora_channel_config.packet_config.preamble_length,
                        self.lora_channel_config.packet_config.implicit_header,
                        packet.len,
                        self.lora_channel_config.packet_config.crc,
                        self.lora_channel_config.packet_config.iq_inverted,
                        &modulation_params).unwrap();
                    
                    // transmit packet
                    match lora_radio.do_tx().await {
                        Ok(_) => { self.tx_queue.pop_front(); }
                        Err(err) => {
                            warn!("{TAG} failed to send packet: {:?}", err);
                            break;
                        }
                    }

                    // stop transmitting if we've exceeded airtime
                    if (embassy_time::Instant::now() - tx_start) >
                        self.lora_channel_config.modulation_config.air_time {
                            break;
                        }
                }
            }
        }
    }
}
