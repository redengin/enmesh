// provide the common crates via re-export
use common::*;

use meshcore::ble::MeshCoreService;
// provid the ble host primitives
use trouble_host::prelude::*;

pub struct MeshCoreGattHandler {}

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
        }
        else {
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
            event.accept_unprocessed()
        }
        else {
            event.reject(AttErrorCode::ATTRIBUTE_NOT_FOUND)
        }
    }
}
