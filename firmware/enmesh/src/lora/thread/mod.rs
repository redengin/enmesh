// provide the common crates via re-export
use common::*;

// provide logging primitives
use log::*;
const TAG: &str = "[LoRa Task]";

/// provide scheduling primitives
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::rwlock::RwLock;
use embassy_time::{Delay, Timer};

use crate::lora::LoRaRf;
use crate::state::LoRaProtocol;

mod meshtastic;
mod meshcore;


pub async fn run(
    global_state: &'static RwLock<NoopRawMutex, crate::State>,
    mut lora_radio: impl lora_phy::mod_traits::RadioKind,
) {
    debug!("{TAG} initializing radio...");
    let _ = lora_radio.reset(&mut Delay).await;
    match lora_radio.init_lora(true).await {
        Ok(_) => {
            debug!("{TAG} radio initialized successfully");
        }
        Err(e) => {
            error!("{TAG} failed to initialize radio: {:?}", e);
            return;
        }
    }

    // create our protocol handlers
    let mut meshtastic_handler = meshtastic::MeshtasticLoraRf {};
    let mut meshcore_handler = meshcore::MeshCoreLoraRf {};

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
                if meshtastic_enabled {
                    Some(LoRaProtocol::Meshtastic)
                } else if meshcore_enabled {
                    Some(LoRaProtocol::MeshCore)
                } else {
                    None
                }
            }

            Some(LoRaProtocol::Meshtastic) => {
                if meshcore_enabled {
                    Some(LoRaProtocol::MeshCore)
                } else if meshtastic_enabled {
                    Some(LoRaProtocol::Meshtastic)
                } else {
                    None
                }
            }

            Some(LoRaProtocol::MeshCore) => {
                if meshtastic_enabled {
                    Some(LoRaProtocol::Meshtastic)
                } else if meshcore_enabled {
                    Some(LoRaProtocol::MeshCore)
                } else {
                    None
                }
            }
        };
        let mut global_state_lock = global_state.write().await;
        global_state_lock.current_protocol = next_protocol;
        drop(global_state_lock);

        // if no protocols are enabled, simply wait and check again
        if next_protocol.is_none() {
            // put the radio to sleep
            let mut global_state_lock = global_state.write().await;
            global_state_lock.current_radio_mode = crate::state::LoRaRadioMode::Sleep;
            drop(global_state_lock);

            // TODO handle if warm start isn't possible
            const WARM_START_IF_POSSIBLE: bool = true;
            lora_radio.set_sleep(WARM_START_IF_POSSIBLE, &mut embassy_time::Delay).await.unwrap();

            Timer::after_secs(1).await;
            continue;
        }

        // get the LoRa config for the next protocol
        let global_state_lock = global_state.read().await;
        let lora_config =
            if next_protocol == Some(LoRaProtocol::Meshtastic) {
                global_state_lock.settings.meshtastic_settings.lora_config
            } else if next_protocol == Some(LoRaProtocol::MeshCore) {
                global_state_lock.settings.meshcore_settings.lora_config
            } else {
                unreachable!()
            };
        drop(global_state_lock);

        // configure the radio for the next protocol (if necessary)
        let next_frequency_hz = lora_config.modulation_config.frequency_hz;
        if next_frequency_hz != last_frequency_hz {
            match lora_radio.calibrate_image(next_frequency_hz).await {
                Ok(_) => {
                    debug!("{TAG} calibrated radio frequency to {next_frequency_hz}");
                    lora_radio.set_channel(next_frequency_hz).await.unwrap();
                    last_frequency_hz = next_frequency_hz;
                }
                Err(e) => {
                    warn!("{TAG} failed to calibrate radio for frequency {next_frequency_hz}: {:?}", e);
                    // skip this protocol and try again later
                    last_frequency_hz = 0;
                    continue;
                }
            }
        }

        // perform an TX/RX cycle
        match next_protocol {
            Some(LoRaProtocol::Meshtastic) => {
                meshtastic_handler.cycle(&mut lora_radio, &global_state, &lora_config).await;
            }
            Some(LoRaProtocol::MeshCore) => {
                meshcore_handler.cycle(&mut lora_radio, &global_state, &lora_config).await;
            }
            None => unreachable!(),
        }
    }
}
