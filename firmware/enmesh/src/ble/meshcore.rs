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
