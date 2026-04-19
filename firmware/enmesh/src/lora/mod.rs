// provide the shared crates via re-export
use common::*;



pub async fn run<LoRaRk, LoRaDly>(lora_radio: lora_phy::LoRa<LoRaRk, LoRaDly>)
where
    LoRaRk: lora_phy::mod_traits::RadioKind,
    LoRaDly: lora_phy::DelayNs,
{
    loop {
        // TODO support UX choice of protocol

    } 
}