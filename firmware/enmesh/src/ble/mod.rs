// provide the common crates via re-export
use common::*;

// provide logging primitives
use log::*;
const TAG: &str = "[BLE Host]";

use embassy_futures::join::join;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
/// provide scheduling primitives
use embassy_sync::rwlock::RwLock;

// provide BLE primitives
use trouble_host::prelude::*;

/// our BLE server
#[gatt_server]
struct Server {
    /// support for meshcore companion BLE
    meshcore_service: ::meshcore::ble::MeshCoreService,
}

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
        _,
        DefaultPacketPool,
        CONNECTIONS_MAX,
        L2CAP_CHANNELS_MAX,
    > = trouble_host::HostResources::new();
    let stack = trouble_host::new(ble_controller, &mut resources)
        // initialize the ble host address
        .set_random_address(Address::random(mac))
        .set_random_generator_seed(random_generator)
        // require the client to input the passkey displayed on the device to pair
        .set_io_capabilities(trouble_host::IoCapabilities::DisplayOnly)
        .build();
    // add any stored bonds (i.e. previous pairings)
    let global_state_lock = global_state.read().await;
    let bonds = global_state_lock.settings.ble_settings.bonds.clone();
    drop(global_state_lock);
    for bond in bonds {
        if let Some(bond_information) = bond {
            debug!("{TAG} adding bond information: {:?}", bond_information);
            match stack.add_bond_information(bond_information) {
                Ok(()) => {}
                Err(e) => {
                    warn!("{TAG} failed to add bond information: {:?}", e);
                }
            }
        }
    }

    // create the ble host
    let runner = stack.runner();
    let mut peripheral = stack.peripheral();
    let server = Server::new_with_config(GapConfig::Peripheral(PeripheralConfig {
        name: "enmesh",
        // FIXME find a supported appearance
        appearance: &appearance::network_device::MESH_DEVICE,
    }))
    .unwrap();

    // start the ble host
    let _ = join(ble_task(runner), async {
        loop {
            match advertise(mac, &mut peripheral, &server).await {
                Ok(conn) => {
                    // support bonding
                    conn.raw().set_bondable(true).unwrap();
                    // handle the connection
                    let _ = gatt_events_task(global_state, &server, &conn).await;

                    // connection ended - update the global state
                    let mut global_state_lock = global_state.write().await;
                    global_state_lock.ble_status = crate::state::BleStatus::Advertising;
                    drop(global_state_lock);
                }
                Err(e) => {
                    panic!("{TAG} error: {:?}", e);
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
    server: &'server Server<'values>,
) -> Result<GattConnection<'values, 'server, DefaultPacketPool>, BleHostError<C::Error>> {
    // create the advertisement name
    const BLE_NAME_SIZE_MAX: usize = 29;
    let name = heapless::format!(BLE_NAME_SIZE_MAX; "EnMesh-{:X}{:X}{:X}{:X}{:X}{:X}",
        mac[0], mac[1], mac[2], mac[3], mac[4], mac[5])
    .unwrap();
    // create the advertisement
    let mut advertiser_data = [0; 31];
    let len = AdStructure::encode_slice(
        &[
            AdStructure::Flags(LE_GENERAL_DISCOVERABLE | BR_EDR_NOT_SUPPORTED),
            // AdStructure::IncompleteServiceUuids16(&[[0x0f, 0x18]]),
            AdStructure::CompleteLocalName(name.as_bytes()),
        ],
        &mut advertiser_data[..],
    )?;
    info!("{TAG} advertising '{}'", name.as_str());
    // advertise and await a connection
    let advertiser = peripheral
        .advertise(
            &Default::default(),
            Advertisement::ConnectableScannableUndirected {
                adv_data: &advertiser_data[..len],
                scan_data: &[],
            },
        )
        .await?;
    debug!("{TAG} connecting");
    let conn = advertiser.accept().await?.with_attribute_server(server)?;
    debug!("{TAG} connection established");
    Ok(conn)
}

/// provide support for MeshCore BLE compantion
mod meshcore;

/// Handle GATT Events until the connection closes
async fn gatt_events_task<P: PacketPool>(
    global_state: &'static RwLock<NoopRawMutex, crate::State>,
    server: &Server<'_>,
    gatt_connection: &GattConnection<'_, '_, P>,
) -> Result<(), Error> {
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
            }
            GattConnectionEvent::PassKeyConfirm(passkey) => {
                debug!("{TAG} received a PassKeyConfirm {passkey}");
            }

            GattConnectionEvent::PairingComplete { security_level, bond } => {
                debug!("{TAG} pairing complete: security level: {:?}, bond: {:?}", security_level, bond);

                if let Some(bond_information) = bond {
                    // add the new bond information to the settings
                    let mut global_state_lock = global_state.write().await;
                    global_state_lock.ble_status = crate::state::BleStatus::Connected;
                    global_state_lock.settings.ble_settings.add_binding(bond_information);
                    drop(global_state_lock);

                    // FIXME this must report the binding to the client
                }
            }

            GattConnectionEvent::PairingFailed(err) => {
                warn!("{TAG} pairing failed: {:?}", err);
            }
            GattConnectionEvent::Disconnected { reason } => { break reason }

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
                    _ => event.reject(AttErrorCode::REQUEST_NOT_SUPPORTED)
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

    // publish that the BLE connection ended
    debug!("{TAG} disconnected: {:?}", reason);

    Ok(())
}
