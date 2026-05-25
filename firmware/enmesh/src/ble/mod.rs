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
        .set_io_capabilities(trouble_host::IoCapabilities::DisplayOnly)
        .build();

    // start the stack
    let runner = stack.runner();
    let mut peripheral = stack.peripheral();

    // start advertising
    let name = heapless::format!(32; "enmesh-{:X}{:X}{:X}{:X}{:X}{:X}",
        mac[0], mac[1], mac[2], mac[3], mac[4], mac[5])
    .unwrap();


    info!("{TAG} advertising '{}'", name.as_str());
    let server = Server::new_with_config(GapConfig::Peripheral(PeripheralConfig {
        // name: "enmesh",
        name: name.as_str(),
        appearance: &appearance::network_device::MESH_DEVICE,
    }))
    .unwrap();

    let _ = join(ble_task(runner), async {
        loop {
            match advertise("Trouble Example", &mut peripheral, &server).await {
                Ok(conn) => gatt_events_task(global_state, &server, &conn)
                    .await
                    .unwrap(),
                Err(e) => {
                    // #[cfg(feature = "defmt")]
                    // let e = defmt::Debug2Format(&e);
                    panic!("{TAG} error: {:?}", e);
                }
            }
        }
    })
    .await;
}

/// This is a background task that is required to run forever alongside any other BLE tasks.
async fn ble_task<C: Controller, P: PacketPool>(mut runner: Runner<'_, C, P>) {
    loop {
        if let Err(e) = runner.run().await {
            // #[cfg(feature = "defmt")]
            // let e = defmt::Debug2Format(&e);
            panic!("{TAG} error: {:?}", e);
        }
    }
}

/// Create an advertiser to use to connect to a BLE Central, and wait for it to connect.
async fn advertise<'values, 'server, C: Controller>(
    name: &'values str,
    peripheral: &mut Peripheral<'values, C, DefaultPacketPool>,
    server: &'server Server<'values>,
) -> Result<GattConnection<'values, 'server, DefaultPacketPool>, BleHostError<C::Error>> {
    let mut advertiser_data = [0; 31];
    let len = AdStructure::encode_slice(
        &[
            AdStructure::Flags(LE_GENERAL_DISCOVERABLE | BR_EDR_NOT_SUPPORTED),
            AdStructure::IncompleteServiceUuids16(&[[0x0f, 0x18]]),
            AdStructure::CompleteLocalName(name.as_bytes()),
        ],
        &mut advertiser_data[..],
    )?;
    let advertiser = peripheral
        .advertise(
            &Default::default(),
            Advertisement::ConnectableScannableUndirected {
                adv_data: &advertiser_data[..len],
                scan_data: &[],
            },
        )
        .await?;
    debug!("{TAG} advertising");
    let conn = advertiser.accept().await?.with_attribute_server(server)?;
    debug!("{TAG} connection established");
    Ok(conn)
}

/// provide GATT READ/WRITE handlers
mod meshcore;

/// Stream Events until the connection closes.
///
/// This function will handle the GATT events and process them.
/// This is how we interact with read and write requests.
async fn gatt_events_task<P: PacketPool>(
    global_state: &'static RwLock<NoopRawMutex, crate::State>,
    server: &Server<'_>,
    conn: &GattConnection<'_, '_, P>,
) -> Result<(), Error> {
    debug!("{TAG} connected");
    // publish that we have a BLE connection
    let mut global_state_lock = global_state.write().await;
    global_state_lock.ble_status = crate::state::BleStatus::Connected;
    drop(global_state_lock);

    // create gatt handlers for the new connection
    let mut meshcore_handler = meshcore::MeshCoreGattHandler::new(global_state);

    let reason = loop {
        match conn.next().await {
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
                    _ => event.accept(),
                };
                // This step is also performed at drop(), but writing it explicitly is necessary
                // in order to ensure reply is sent.
                match reply {
                    Ok(reply) => reply.send().await,
                    Err(e) => warn!("{TAG} error sending response: {:?}", e),
                };
            }
            _ => {} // ignore other Gatt Connection Events
        }
    };

    // publish that the BLE connection ended
    let mut global_state_lock = global_state.write().await;
    global_state_lock.ble_status = crate::state::BleStatus::Disconnected;
    drop(global_state_lock);
    debug!("{TAG} disconnected: {:?}", reason);
    Ok(())
}
