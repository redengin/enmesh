mod server;

// provide the common crates via re-export
use common::*;

// provide logging primitives
use log::*;
use serde::{Deserialize, Serialize};
const TAG: &str = "[BLE Host]";

// provide the enmesh firmware interfaces
use crate::prelude::*;

// provide the trouble host interfaces
use trouble_host::prelude::*;
use trouble_host_rand_core as rand_core;

// provide additional sync primitives
use embassy_futures::join::join;

pub async fn run(
    global_state: &'static RwLock<NoopRawMutex, crate::State>,
    ble_controller: impl trouble_host::Controller,
    mac: [u8; 6],
    random_generator: &mut (impl rand_core::RngCore + rand_core::CryptoRng),
) {
    debug!("{TAG} starting...");

    // create the stack
    const CONNECTIONS_MAX: usize = 1;
    const L2CAP_CHANNELS_MAX: usize = 1; // FIXME
    let mut resources: trouble_host::HostResources<
        DefaultPacketPool,
        CONNECTIONS_MAX,
        L2CAP_CHANNELS_MAX,
    > = trouble_host::HostResources::new();
    let stack = trouble_host::new(ble_controller, &mut resources)
        // initialize the ble host address
        .set_random_address(Address::random(mac))
        .set_random_generator_seed(random_generator);
    // require the client to input the passkey displayed on the device to pair
    stack.set_io_capabilities(trouble_host::IoCapabilities::DisplayOnly);
    debug!("{TAG} stack created");

    // add any stored bonds (i.e. previous pairings)
    let global_state_lock = global_state.read().await;
    let stored_bonds = global_state_lock.settings.ble_settings.bonds.clone();
    drop(global_state_lock);
    for stored_bond in stored_bonds {
        if let Some(bond) = stored_bond {
            // debug!("{TAG} adding bond information: [bond: {:?}]", bond);
            match stack.add_bond_information(bond.into()) {
                Ok(()) => {}
                Err(e) => {
                    warn!("{TAG} failed to add bond information: {:?}", e);
                }
            }
        }
    }

    // create the BLE host
    let Host {
        mut peripheral,
        runner,
        ..
    } = stack.build();
    let server = server::Server::new_with_config(GapConfig::Peripheral(PeripheralConfig {
        name: "enmesh", // internal label, not advertised over BLE
        appearance: &appearance::network_device::MESH_DEVICE,
    }))
    .unwrap();
    debug!("{TAG} host created");

    // start the ble host
    let _ = join(ble_task(runner), async {
        loop {
            // update the global state
            let mut global_state_lock = global_state.write().await;
            global_state_lock.ble_status = crate::state::BleStatus::Advertising;
            drop(global_state_lock);

            match advertise(mac, &mut peripheral, &server).await {
                Ok(conn) => {
                    debug!("{TAG} connecting...");
                    // support bonding
                    conn.raw().set_bondable(true).unwrap();
                    // handle the connection
                    handle_connection(global_state, &server, &conn).await;
                }
                Err(e) => {
                    error!("{TAG} failed to advertise: {:?}", e);
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
    mac: [u8; 6],
    peripheral: &mut Peripheral<'values, C, DefaultPacketPool>,
    server: &'server server::Server<'values>,
) -> Result<GattConnection<'values, 'server, DefaultPacketPool>, BleHostError<C::Error>> {
    // create the advertisement name
    const BLE_NAME_SIZE_MAX: usize = 29;
    let name = heapless::format!(BLE_NAME_SIZE_MAX; "EnMesh-{:X}{:X}{:X}{:X}{:X}{:X}",
        mac[0], mac[1], mac[2], mac[3], mac[4], mac[5])
    .unwrap();

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
        &[
            // AdStructure::CompleteServiceUuids128(&[
            //     ::meshcore::ble::NORDIC_UART_SERVICE_UUID.to_le_bytes(),
            // ]),
        ],
        &mut scan_data[..],
    )?;
    info!("{TAG} advertising '{}'", name.as_str());
    // advertise and await a connection
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
    let mut meshcore_handler = meshcore::MeshCoreGattHandler::new(global_state);

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
            }

            GattConnectionEvent::PairingComplete {
                security_level,
                bond,
            } => {
                debug!(
                    "{TAG} pairing complete: security level: {:?}, bond: {:?}",
                    security_level, bond
                );

                if let Some(bond_information) = bond {
                    // add the new bond information to the settings
                    let mut global_state_lock = global_state.write().await;
                    global_state_lock.ble_status = crate::state::BleStatus::Connected;
                    global_state_lock
                        .settings
                        .ble_settings
                        .add_binding(bond_information.into());
                    drop(global_state_lock);
                }
            }

            GattConnectionEvent::PairingFailed(err) => {
                warn!("{TAG} pairing failed: {:?}", err);
            }
            GattConnectionEvent::Disconnected { reason } => break reason,

            GattConnectionEvent::Gatt { event } => {
                let reply = match event {
                    GattEvent::Read(event) => {
                        match event.handle() {
                            // handle meshcore
                            handle if handle == server.meshcore_service.tx.handle => {
                                meshcore_handler.handle_gatt_read(
                                    event,
                                    &server.meshcore_service,
                                    handle,
                                )
                            }
                            // ignore others
                            _ => event.reject(AttErrorCode::ATTRIBUTE_NOT_FOUND),
                        }
                    }
                    GattEvent::Write(event) => {
                        match event.handle() {
                            // handle meshcore
                            handle if handle == server.meshcore_service.tx.handle => {
                                meshcore_handler.handle_gatt_write(
                                    event,
                                    &server.meshcore_service,
                                    handle,
                                )
                            }
                            // ignore others
                            _ => event.reject(AttErrorCode::ATTRIBUTE_NOT_FOUND),
                        }
                    }
                    _ => event.accept()
                };

                // send response
                match reply {
                    Ok(reply) => reply.send().await,
                    Err(e) => warn!("{TAG} error sending response: {:?}", e),
                }
            }

            _ => {} // ignore other Gatt Connection Events
        }
    };

    debug!("{TAG} disconnected: {:?}", reason);
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
            peer_address: bond_information.identity.bd_addr.0,
        }
    }
}
impl From<StoredBleBond> for trouble_host::BondInformation {
    fn from(stored_bond: StoredBleBond) -> Self {
        Self {
            ltk: trouble_host::LongTermKey(stored_bond.long_term_key),
            identity: trouble_host::Identity {
                bd_addr: BdAddr::new(stored_bond.peer_address),
                irk: None,
            },
            is_bonded: true,
            security_level: trouble_host::connection::SecurityLevel::EncryptedAuthenticated,
        }
    }
}

