// trouble-host currently doesn't support latest cargo,
// so we create a separate module to provide the specific cargo versions
// necessary to support trouble-host

// provide the common crates via re-export
use common::*;

// provide BLE primitives
use common::trouble_host::prelude::*;
// provide an embassy-sync that supports trouble-host
use common::trouble_host_embassy_sync as embassy_sync;

/// our BLE server
#[gatt_server]
pub(crate) struct Server {
    /// support for meshcore companion BLE
    pub(crate) meshcore_service: MeshCoreService,
}

/// MeshCore Companion BLE protocol
#[gatt_service(uuid = ::meshcore::ble::NORDIC_UART_SERVICE_UUID)]
pub struct MeshCoreService {
    #[characteristic(uuid = ::meshcore::ble::NORDIC_UART_TX_CHARACTERISTIC_UUID,
        notify, read, permissions(encrypted))]
    /// https://github.com/espressif/arduino-esp32/blob/master/libraries/BLE/src/BLE2902.cpp#L61
    /// payload is a tuple (data, number of bytes not tranmitted))
    pub tx: [u8; 2],

    // #[descriptor(uuid = descriptors::CLIENT_CHARACTERISTIC_CONFIGURATION, read, value = [0, 100])]
    #[characteristic(uuid = ::meshcore::ble::NORDIC_UART_RX_CHARACTERISTIC_UUID, write, permissions(encrypted))]
    /// https://github.com/espressif/arduino-esp32/blob/master/libraries/BLE/src/BLE2902.cpp#L61
    /// payload is a tuple (data, number of bytes not tranmitted))
    pub rx: [u8; 2],
}