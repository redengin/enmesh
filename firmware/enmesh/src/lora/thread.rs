// provide the common runtime primitive
use common::*;

/// provide scheduling primitives
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::rwlock::RwLock;
use embassy_time::Timer;

use crate::state::LoRaProtocol;

pub async fn run(
    global_state: &'static RwLock<NoopRawMutex, crate::State>,
    mut lora_radio: impl lora_phy::mod_traits::RadioKind,
)
{
    // round robin switch between enabled protocols
    let mut last_protocol: Option<crate::state::LoRaProtocol> = None;
    loop {
        let global_state_lock = global_state.read().await;
        let meshtastic_enabled = global_state_lock.settings.meshtastic_settings.enabled;    
        let meshcore_enabled = global_state_lock.settings.meshtastic_settings.enabled;
        let current_protocol = global_state_lock.current_protocol;
        drop(global_state_lock);

        // choose the next protocol
        let next_protocol = match current_protocol {
            None => {
                if meshtastic_enabled { Some(LoRaProtocol::Meshtastic) }
                else if meshcore_enabled { Some(LoRaProtocol::MeshCore) }
                else { None }
            },

            Some(LoRaProtocol::Meshtastic) => {
                if meshcore_enabled { Some(LoRaProtocol::MeshCore) }
                else if meshtastic_enabled { Some(LoRaProtocol::Meshtastic) }
                else { None }
            },

            Some(LoRaProtocol::MeshCore) => {
                if meshtastic_enabled { Some(LoRaProtocol::Meshtastic) }
                else if meshcore_enabled { Some(LoRaProtocol::MeshCore) }
                else { None }
            },
        };
        let mut global_state_lock = global_state.write().await;
        global_state_lock.current_protocol = next_protocol;
        drop(global_state_lock);

        // configure the radio for the next protocol
        configure_lora_frequency(&mut lora_radio).await;


    }
}


async fn configure_lora_frequency(
    lora_radio: &mut impl lora_phy::mod_traits::RadioKind,
)
    -> Result<(), lora_phy::mod_params::RadioError>
{
    lora_radio.calibrate_image(100).await
}


// use crate::{prelude::*, state::LoRaProtocol};

// /// round-robin switching between enabled protocols
// pub async fn run<LoRaRk, LoRaDly>(
//     global_state: &'static RwLock<NoopRawMutex, crate::State>,
//     lora_radio: lora_phy::LoRa<LoRaRk, LoRaDly>,
// ) where
//     LoRaRk: lora_phy::mod_traits::RadioKind,
//     LoRaDly: lora_phy::DelayNs,
// {
//     loop {
//         // simply wait if no protocols are enabled
//         let meshtastic_enabled = global_state.read().await.settings.meshtastic_settings.enabled;
//         let meshcore_enabled = global_state.read().await.settings.meshcore_settings.enabled;
//         if !meshtastic_enabled && !meshcore_enabled {
//             Timer::after_secs(1).await;
//             continue
//         }

//         let current_protocol = global_state.read().await.current_protocol;
//         match current_protocol {
//             LoRaProtocol::Meshtastic => {
//                 if meshtastic_enabled {
//                     // TODO perform an RX/TX cycle
//                 }
//             },
//             LoRaProtocol::MeshCore => {
//                 if meshcore_enabled {
//                     // TODO perform an RX/TX cycle
//                 }
//             },
//         }

//         // switch to the next protocol
//         global_state.write().await.current_protocol.next();
//     }
// }
