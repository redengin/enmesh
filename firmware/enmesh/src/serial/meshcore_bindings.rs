// provide the common crates via re-export
use common::*;

// provide logging primitives
// use log::*;
// const TAG: &str = "[Serial Console]";

/// provide scheduling primitives
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::rwlock::RwLock;

use meshcore::cli::CliCommands;

pub(crate) async fn handle<'a>(
    global_state: &'static RwLock<NoopRawMutex, crate::State>,
    cli_command: meshcore::cli::CliCommands<'a>,
) -> Result<Option<&'a str>, &'a str>
{
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

        CliCommands::RemoveNeighbor(_neihbor)=> {
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

            Ok(Some(version))
        }

        CliCommands::GetHardwareName => {
            let global_state_lock = global_state.read().await;
            let hardware_name = global_state_lock.hardware_name;
            drop(global_state_lock);

            Ok(Some(hardware_name))
        }





        _ => Err("not implemented")
    }
}