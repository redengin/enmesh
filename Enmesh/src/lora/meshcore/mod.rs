use crate::lora::*;
const TAG: &str = "MeshCoreLoRa";

const MAX_PACKETS: usize = 255;
const MAX_PACKET_SIZE: usize = 255;
pub struct LoRaPacket {
    len : usize,
    buffer: [u8; MAX_PACKET_SIZE],
}

pub struct MeshCoreLora {
    pub lora_channel_config: EnmeshLoRaChannelConfig,

    tx_queue: heapless::deque::Deque<LoRaPacket, MAX_PACKETS>,
}
impl MeshCoreLora {
    pub fn new(
        lora_channel_config: EnmeshLoRaChannelConfig,
    ) -> Self
    {
        Self {
            lora_channel_config,
            tx_queue: heapless::Deque::new(),
        }
    }

    pub fn listen(&mut self, _lora_radio: &mut impl lora_phy::mod_traits::RadioKind) {
        // FIXME implement
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
            self.listen(lora_radio);
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
                self.listen(lora_radio);
            }
            else {
                // send our packets

                // TODO set transmit power (dynamically scaled to meet recievers)

                // prepare for transmit
                lora_radio.create_packet_params(
                    self.lora_channel_config.packet_config.preamble_length,
                    self.lora_channel_config.packet_config.implicit_header,
                    self.lora_channel_config.packet_config.max_payload_length,
                    self.lora_channel_config.packet_config.crc,
                    self.lora_channel_config.packet_config.iq_inverted,
                    &modulation_params).unwrap();
                
                // transmit packets
                // FIXME lora_radio.do_tx().await;
            }
        }
    }
}
