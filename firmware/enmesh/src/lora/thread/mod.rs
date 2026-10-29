/// provide the common crates via re-export
use common::*;

/// provide logging primitives
use log::*;
const TAG: &str = "[LoRa Task]";

/// provide scheduling primitives
use crate::prelude::*;

mod meshcore;
/// provide protocol handlers
mod meshtastic;

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
    let _meshtastic_lorarf = meshtastic::MeshtasticLoraRf {};
    let _meshcore_lorarf = meshcore::MeshCoreLoraRf {};

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
            last_frequency = 0;

            Timer::after_secs(1).await;
            continue;
        }

        // determine if we need to calibrate radio
        let next_frequency = match protocol {
            Protocols::Meshtastic => meshtastic_lora_channel.frequency_in_hz,
            Protocols::MeshCore => meshcore_lora_channel.frequency_in_hz,
            Protocols::None => unreachable!(),
        };
        if last_frequency != next_frequency {
            // radio needs calibration
            match lora_radio.calibrate_image(next_frequency).await {
                Err(e) => {
                    error!("{TAG} failed to recalibrate radio: {:?}", e);
                    Timer::after_secs(1).await;
                    continue;
                }
                Ok(_) => last_frequency = next_frequency,
            }
        }

        // configure the modulation and packet configuration
        let lora_channel = match protocol {
            Protocols::Meshtastic => meshtastic_lora_channel,
            Protocols::MeshCore => meshcore_lora_channel,
            Protocols::None => unreachable!(),
        };
        match lora_radio.create_modulation_params(
            lora_channel.spreading_factor,
            lora_channel.bandwidth,
            lora_channel.coding_rate,
            lora_channel.frequency_in_hz,
        ) {
            Err(e) => {
                error!("{TAG} failed to create modulation parameters: {:?}", e);
                Timer::after_secs(1).await;
                continue;
            }
            Ok(modulation_params) => {
                match lora_radio.set_modulation_params(&modulation_params).await {
                    Err(e) => {
                        error!("{TAG} failed to configure modulation parameters: {:?}", e);
                        Timer::after_secs(1).await;
                        continue;
                    }
                    Ok(_) => {
                        // configure the LoRa packets
                        let packet_config = match protocol {
                            Protocols::Meshtastic => meshtastic::PACKET_CONFIG,
                            Protocols::MeshCore => meshcore::PACKET_CONFIG,
                            Protocols::None => unreachable!(),
                        };
                        match lora_radio.create_packet_params(
                            packet_config.preamble_length,
                            packet_config.implicit_header,
                            packet_config.max_payload_length,
                            packet_config.crc_on,
                            packet_config.iq_inverted,
                            &modulation_params,
                        ) {
                            Err(e) => {
                                error!("{TAG} failed to create packet parameters: {:?}", e);
                                Timer::after_secs(1).await;
                                continue;
                            }
                            Ok(packet_params) => {
                                match lora_radio.set_packet_params(&packet_params).await {
                                    Err(e) => {
                                        error!( "{TAG} failed to configure packet parameters: {:?}", e);
                                        Timer::after_secs(1).await;
                                        continue;
                                    }
                                    Ok(_) => {}
                                }
                            }
                        }
                    }
                }
            }
        };

        // perform TX and RX
        // const AIRTIME: Duration = Duration::from_secs(10);
        // let stop_time = Instant::now() + AIRTIME;
        // do one LoRa TX/RX cycle
        // match protocol {
        //     Protocols::Meshtastic => {
        //         // meshtastic_lorarf
        //         //     .do_cycle(global_state, &mut lora_radio, &meshtastic_lora_channel)
        //         //     .await
        //     }
        //     Protocols::MeshCore => {
        //         meshcore_lorarf
        //             .do_cycle(global_state, &mut lora_radio, &meshcore_lora_channel)
        //             .await
        //             .ok();
        //     }
        //     Protocols::None => unreachable!(),
        // }
    }
}

#[derive(PartialEq)]
enum Protocols {
    /// no protocols enabled
    None,
    Meshtastic,
    MeshCore,
}
impl Protocols {
    pub fn next(&self, meshtastic_enabled: bool, meshcore_enabled: bool) -> Self {
        match self {
            Self::None => {
                if meshtastic_enabled {
                    return Self::Meshtastic;
                } else if meshcore_enabled {
                    return Self::MeshCore;
                } else {
                    return Self::None;
                }
            }
            Self::MeshCore => {
                if meshtastic_enabled {
                    return Self::Meshtastic;
                } else if meshcore_enabled {
                    return Self::MeshCore;
                } else {
                    return Self::None;
                }
            }
            Self::Meshtastic => {
                if meshcore_enabled {
                    return Self::MeshCore;
                } else if meshtastic_enabled {
                    return Self::Meshtastic;
                } else {
                    return Self::None;
                }
            }
        }
    }
}
