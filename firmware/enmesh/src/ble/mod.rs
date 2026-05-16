// provide the common crates via re-export
use common::*;

// provide logging primitives
use log::*;
const TAG: &str = "[BLE Host]";

/// provide scheduling primitives
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::rwlock::RwLock;
use embassy_futures::join::join;
// use embassy_futures::select::select;


// provide BLE primitives
use trouble_host::prelude::*;

/// our BLE server
#[gatt_server]
struct Server {
    /// support for meshcore companion BLE
    _meshcore_service: meshcore::MeshCoreService,
}


pub async fn run(
    _global_state: &'static RwLock<NoopRawMutex, crate::State>,
    ble_controller: impl trouble_host::Controller,
    mac: [u8; 6],
) {
    debug!("{TAG} starting...");

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
        .build();
    let runner = stack.runner();
    let mut peripheral = stack.peripheral();

    info!("{TAG} advertising");
    let server = Server::new_with_config(GapConfig::Peripheral(PeripheralConfig {
        name: "enmesh",
        appearance: &appearance::network_device::MESH_DEVICE,
    }))
    .unwrap();

    let _ = join(ble_task(runner), async {
        loop {
            match advertise("Trouble Example", &mut peripheral, &server).await {
                Ok(conn) => {
                    gatt_events_task(&server, &conn).await.unwrap()
                    // set up tasks when the connection is established to a central, so they don't run when no one is connected.
                    // let a = gatt_events_task(&server, &conn);
                    // let b = custom_task(&server, &conn, &stack);
                    // run until any task ends (usually because the connection has been closed),
                    // then return to advertising state.
                    // select(a, b).await;
                }
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

/// Stream Events until the connection closes.
///
/// This function will handle the GATT events and process them.
/// This is how we interact with read and write requests.
async fn gatt_events_task<P: PacketPool>(_server: &Server<'_>, conn: &GattConnection<'_, '_, P>) -> Result<(), Error> {
    // let level = server.battery_service.level;
    // let status_handle = server.battery_service.status.handle;
    // let mut status = false;
    let reason = loop {
        match conn.next().await {
            GattConnectionEvent::Disconnected { reason } => break reason,
            GattConnectionEvent::Gatt { event } => {
                let reply = match event {
                    GattEvent::Read(event) => {
                        // if event.handle() == level.handle {
                        //     let value = conn.get(&level);
                        //     info!("[gatt] Read Event to Level Characteristic: {:?}", value);
                        //     event.accept()
                        // } else if event.handle() == status_handle {
                        //     event.accept_unprocessed(&status)
                        // } else {
                        //     event.accept()
                        // }
                        // FIXME currently ignores all events
                        event.reject(AttErrorCode::ATTRIBUTE_NOT_FOUND)
                    }
                    GattEvent::Write(event) => {
                        // if event.handle() == level.handle {
                        //     event.with_data(|offset, data| {
                        //         info!("[gatt] Write Event to Level Characteristic at {}: {:?}", offset, data)
                        //     });
                        //     event.accept()
                        // } else if event.handle() == status_handle {
                        //     match event.validate(1, 1) {
                        //         Ok(()) => {
                        //             event.with_data(|offset, data| {
                        //                 if data.len() == 1 {
                        //                     // If data.len() is 1, offset must be 0 or else validate would have errored
                        //                     assert!(offset == 0);
                        //                     status = data[0] != 0;
                        //                 }
                        //             });
                        //             event.accept_unprocessed()
                        //         }
                        //         Err(err) => event.reject(err),
                        //     }
                        // } else {
                        //     event.accept()
                        // }
                        // FIXME currently ignores all events
                        event.reject(AttErrorCode::ATTRIBUTE_NOT_FOUND)
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
    info!("{TAG} disconnected: {:?}", reason);
    Ok(())
}