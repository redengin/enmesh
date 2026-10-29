
pub struct MeshtasticLoraRf {

}

use crate::lora::LoRaPacketConfig;
pub const PACKET_CONFIG: LoRaPacketConfig = LoRaPacketConfig {
    preamble_length: 16,
    implicit_header: true,
    max_payload_length: 255,
    crc_on: true,
    iq_inverted: false,
};
// pub const LORA_SYNC_WORD:u8 = 0x2B;


impl crate::lora::LoRaRf for MeshtasticLoraRf {

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