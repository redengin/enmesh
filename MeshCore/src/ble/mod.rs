use trouble_host::prelude::*;
use trouble_host_embassy_sync as embassy_sync;
use trouble_host_static_cell as static_cell;

/// MeshCore Companion BLE protocol
///=============================================================================
/// * https://github.com/meshcore-dev/MeshCore/blob/main/docs/companion_protocol.md
/// * https://docs.meshcore.io/companion_protocol/
/// * https://github.com/ruuvi/docs/blob/master/communication/bluetooth-connection/nordic-uart-service-nus/README.md
pub const NORDIC_UART_SERVICE_UUID: BluetoothUuid128 =
    BluetoothUuid128::new(0x6E400001_B5A3_F393_E0A9_E50E24DCCA9E);
pub const NORDIC_UART_TX_CHARACTERISTIC_UUID: BluetoothUuid128 =
    BluetoothUuid128::new(0x6E400003_B5A3_F393_E0A9_E50E24DCCA9E);
pub const NORDIC_UART_RX_CHARACTERISTIC_UUID: BluetoothUuid128 =
    BluetoothUuid128::new(0x6E400002_B5A3_F393_E0A9_E50E24DCCA9E);

/// MeshCore Companion BLE protocol
#[gatt_service(uuid = NORDIC_UART_SERVICE_UUID)]
pub struct MeshCoreService {
    #[characteristic(uuid = NORDIC_UART_TX_CHARACTERISTIC_UUID,
        notify, read, permissions(encrypted), value = [0u8; 2])]
    /// https://github.com/espressif/arduino-esp32/blob/master/libraries/BLE/src/BLE2902.cpp#L61
    /// payload is a tuple (data byte, number of bytes remaining)
    pub tx: [u8; 2],

    #[characteristic(uuid = NORDIC_UART_RX_CHARACTERISTIC_UUID, write, permissions(encrypted))]
    /// https://github.com/espressif/arduino-esp32/blob/master/libraries/BLE/src/BLE2902.cpp#L61
    /// payload is a tuple (data byte, number of bytes remaining)
    pub rx: [u8; 2],
}

/// provide MeshCore BLE command serde
pub mod commands;
