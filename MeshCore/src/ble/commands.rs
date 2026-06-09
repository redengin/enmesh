/// MeshCore Companion BLE protocol
///=============================================================================
/// * https://github.com/meshcore-dev/MeshCore/blob/main/docs/companion_protocol.md
/// * https://docs.meshcore.io/companion_protocol/
pub enum MeshCoreBleCommands<'a> {
    AppStart {
        reserved: &'a [u8],
        application_name: &'a [u8],
    },
    DeviceQuery,
    GetChannelInfo {
        channel_index: &'a u8,
    },
    /// FIXME - this command is insecure and inefficient
    /// * places private secrets onto remote devices, allowing those secrets to be
    ///     exposed by a malicious actor
    /// * begets implementations that require the remote device to encrypt/decrypt
    ///     * this is inefficient as the companion sender has greater compute
    ///         resources available to perform the encryption/decryption
    /// 
    SetChannel {
        channel_index: &'a u8,
        channel_name: &'a [u8],
        secret: &'a [u8],
    },
    // FIXME - this command is insecure and ineffecient
    // * see above comments on SetChannel for details
    // SendChannelMessage {
    //     channel_index: &'a u8,
    //     timestamp: &'a [u8],
    //     message: &'a [u8],
    // }
    // FIXME - not even sure what this is to be, but it's security/efficiency is questionable
    // * see above comments on SetChannel for details
    // SendChannelDataDatagram {
    //     channel_index: &'a u8,
    //     path_length: &'a u8,
    //     path: &'a [u8],
    //     type: &'a [u8],
    //     payload: &'a [u8],
    // }
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

    use super::*;

    #[test]
    #[allow(non_snake_case)]
    fn serde_MeshCoreBleCommands() {
        /// https://github.com/meshcore-dev/MeshCore/blob/main/docs/companion_protocol.md#mtu-maximum-transmission-unit
        const MAX_PACKET_SIZE: usize = 50;
        {
            // APP_START
            // https://github.com/meshcore-dev/MeshCore/blob/main/docs/companion_protocol.md#1-app-start
            const EXAMPLE_DATA: [u8; 13] = [
                0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x6d, 0x63, 0x63, 0x6c, 0x69,
            ];

            // test deserialization
            if let Some(command) = MeshCoreBleCommands::from_bytes(&EXAMPLE_DATA) {
                match command {
                    MeshCoreBleCommands::AppStart {
                        reserved,
                        application_name,
                    } => {
                        assert_eq!(&EXAMPLE_DATA[1..8], reserved);
                        assert_eq!(&EXAMPLE_DATA[8..], application_name);
                    }
                    _ => panic!("should have found an AppStart"),
                }
            } else {
                panic!("failed to deserialize example test vector");
            }

            // test serialization
            let app_start = MeshCoreBleCommands::AppStart {
                reserved: &EXAMPLE_DATA[1..7],
                application_name: &EXAMPLE_DATA[8..],
            };
            let mut buffer = [0u8; MAX_PACKET_SIZE];
            match app_start.to_bytes(&mut buffer) {
                Ok(used_bytes) => {
                    assert_eq!(EXAMPLE_DATA.len(), used_bytes);
                    assert_eq!(EXAMPLE_DATA, buffer[..used_bytes]);
                }
                Err(e) => panic!("failed serialization of APP_START: {e}"),
            }
        }
        {
            // DEVICE_QUERY
            // https://github.com/meshcore-dev/MeshCore/blob/main/docs/companion_protocol.md#2-device-query
            const EXAMPLE_DATA: [u8; 2] = [0x16, 0x03];

            // test deserialization
            if let Some(command) = MeshCoreBleCommands::from_bytes(&EXAMPLE_DATA) {
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
                    assert_eq!(EXAMPLE_DATA.len(), used_bytes);
                    assert_eq!(EXAMPLE_DATA, buffer[..used_bytes]);
                }
                Err(e) => panic!("failed serialization of DEVICE_QUERY: {e}"),
            }
        }
        {
            // GET_CHANNEL_INFO
            // https://github.com/meshcore-dev/MeshCore/blob/main/docs/companion_protocol.md#3-get-channel-info
            const EXAMPLE_DATA: [u8; 2] = [0x1F, 0x01];

            // test deserialization
            if let Some(command) = MeshCoreBleCommands::from_bytes(&EXAMPLE_DATA) {
                match command {
                    MeshCoreBleCommands::GetChannelInfo { channel_index } => {
                        assert_eq!(EXAMPLE_DATA[1], *channel_index);
                    }
                    _ => panic!("should have found an GET_CHANNEL_INFO"),
                }
            }

            // test serialization
            let get_channel_info = MeshCoreBleCommands::GetChannelInfo { channel_index: &1 };
            let mut buffer = [0u8; MAX_PACKET_SIZE];
            match get_channel_info.to_bytes(&mut buffer) {
                Ok(used_bytes) => {
                    assert_eq!(EXAMPLE_DATA.len(), used_bytes);
                    assert_eq!(EXAMPLE_DATA, buffer[..used_bytes]);
                }
                Err(e) => panic!("failed serialization of GET_CHANNEL_INFO: {e}"),
            }
        }
        {
            // SET_CHANNEL
            // https://github.com/meshcore-dev/MeshCore/blob/main/docs/companion_protocol.md#4-set-channel
            let mut mut_example_data: [u8; 50] = [0u8; 50];
            mut_example_data[0] = 0x20;
            mut_example_data[1] = 0x01;
            const NAME: [u8; 4] = [0x53, 0x4D, 0x53, 0x00];
            mut_example_data[2..(2 + NAME.len())].copy_from_slice(&NAME);
            const SECRET: [u8; 16] = u128::MAX.to_le_bytes();
            mut_example_data[34..].copy_from_slice(&SECRET);
            #[allow(non_snake_case)]
            let EXAMPLE_DATA: [u8; 50] = mut_example_data;

            // test deserialization
            if let Some(command) = MeshCoreBleCommands::from_bytes(&EXAMPLE_DATA) {
                match command {
                    MeshCoreBleCommands::SetChannel {
                        channel_index,
                        channel_name,
                        secret,
                    } => {
                        assert_eq!(EXAMPLE_DATA[1], *channel_index);
                        assert_eq!(
                            EXAMPLE_DATA[2..(2 + NAME.len())],
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
                    assert_eq!(EXAMPLE_DATA.len(), used_bytes);
                    assert_eq!(EXAMPLE_DATA, buffer[..used_bytes]);
                }
                Err(e) => panic!("failed serialization of SET_CHANNEL: {e}"),
            }
        }
    }
}
