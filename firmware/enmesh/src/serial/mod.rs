// // provide the common crates via re-export
// use common::*;

// // provide logging primitives
// use log::*;
// const TAG: &str = "[Serial Task]";

// /// provide scheduling primitives
// use embassy_sync::blocking_mutex::raw::NoopRawMutex;
// use embassy_sync::rwlock::RwLock;
// // use embassy_time::{Delay, Timer};


// pub async fn run(
//     global_state: &'static RwLock<NoopRawMutex, crate::State>,
//     serial: &'static 
// ) {
// }