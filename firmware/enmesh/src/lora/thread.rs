// provide the common runtime primitive
use common::*;

// provide logging primitives
use log::*;

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
    let mut last_frequency_hz: u32 = 0;
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

        // if no protocols are enabled, simply wait and check again
        if next_protocol.is_none() {
            Timer::after_secs(1).await;
            continue
        }   

        // configure the radio for the next protocol (if necessary)
        let global_state_lock = global_state.read().await;
        let next_frequency_hz =
            if next_protocol == Some(LoRaProtocol::Meshtastic) {
                global_state_lock.settings.meshtastic_settings.lora_config.modulation_config.frequency_hz
            }
            else if next_protocol == Some(LoRaProtocol::MeshCore) {
                global_state_lock.settings.meshcore_settings.lora_config.modulation_config.frequency_hz
            }
            else {
                panic!("unreachable")
            };
        drop(global_state_lock);
        if next_frequency_hz != last_frequency_hz {
            match lora_radio.calibrate_image(next_frequency_hz).await
            {
                Ok(_) => { last_frequency_hz = next_frequency_hz; },
                Err(e) => {
                    info!("failed to calibrate radio for frequency {next_frequency_hz}: {:?}", e);
                    continue;
                }
            }
        } 


    }
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
