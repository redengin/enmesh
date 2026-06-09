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
    // #[descriptor(uuid = NORDIC_UART_TX_CHARACTERISTIC_UUID, read)]
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

enum MeshCoreBleCommands<'a> {
    AppStart {
        reserved: &'a [u8],
        application_name: &'a [u8],
    },
    DeviceQuery,
    GetChannelInfo {
        channel_index: &'a u8,
    },
    SetChannel {
        channel_index: &'a u8,
        channel_name: &'a [u8],
        secret: &'a [u8],
    },
}
impl<'a> MeshCoreBleCommands<'a> {
    pub fn from_bytes(buffer: &'a [u8]) -> Option<Self> {
        return match buffer[0] {
            0x01 => Some(Self::AppStart {
                reserved: &buffer[1..8],
                application_name: &buffer[8..buffer.len()],
            }),
            0x16 => {
                if buffer[1] == 0x03 {
                    return Some(Self::DeviceQuery);
                }
                None
            }
            0x1F => Some(Self::GetChannelInfo {
                channel_index: &buffer[1],
            }),
            0x20 => Some(Self::SetChannel {
                channel_index: &buffer[1],
                channel_name: &buffer[2..34],
                secret: &buffer[34..50],
            }),

            _ => None,
        };
    }

    pub fn to_bytes(&self, buffer: &mut [u8]) -> Result<usize, &str> {
        return match self {
            MeshCoreBleCommands::AppStart {
                reserved,
                application_name,
            } => {
                buffer[0] = 0x01;
                buffer[1..7].copy_from_slice(reserved);
                buffer[8..(8 + application_name.len())].copy_from_slice(application_name);
                Ok(8 + application_name.len())
            }

            MeshCoreBleCommands::DeviceQuery => {
                buffer[0] = 0x16;
                buffer[1] = 0x03;
                Ok(2)
            }

            MeshCoreBleCommands::GetChannelInfo { channel_index } => {
                buffer[0] = 0x1F;
                buffer[1] = **channel_index;
                Ok(2)
            }

            MeshCoreBleCommands::SetChannel {
                channel_index,
                channel_name,
                secret,
            } => {
                buffer[0] = 0x20;
                buffer[1] = **channel_index;
                buffer[2..(2 + channel_name.len())].copy_from_slice(channel_name);
                buffer[34..50].copy_from_slice(secret);
                Ok(50)
            }
        };
    }
}

// TESTING
//--------------------------------------------------------------------------------
#[cfg(test)]
mod tests {

    use trouble_host::gatt::GattConnectionEvent::PassKeyInput;

    use super::*;

    #[test]
    fn serde_MeshCoreBleCommands() {
        /// https://github.com/meshcore-dev/MeshCore/blob/main/docs/companion_protocol.md#mtu-maximum-transmission-unit
        const MAX_PACKET_SIZE: usize = 50;
        {
            // APP_START
            // https://github.com/meshcore-dev/MeshCore/blob/main/docs/companion_protocol.md#1-app-start
            let example_data: [u8; _] = [
                0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x6d, 0x63, 0x63, 0x6c, 0x69,
            ];

            // test deserialization
            if let Some(command) = MeshCoreBleCommands::from_bytes(&example_data) {
                match command {
                    MeshCoreBleCommands::AppStart {
                        reserved,
                        application_name,
                    } => {
                        assert_eq!(&example_data[1..8], reserved);
                        assert_eq!(&example_data[8..], application_name);
                    }
                    _ => panic!("should have found an AppStart"),
                }
            } else {
                panic!("failed to deserialize example test vector");
            }

            // test serialization
            let app_start = MeshCoreBleCommands::AppStart {
                reserved: &example_data[1..7],
                application_name: &example_data[8..],
            };
            let mut buffer = [0u8; MAX_PACKET_SIZE];
            match app_start.to_bytes(&mut buffer) {
                Ok(used_bytes) => {
                    assert_eq!(example_data.len(), used_bytes);
                    assert_eq!(example_data, buffer[..used_bytes]);
                }
                Err(e) => panic!("failed serialization of APP_START: {e}"),
            }
        }
        {
            // DEVICE_QUERY
            // https://github.com/meshcore-dev/MeshCore/blob/main/docs/companion_protocol.md#2-device-query
            let example_data: [u8; _] = [0x16, 0x03];

            // test deserialization
            if let Some(command) = MeshCoreBleCommands::from_bytes(&example_data) {
                match command {
                    MeshCoreBleCommands::DeviceQuery => { /* ok */ }
                    _ => panic!("should have found an DeviceQuery"),
                }
            }

            // test serialization
            let device_query = MeshCoreBleCommands::DeviceQuery;
            let mut buffer = [0u8; MAX_PACKET_SIZE];
            match device_query.to_bytes(&mut buffer) {
                Ok(used_bytes) => {
                    assert_eq!(example_data.len(), used_bytes);
                    assert_eq!(example_data, buffer[..used_bytes]);
                }
                Err(e) => panic!("failed serialization of DEVICE_QUERY: {e}"),
            }
        }
        {
            // GET_CHANNEL_INFO
            // https://github.com/meshcore-dev/MeshCore/blob/main/docs/companion_protocol.md#3-get-channel-info
            let example_data: [u8; _] = [0x1F, 0x01];

            // test deserialization
            if let Some(command) = MeshCoreBleCommands::from_bytes(&example_data) {
                match command {
                    MeshCoreBleCommands::GetChannelInfo { channel_index } => {
                        assert_eq!(example_data[1], *channel_index);
                    }
                    _ => panic!("should have found an GET_CHANNEL_INFO"),
                }
            }

            // test serialization
            let get_channel_info = MeshCoreBleCommands::GetChannelInfo { channel_index: &1 };
            let mut buffer = [0u8; MAX_PACKET_SIZE];
            match get_channel_info.to_bytes(&mut buffer) {
                Ok(used_bytes) => {
                    assert_eq!(example_data.len(), used_bytes);
                    assert_eq!(example_data, buffer[..used_bytes]);
                }
                Err(e) => panic!("failed serialization of GET_CHANNEL_INFO: {e}"),
            }
        }
        {
            // SET_CHANNEL
            // https://github.com/meshcore-dev/MeshCore/blob/main/docs/companion_protocol.md#4-set-channel
            let mut example_data: [u8; 50] = [0u8; 50];
            example_data[0] = 0x20;
            example_data[1] = 0x01;
            const NAME: [u8; 4] = [0x53, 0x4D, 0x53, 0x00];
            example_data[2..(2 + NAME.len())].copy_from_slice(&NAME);
            const SECRET: [u8; 16] = u128::MAX.to_le_bytes();
            example_data[34..].copy_from_slice(&SECRET);

            // test deserialization
            if let Some(command) = MeshCoreBleCommands::from_bytes(&example_data) {
                match command {
                    MeshCoreBleCommands::SetChannel {
                        channel_index,
                        channel_name,
                        secret,
                    } => {
                        assert_eq!(example_data[1], *channel_index);
                        assert_eq!(
                            example_data[2..(2 + NAME.len())],
                            channel_name[..NAME.len()]
                        );
                        assert_eq!(SECRET, secret);
                    }
                    _ => panic!("should have found an SET_CHANNEL"),
                }
            }

            // test serialization
            let set_channel = MeshCoreBleCommands::SetChannel {
                channel_index: &1,
                channel_name: &NAME,
                secret: &SECRET,
            };
            let mut buffer = [0u8; MAX_PACKET_SIZE];
            match set_channel.to_bytes(&mut buffer) {
                Ok(used_bytes) => {
                    assert_eq!(example_data.len(), used_bytes);
                    assert_eq!(example_data, buffer[..used_bytes]);
                }
                Err(e) => panic!("failed serialization of SET_CHANNEL: {e}"),
            }
        }
    }
}
