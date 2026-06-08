// provide the common crates via re-export
use common::*;

// provid the ble host primitives
use trouble_host::prelude::*;

use embassy_sync::blocking_mutex::raw::NoopRawMutex;
/// provide scheduling primitives
use embassy_sync::rwlock::RwLock;

/// provide definition of MeshCore companion BLE service
use ::meshcore::ble::MeshCoreService;

pub struct MeshCoreGattHandler {
    // FIXME
    // global_state: &'static RwLock<NoopRawMutex, crate::State>,
}
impl MeshCoreGattHandler {
    pub fn new(_global_state: &'static RwLock<NoopRawMutex, crate::State>) -> Self {
        Self {
            // global_state
        }
    }
}
impl MeshCoreGattHandler {
    pub fn handle_gatt_read<'stack, 'server, P: PacketPool>(
        &mut self,
        event: ReadEvent<'stack, 'server, P>,
        service: &MeshCoreService,
        handle: u16,
    ) -> Result<Reply<'stack, P>, Error> {
        if handle == service.tx.handle {
            // TODO handle data
            event.reject(AttErrorCode::VALUE_NOT_ALLOWED)
        } else {
            event.reject(AttErrorCode::ATTRIBUTE_NOT_FOUND)
        }
    }

    pub fn handle_gatt_write<'stack, 'server, P: PacketPool>(
        &mut self,
        event: WriteEvent<'stack, 'server, P>,
        service: &MeshCoreService,
        handle: u16,
    ) -> Result<Reply<'stack, P>, Error> {
        if handle == service.rx.handle {
            // TODO handle data
            // event.accept_unprocessed()
            todo!()
        } else {
            event.reject(AttErrorCode::ATTRIBUTE_NOT_FOUND)
        }
    }
}

enum MeshCoreBleCommands<'a> {
    AppStart {
        reserved: &'a [u8],
        application_name: &'a [u8],
    },
    // DeviceQuery,
}
impl<'a> MeshCoreBleCommands<'a> {
    pub fn from_bytes(buffer: &'a [u8]) -> Option<Self> {
        return match buffer[0] {
            0x01 => Some(Self::AppStart {
                reserved: &buffer[1..8],
                application_name: &buffer[8..buffer.len()],
            }),
            // 0x16 => {
            //     if buffer[1] == 0x03 {
            //         return Some(Self::DeviceQuery);
            //     }
            //     None
            // }

            _ => None,
        };
    }

    pub fn to_bytes(&self, buffer: &mut [u8]) -> Result<usize, &str> {
        return match self {
            MeshCoreBleCommands::AppStart { reserved, application_name } => {
                buffer[0] = 0x01;
                buffer[1..7].copy_from_slice(reserved);
                buffer[8..(8 + application_name.len())].copy_from_slice(application_name);
                Ok(8 + application_name.len())
            }

            // MeshCoreBleCommands::DeviceQuery => {
            //     todo!()
            // }
        }
    }
}

// TESTING
//--------------------------------------------------------------------------------
#[cfg(test)]
mod tests {

use super::*;

    #[test]
    fn serde_MeshCoreBleCommands() {
        /// https://github.com/meshcore-dev/MeshCore/blob/main/docs/companion_protocol.md#mtu-maximum-transmission-unit
        const MAX_PACKET_SIZE:usize = 50;
        let mut buffer = [0u8; MAX_PACKET_SIZE];
        {   // APP_START
            // https://github.com/meshcore-dev/MeshCore/blob/main/docs/companion_protocol.md#1-app-start
            let example_data: [u8; _] = [0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x6d, 0x63, 0x63, 0x6c, 0x69];

            // test deserialization
            if let Some(command) = MeshCoreBleCommands::from_bytes(&example_data) {
                match command {
                    MeshCoreBleCommands::AppStart { reserved, application_name } => {
                        assert_eq!(&example_data[1..8], reserved);
                        assert_eq!(&example_data[8..], application_name);
                    }
                    _ => {
                        panic!("should have found an AppStart");
                    }
                }
            }
            else {
                panic!("failed to deserialize example test vector");
            }

            // test serialization
            let app_start = MeshCoreBleCommands::AppStart { reserved: &example_data[1..7], application_name: &example_data[8..] };
            match app_start.to_bytes(&mut buffer) {
                Ok(used_bytes) => {
                    assert_eq!(example_data.len(), used_bytes);
                }
                Err(e) => panic!("failed serialization of APP_START: {e}")
            }
        }
    }
}
