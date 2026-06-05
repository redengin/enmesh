use trouble_host::prelude::*;

/// MeshCore Companion BLE protocol
///
/// * https://github.com/meshcore-dev/MeshCore/blob/main/docs/companion_protocol.md
/// * https://docs.meshcore.io/companion_protocol/
/// * https://github.com/ruuvi/docs/blob/master/communication/bluetooth-connection/nordic-uart-service-nus/README.md
pub const NORDIC_UART_SERVICE_UUID: BluetoothUuid128 =
    BluetoothUuid128::new(0x6E400001_B5A3_F393_E0A9_E50E24DCCA9E);
pub const NORDIC_UART_RX_CHARACTERISTIC_UUID: BluetoothUuid128 =
    BluetoothUuid128::new(0x6E400002_B5A3_F393_E0A9_E50E24DCCA9E);
pub const NORDIC_UART_TX_CHARACTERISTIC_UUID: BluetoothUuid128 =
    BluetoothUuid128::new(0x6E400003_B5A3_F393_E0A9_E50E24DCCA9E);
