// trouble-host currently doesn't support latest cargo,
// so we create a separate module to provide the specific cargo
// versions necessary to support trouble-host

// provide the common crates via re-export
use common::*;

// provide BLE primitives
use common::trouble_host::prelude::*;
// provide an embassy-sync that supports trouble-host
use common::trouble_host_embassy_sync as embassy_sync;

// provide MeshCore BLE support
use ::meshcore::ble::MeshCoreService;

/// our BLE server
#[gatt_server]
pub(crate) struct Server {
    /// support for meshcore companion BLE
    pub(crate) meshcore_service: MeshCoreService,
}