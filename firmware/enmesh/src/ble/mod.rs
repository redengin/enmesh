mod server;

// provide the common crates via re-export
use common::*;

// provide logging primitives
use log::*;
use serde::{Deserialize, Serialize};
const TAG: &str = "[BLE Host]";

// provide the enmesh firmware interfaces
use crate::{ble::meshcore::MeshCoreGattHandler, prelude::*};

// provide the trouble host interfaces
use trouble_host::prelude::*;

// provide additional sync primitives
use embassy_futures::join::join;

pub async fn run(
    global_state: &'static RwLock<NoopRawMutex, crate::State>,
    ble_controller: impl trouble_host::Controller,
    mac: [u8; 6],
) {
    debug!("{TAG} starting...");

    // create the stack
    const CONNECTIONS_MAX: usize = 1;
    const L2CAP_CHANNELS_MAX: usize = 1; // FIXME
    let mut resources: trouble_host::HostResources<
        _,
        DefaultPacketPool,
        CONNECTIONS_MAX,
        L2CAP_CHANNELS_MAX,
    > = trouble_host::HostResources::new();
    let stack = trouble_host::new(ble_controller, &mut resources)
        // initialize the ble host address
        .set_random_address(Address::random(mac))
        // use display only IO capability for pairing
        .set_io_capabilities(trouble_host::IoCapabilities::DisplayOnly)
        .build();
    debug!("{TAG} BLE stack created...");

    // add any stored bonds (i.e. pairings)
    let global_state_lock = global_state.read().await;
    let stored_bonds = global_state_lock.settings.ble_settings.bonds.clone();
    drop(global_state_lock);
    for stored_bond in stored_bonds {
        if let Some(bond) = stored_bond {
            let bond_information: trouble_host::BondInformation = bond.into();
            match stack.add_bond_information(bond_information.clone()) {
                Ok(()) => {
                    debug!("{TAG} adding bond information: [{:?}]", bond_information);
                }
                Err(e) => {
                    warn!("{TAG} failed to add bond information: {:?}", e);
                }
            }
        }
    }

    // create the BLE Host server
    const BLE_NAME_SIZE_MAX: usize = 29;
    let name = heapless::format!(BLE_NAME_SIZE_MAX; "EnMesh-{:X}{:X}{:X}{:X}{:X}{:X}",
        mac[5], mac[4], mac[3], mac[2], mac[1], mac[0])
    .unwrap();
    let server = server::Server::new_with_config(GapConfig::Peripheral(PeripheralConfig {
        name: name.as_str(),
        appearance: &appearance::network_device::MESH_DEVICE,
    }))
    .unwrap();
    debug!("{TAG} host created");

    // start the ble host
    let _ = join(ble_task(stack.runner()), async {
        loop {
            // update the global state
            let mut global_state_lock = global_state.write().await;
            global_state_lock.ble_status = crate::state::BleStatus::Advertising;
            drop(global_state_lock);

            match advertise(name.as_str(), &mut stack.peripheral(), &server).await {
                Ok(conn) => {
                    debug!("{TAG} connecting...");
                    // support binding
                    conn.raw().set_bondable(true).unwrap();
                    // handle the connection
                    handle_connection(global_state, &server, &conn).await;
                }
                Err(e) => {
                    error!("{TAG} failed to advertise: {:?}", e);
                    warn!("{TAG} BLE advertising is disabled");
                    // abort BLE advertising
                    return;
                }
            }
        }
    })
    .await;
}

/// background task that is required to run forever alongside any other BLE tasks
async fn ble_task<C: Controller, P: PacketPool>(mut runner: Runner<'_, C, P>) {
    loop {
        if let Err(e) = runner.run().await {
            panic!("{TAG} error: {:?}", e);
        }
    }
}

/// BLE advertiser task that awaits a connection
async fn advertise<'values, 'server, C: Controller>(
    name: &str,
    peripheral: &mut Peripheral<'values, C, DefaultPacketPool>,
    server: &'server server::Server<'values>,
) -> Result<GattConnection<'values, 'server, DefaultPacketPool>, BleHostError<C::Error>> {
    // create the advertisement
    const BLE_ADV_DATA_SIZE_MAX: usize = 31;
    const _BLE5_ADV_DATA_SIZE_MAX: usize = 254;
    let mut advertisement_data = [0; BLE_ADV_DATA_SIZE_MAX];
    let advertisment_len = AdStructure::encode_slice(
        &[
            AdStructure::Flags(LE_GENERAL_DISCOVERABLE | BR_EDR_NOT_SUPPORTED),
            AdStructure::CompleteLocalName(name.as_bytes()),
        ],
        &mut advertisement_data[..],
    )?;
    let mut scan_data = [0; BLE_ADV_DATA_SIZE_MAX];
    let scan_len = AdStructure::encode_slice(
        &[AdStructure::IncompleteServiceUuids128(&[
            ::meshcore::ble::NORDIC_UART_SERVICE_UUID.to_le_bytes(),
            // FIXME currently another UUID128 won't fit
            // ::meshtastic::ble::MESHTASTIC_UUID.to_le_bytes(),
        ])],
        &mut scan_data[..],
    )?;
    info!("{TAG} advertising '{name}'");
    // debug!("ad_len: {advertisment_len}  scan_len: {scan_len}");
    let advertiser = peripheral
        .advertise(
            &Default::default(),
            Advertisement::ConnectableScannableUndirected {
                adv_data: &advertisement_data[..advertisment_len],
                scan_data: &scan_data[..scan_len],
            },
        )
        .await?;
    let conn = advertiser.accept().await?.with_attribute_server(server)?;
    Ok(conn)
}

// provide support for MeshCore BLE companion
mod meshcore;

/// Handle GATT Events until the connection closes
async fn handle_connection<P: PacketPool>(
    global_state: &'static RwLock<NoopRawMutex, crate::State>,
    server: &server::Server<'_>,
    gatt_connection: &GattConnection<'_, '_, P>,
) {
    // publish that we have a BLE connection
    debug!("{TAG} connected");
    // update the global state
    let mut global_state_lock = global_state.write().await;
    global_state_lock.ble_status = crate::state::BleStatus::Connected;
    drop(global_state_lock);

    // create gatt handlers for the new connection
    let mut meshcore_gatt_handler = meshcore::MeshCoreGattHandler::new(global_state);

    let mut needs_bond = false;

    let reason = loop {
        match gatt_connection.next().await {
            GattConnectionEvent::PassKeyDisplay(passkey) => {
                debug!("{TAG} received a PassKeyDisplay {passkey}");
                // notify the ux
                let mut global_state_lock = global_state.write().await;
                global_state_lock.ble_status = crate::state::BleStatus::Pairing {
                    passkey: passkey.value(),
                };
                drop(global_state_lock);
                needs_bond = true;
            }

            GattConnectionEvent::PairingComplete {
                security_level,
                bond,
            } => {
                debug!(
                    "{TAG} pairing complete: security level: {:?}, bond: {:?}",
                    security_level, bond
                );
                if needs_bond {
                    // add the new bond information to the settings
                    if let Some(bond_information) = bond {
                        let mut global_state_lock = global_state.write().await;
                        global_state_lock
                            .settings
                            .ble_settings
                            .add_binding(bond_information.into());
                        drop(global_state_lock);
                    }
                }
                let mut global_state_lock = global_state.write().await;
                global_state_lock.ble_status = crate::state::BleStatus::Connected;
                drop(global_state_lock);
            }

            GattConnectionEvent::PairingFailed(err) => {
                warn!("{TAG} pairing failed: {:?}", err);
            }
            GattConnectionEvent::Disconnected { reason } => break reason,

            GattConnectionEvent::Gatt { event } => {
                // handle event and send response
                match handle_gatt_event(server, event, &mut meshcore_gatt_handler) {
                    Ok(reply) => reply.send().await,
                    Err(e) => warn!("{TAG} error sending response: {:?}", e),
                }
            }

            _ => {} // ignore other Gatt Connection Events
        }
    };

    debug!("{TAG} disconnected: {:?}", reason);
}

fn handle_gatt_event<'server, 'stack, P: PacketPool>(
    server: &'server server::Server,
    event: GattEvent<'stack, 'server, P>,
    _meshcore_gatt_handler: &'server mut MeshCoreGattHandler,
) -> Result<Reply<'stack, P>, Error> {
    return match event {
        GattEvent::Read(event) => {
            let handle = event.handle();
            // handle meshcore
            if server.meshcore_service.handle_range().contains(&handle) {
                // TODO
                // meshcore_gatt_handler.handle_gatt_read(event, service, handle)
                event.reject(AttErrorCode::REQUEST_NOT_SUPPORTED)
            }
            // handle meshtastic
            else if server.meshtastic_service.handle_range().contains(&handle) {
                // TODO
                event.reject(AttErrorCode::REQUEST_NOT_SUPPORTED)
            } else {
                // ignore others
                event.reject(AttErrorCode::ATTRIBUTE_NOT_FOUND)
            }
        }
        GattEvent::Write(event) => {
            let handle = event.handle();
            // handle meshcore
            if server.meshcore_service.handle_range().contains(&handle) {
                // TODO
                // meshcore_gatt_handler.handle_gatt_write(event, service, handle)
                event.reject(AttErrorCode::REQUEST_NOT_SUPPORTED)
            }
            // handle meshtastic
            else if server.meshcore_service.handle_range().contains(&handle) {
                // TODO
                event.reject(AttErrorCode::REQUEST_NOT_SUPPORTED)
            } else {
                // ignore others
                event.reject(AttErrorCode::ATTRIBUTE_NOT_FOUND)
            }
        }
        // use the trouble reply to other events
        _ => event.accept(),
    };
}

/// support for storing BLE bonds
#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct StoredBleBond {
    /// BLE long term key
    long_term_key: u128,
    /// BLE peer address
    peer_address: [u8; 6],
}
impl From<trouble_host::BondInformation> for StoredBleBond {
    fn from(bond_information: trouble_host::BondInformation) -> Self {
        Self {
            long_term_key: bond_information.ltk.0,
            peer_address: bond_information.identity.addr.addr.0,
        }
    }
}
impl From<StoredBleBond> for trouble_host::BondInformation {
    fn from(stored_bond: StoredBleBond) -> Self {
        Self {
            ltk: trouble_host::LongTermKey(stored_bond.long_term_key),
            identity: trouble_host::Identity {
                addr: trouble_host::Address{
                    kind: AddrKind::RANDOM,
                    addr: BdAddr::new(stored_bond.peer_address)
                },
                irk: None,
            },
            is_bonded: true,
            security_level: trouble_host::connection::SecurityLevel::EncryptedAuthenticated,
        }
    }
}
