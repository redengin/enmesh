// provide the common crates via re-export
use common::*;

// provide logging primitives
// use log::*;
// const TAG: &str = "[Serial Console]";

/// provide scheduling primitives
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::rwlock::RwLock;

use meshcore::cli::CliCommands;

const STRING_MAX: usize = 80;
pub(crate) async fn handle<'a>(
    global_state: &'static RwLock<NoopRawMutex, crate::State>,
    cli_command: meshcore::cli::CliCommands<'a>,
) -> Result<Option<heapless::String<STRING_MAX>>, &'a str> {
    return match cli_command {
        CliCommands::Reboot => {
            // TODO
            Err("not implemented")
        }

        CliCommands::PowerOff => {
            // TODO
            Err("not implemented")
        }

        CliCommands::ResetClockAndReboot => {
            // TODO
            Err("not implemented")
        }

        CliCommands::ClockSync => {
            // TODO
            Err("not implemented")
        }

        CliCommands::ShowClock => {
            // TODO
            Err("not implemented")
        }

        CliCommands::SetClock(_epoch_time) => {
            // TODO
            Err("not implemented")
        }

        CliCommands::SendFloodAdvert => {
            // TODO
            Err("not implemented")
        }

        CliCommands::SendZeroHopAdvert => {
            // TODO
            Err("not implemented")
        }

        CliCommands::StartOta => {
            // TODO
            Err("not implemented")
        }

        CliCommands::FactoryReset => {
            // TODO
            Err("not implemented")
        }

        CliCommands::ShowLastAdverts => {
            // TODO
            Err("not implemented")
        }

        CliCommands::RemoveNeighbor(_neihbor) => {
            // TODO
            Err("not implemented")
        }

        CliCommands::DiscoverZeroHopNeighbors => {
            // TODO
            Err("not implemented")
        }

        CliCommands::ClearStatistics => {
            // TODO
            Err("not implemented")
        }

        CliCommands::GetCoreStats => {
            // TODO
            Err("not implemented")
        }

        CliCommands::GetRadioStats => {
            // TODO
            Err("not implemented")
        }

        CliCommands::GetPacketStats => {
            // TODO
            Err("not implemented")
        }

        CliCommands::StartRxLog => {
            // TODO
            Err("not implemented")
        }

        CliCommands::EraseRxLog => {
            // TODO
            Err("not implemented")
        }

        CliCommands::GetRxLog => {
            // TODO
            Err("not implemented")
        }

        CliCommands::GetVersion => {
            let global_state_lock = global_state.read().await;
            let version = global_state_lock.firmware_version;
            drop(global_state_lock);

            let mut message: heapless::String<STRING_MAX> = heapless::String::new();
            message.push_str(version).unwrap();
            Ok(Some(message))
        }

        CliCommands::GetHardwareName => {
            let global_state_lock = global_state.read().await;
            let hardware_name = global_state_lock.hardware_name;
            drop(global_state_lock);

            let mut message: heapless::String<STRING_MAX> = heapless::String::new();
            message.push_str(hardware_name).unwrap();
            Ok(Some(message))
        }

        CliCommands::GetRadioConfig => {
            let global_state_lock = global_state.read().await;
            let modulation_config = global_state_lock
                .settings
                .meshcore_settings
                .lora_config
                .modulation_config;
            drop(global_state_lock);

            let message = heapless::format!(STRING_MAX;
                "{:3.3}, {}, {}, {}",
                ((modulation_config.frequency_hz as f32) / 1E6),
                modulation_config.bandwidth.hz(),
                modulation_config.spreading_factor.factor(),
                modulation_config.coding_rate.denom()
            )
            .unwrap();

            Ok(Some(message))
        }

        CliCommands::SetRadioConfig {
            freq,
            bw_khz,
            sf,
            cr,
        } => {
            let freq_hz = (freq * 1E6) as u32;
            // TODO validate freq_hz

            let bandwidth = match bw_khz {
                7 => lora_modulation::Bandwidth::_7KHz,
                10 => lora_modulation::Bandwidth::_10KHz,
                15 => lora_modulation::Bandwidth::_15KHz,
                20 => lora_modulation::Bandwidth::_20KHz,
                31 => lora_modulation::Bandwidth::_31KHz,
                41 => lora_modulation::Bandwidth::_41KHz,
                62 => lora_modulation::Bandwidth::_62KHz,
                125 => lora_modulation::Bandwidth::_125KHz,
                250 => lora_modulation::Bandwidth::_250KHz,
                500 => lora_modulation::Bandwidth::_500KHz,
                _ => return Err("uknown bandwidth setting '{bw_khz}'"),
            };
            let spreading_factor = match sf {
                5 => lora_modulation::SpreadingFactor::_5,
                6 => lora_modulation::SpreadingFactor::_6,
                7 => lora_modulation::SpreadingFactor::_7,
                8 => lora_modulation::SpreadingFactor::_8,
                9 => lora_modulation::SpreadingFactor::_9,
                10 => lora_modulation::SpreadingFactor::_10,
                11 => lora_modulation::SpreadingFactor::_11,
                12 => lora_modulation::SpreadingFactor::_12,
                _ => return Err("uknown spreading factor '{sf}'"),
            };
            let coding_rate = match cr {
                5 => lora_modulation::CodingRate::_4_5,
                6 => lora_modulation::CodingRate::_4_6,
                7 => lora_modulation::CodingRate::_4_7,
                8 => lora_modulation::CodingRate::_4_8,
                _ => return Err("uknown coding rate '{cr}'"),
            };

            let mut global_state_lock = global_state.write().await;
            global_state_lock
                .settings
                .meshcore_settings
                .lora_config
                .modulation_config
                .frequency_hz = freq_hz;
            global_state_lock
                .settings
                .meshcore_settings
                .lora_config
                .modulation_config
                .bandwidth = bandwidth;
            global_state_lock
                .settings
                .meshcore_settings
                .lora_config
                .modulation_config
                .spreading_factor = spreading_factor;
            global_state_lock
                .settings
                .meshcore_settings
                .lora_config
                .modulation_config
                .coding_rate = coding_rate;
            drop(global_state_lock);

            Ok(None)
        }

        CliCommands::GetTxPower => {
            let global_state_lock = global_state.read().await;
            let tx_power = global_state_lock
                .settings
                .meshcore_settings
                .lora_config
                .modulation_config
                .tx_power_dbm;
            drop(global_state_lock);

            let message = heapless::format!(STRING_MAX;
                "{tx_power}")
            .unwrap();
            Ok(Some(message))
        }

        CliCommands::SetTxPower(tx_power_dbm) => {
            let mut global_state_lock = global_state.write().await;
            global_state_lock
                .settings
                .meshcore_settings
                .lora_config
                .modulation_config
                .tx_power_dbm = tx_power_dbm as i32;
            drop(global_state_lock);

            Ok(None)
        }

        // CliCommands::SetTempRadioConfig { freq, bw, sf, cr, duration_minutes } =>
        CliCommands::SetTempRadioConfig { .. } => Err("not implemented"),

        CliCommands::GetFreq => {
            let global_state_lock = global_state.read().await;
            let frequency_hz = global_state_lock
                .settings
                .meshcore_settings
                .lora_config
                .modulation_config
                .frequency_hz;
            drop(global_state_lock);

            let message = heapless::format!(STRING_MAX;
                "{:3.3}", ((frequency_hz as f32) / 1E6)
            )
            .unwrap();
            Ok(Some(message))
        }

        CliCommands::GetRxGainStatus => {
            // rx gain always enabled
            let mut message: heapless::String<STRING_MAX> = heapless::String::new();
            message.push_str("enabled").unwrap();

            Ok(Some(message))
        }
        CliCommands::SetRxGain(enable) => {
            // rx gain always enabled
            match enable {
                true => Err("rxgain always enabled"),
                false => Ok(None)
            }
        }




        _ => Err("not implemented"),
    };
}
