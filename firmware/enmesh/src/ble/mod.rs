// provide the common crates via re-export
use common::*;

// provide logging primitives
use log::*;
const TAG: &str = "[BLE Host]";

/// provide scheduling primitives
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::rwlock::RwLock;

// provide BLE primitives
use trouble_host;
use trouble_host::prelude::DefaultPacketPool;

pub async fn run(
    _global_state: &'static RwLock<NoopRawMutex, crate::State>,
    ble_controller: impl trouble_host::Controller,
) {
    debug!("{TAG} starting...");

    const CONNECTIONS_MAX: usize = 1;
    const L2CAP_CHANNELS_MAX: usize = 1; // FIXME
    let mut ble_resources: trouble_host::HostResources<
        DefaultPacketPool,
        CONNECTIONS_MAX,
        L2CAP_CHANNELS_MAX,
    > = trouble_host::HostResources::new();

    let ble_stack = trouble_host::new(ble_controller, &mut ble_resources);

}

// use trouble_host::prelude::*;
// #[gatt_server]
// struct EnmeshBle {
//     battery_service: BatteryService,
// }

// #[gatt_service(uuid = service::BATTERY)]
// struct BatteryService {
//     /// Battery Level
//     #[descriptor(uuid = descriptors::VALID_RANGE, read, value = [0, 100])]
//     #[descriptor(uuid = descriptors::MEASUREMENT_DESCRIPTION, name = "hello", read, value = "Battery Level", type = &'static str)]
//     #[characteristic(uuid = characteristic::BATTERY_LEVEL, read, notify, value = 10)]
//     level: u8,
//     #[characteristic(uuid = "408813df-5dd4-1f87-ec11-cdb001100000", write, read, notify)]
//     status: bool,
// }