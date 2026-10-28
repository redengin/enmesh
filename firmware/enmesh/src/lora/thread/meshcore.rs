/// provide the shared crates via re-export
use common::*;

/// provide enmesh firmware primitives
use crate::prelude::*;

pub struct MeshCoreLoraRf {}

impl crate::lora::LoRaRf for MeshCoreLoraRf {
    async fn do_cycle(
        &mut self,
        _global_state: &'static RwLock<NoopRawMutex, crate::State>,
        lora_radio: &mut impl lora_phy::mod_traits::RadioKind,
        lora_channel: &crate::lora::LoRaChannel,
    ) -> Result<(), lora_phy::mod_params::RadioError> {
        // prepare radio
        let packet_config = crate::lora::LoRaPacketConfig {
            // FIXME configure per MeshCore
            preamble_length: 1,
            implicit_header: false,
            max_payload_length: 120,
            crc: true,
            iq_inverted: false
        };
        crate::lora::prepare_radio(lora_radio, lora_channel, &packet_config).await?;

        // TODO try sending if tx_queue

        // TODO listen for broadcasts

        Ok(())
    }

    // fn handle_received_packet(&mut self, _packet: crate::lora::ReceivedLoRaPacket) {
    //     // TODO
    // }

    // fn has_transmit_packets(&mut self) -> bool {
    //     // TODO
    //     false
    // }

    // fn tx_peek(&mut self) -> Option<crate::lora::ReceivedLoRaPacket> {
    //     // TODO
    //     None
    // }

    // fn tx_pop(&mut self) -> Option<crate::lora::ReceivedLoRaPacket> {
    //     // TODO
    //     None
    // }
}
