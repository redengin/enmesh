
pub struct MeshCoreLoraRf {

}

impl crate::lora::LoRaRf for MeshCoreLoraRf {
    fn handle_received_packet(&mut self, _packet: crate::lora::ReceivedLoRaPacket) {
        // TODO
    }

    fn has_transmit_packets(&mut self) -> bool {
        // TODO
        false
    }

    fn tx_peek(&mut self) -> Option<crate::lora::ReceivedLoRaPacket> {
        // TODO
        None
    }

    fn tx_pop(&mut self) -> Option<crate::lora::ReceivedLoRaPacket> {
        // TODO
        None
    }
}