mod server;

// provide the common crates via re-export
use common::*;

// provide logging primitives
// use log::*;
// const TAG: &str = "[BLE Host]";

// provide the enmesh firmware interfaces
use crate::prelude::*;


// provide the trouble host interfaces
use trouble_host_rand_core as rand_core;

pub async fn run(
    _global_state: &'static RwLock<NoopRawMutex, crate::State>,
    _ble_controller: impl trouble_host::Controller,
    _mac: [u8; 6],
    _random_generator: &mut (impl rand_core::RngCore + rand_core::CryptoRng),
) {
//     // debug!("{TAG} starting...");

//     // // create the stack
//     // const CONNECTIONS_MAX: usize = 1;
//     // const L2CAP_CHANNELS_MAX: usize = 1; // FIXME
//     // let mut resources: trouble_host::HostResources<
//     //     DefaultPacketPool,
//     //     CONNECTIONS_MAX,
//     //     L2CAP_CHANNELS_MAX,
//     // > = trouble_host::HostResources::new();
//     // let stack = trouble_host::new(ble_controller, &mut resources)
//     //     // initialize the ble host address
//     //     .set_random_address(Address::random(mac))
//     //     .set_random_generator_seed(random_generator);

//     // // add any stored bonds (i.e. previous pairings)
//     // let global_state_lock = global_state.read().await;
//     // let bonds = global_state_lock.settings.ble_settings.bonds.clone();
//     // drop(global_state_lock);
//     // for bond in bonds {
//     //     if let Some(bond_information) = bond {
//     //         debug!("{TAG} adding bond information: {:?}", bond_information);
//     //         match stack.add_bond_information(bond_information) {
//     //             Ok(()) => {}
//     //             Err(e) => {
//     //                 warn!("{TAG} failed to add bond information: {:?}", e);
//     //             }
//     //         }
//     //     }
//     // }

}
