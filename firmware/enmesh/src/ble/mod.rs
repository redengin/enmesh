// provide the common crates via re-export
use common::*;

// provide logging primitives
use log::*;
const TAG: &str = "[BLE Host]";

/// provide scheduling primitives
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::rwlock::RwLock;

// provide BLE primitives
// use trouble_host;
use trouble_host::prelude::DefaultPacketPool;

pub async fn run(
    _global_state: &'static RwLock<NoopRawMutex, crate::State>,
    ble_controller: impl trouble_host::Controller,
) {
    debug!("{TAG} starting...");

    const CONNECTIONS_MAX: usize = 1;
    const L2CAP_CHANNELS_MAX: usize = 1; // FIXME
    let mut ble_resources: trouble_host::HostResources<
        _,
        DefaultPacketPool,
        CONNECTIONS_MAX,
        L2CAP_CHANNELS_MAX,
    > = trouble_host::HostResources::new();

    let _ble_stack = trouble_host::new(ble_controller, &mut ble_resources);
}
