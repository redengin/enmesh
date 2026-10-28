/// provide the common crates via re-export
use common::*;

/// provide logging primitives
use log::*;
const TAG: &str = "[LoRa Task]";

/// provide scheduling primitives
use crate::prelude::*;

use crate::lora::LoRaRf;
// mod meshtastic;
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
    // let mut meshtastic_lorarf = meshtastic::MeshtasticLoraRf {};
    let mut meshcore_lorarf = meshcore::MeshCoreLoraRf {};

    // round robin service enabled protocols
    let mut protocol = Protocols::None;
    let mut last_frequency: u32 = 0;
    loop {
        // gather settings for protocols
        let global_state_lock = global_state.read().await;
        let meshtastic_enabled = global_state_lock.settings.meshtastic_settings.enabled;
        let meshtastic_lora_channel = global_state_lock.settings.meshtastic_settings.lora_channel;
        let meshcore_enabled = global_state_lock.settings.meshtastic_settings.enabled;
        let meshcore_lora_channel = global_state_lock.settings.meshcore_settings.lora_channel;
        drop(global_state_lock);

        // choose next protocol
        protocol = protocol.next(meshtastic_enabled, meshcore_enabled);

        if protocol == Protocols::None {
            // no protocols enabled, put the radio to sleep and wait
            let mut global_state_lock = global_state.write().await;
            global_state_lock.current_radio_mode = crate::state::LoRaRadioMode::Sleep;
            drop(global_state_lock);
            const WARM_START_IF_POSSIBLE: bool = true;
            lora_radio
                .set_sleep(WARM_START_IF_POSSIBLE, &mut embassy_time::Delay)
                .await
                .unwrap();

            Timer::after_secs(1).await;
            continue;
        } else {
            // determine if we need to calibrate radio
            let next_frequency = match protocol {
                Protocols::Meshstastic => meshtastic_lora_channel.frequency_hz,
                Protocols::MeshCore => meshcore_lora_channel.frequency_hz,
                Protocols::None => unreachable!(),
            };
            if last_frequency != next_frequency {
                // radio needs calibration
                lora_radio.calibrate_image(next_frequency).await.unwrap();
                last_frequency = next_frequency;
            }

            // do one LoRa TX/RX cycle
            match protocol {
                Protocols::Meshstastic => {
                    // meshtastic_lorarf
                    //     .do_cycle(global_state, &mut lora_radio, &meshtastic_lora_channel)
                    //     .await
                }
                Protocols::MeshCore => {
                    meshcore_lorarf
                        .do_cycle(global_state, &mut lora_radio, &meshcore_lora_channel)
                        .await.ok();
                }
                Protocols::None => unreachable!(),
            }
        }
    }
}

#[derive(PartialEq)]
enum Protocols {
    /// no protocols enabled
    None,
    Meshstastic,
    MeshCore,
}
impl Protocols {
    pub fn next(&self, meshtastic_enabled: bool, meshcore_enabled: bool) -> Self {
        match self {
            Self::None => {
                if meshtastic_enabled {
                    return Self::Meshstastic;
                } else if meshcore_enabled {
                    return Self::MeshCore;
                } else {
                    return Self::None;
                }
            }
            Self::MeshCore => {
                if meshtastic_enabled {
                    return Self::Meshstastic;
                } else if meshcore_enabled {
                    return Self::MeshCore;
                } else {
                    return Self::None;
                }
            }
            Self::Meshstastic => {
                if meshcore_enabled {
                    return Self::MeshCore;
                } else if meshtastic_enabled {
                    return Self::Meshstastic;
                } else {
                    return Self::None;
                }
            }
        }
    }
}
