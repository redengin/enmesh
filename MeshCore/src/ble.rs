use trouble_host::prelude::*;

/// MeshCore Companion BLE protocol
/// 
/// * https://github.com/meshcore-dev/MeshCore/blob/main/docs/companion_protocol.md
/// * https://docs.meshcore.io/companion_protocol/
#[gatt_service(uuid = BluetoothUuid128::new(0x6E400001_B5A3_F393_E0A9_E50E24DCCA9E))]
pub struct MeshCoreService {
    // #[descriptor(uuid = descriptors::CLIENT_CHARACTERISTIC_CONFIGURATION, read, write)]
    #[characteristic(uuid = BluetoothUuid128::new(0x6E400002_B5A3_F393_E0A9_E50E24DCCA9E)
        , write, permissions(encrypted), notify)]
    pub rx: [u8;2], // FIXME

    // #[descriptor(uuid = descriptors::CLIENT_CHARACTERISTIC_CONFIGURATION, read, value = [0, 100])]
    #[characteristic(uuid = BluetoothUuid128::new(0x6E400003_B5A3_F393_E0A9_E50E24DCCA9E)
        , read, permissions(encrypted), notify)]
    pub tx: [u8;2], // FIXME
}

