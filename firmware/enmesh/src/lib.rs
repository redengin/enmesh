#![no_std]
extern crate alloc;

/// provide primitives necessary to use enmesh firmware
pub mod prelude {
    pub use common::log::*;
    pub use common::embassy_sync::rwlock::RwLock;
    pub use common::embassy_sync::blocking_mutex::raw::NoopRawMutex;
    pub use common::embassy_time::{Timer, Duration, Instant, Ticker};
}

/// globally shared state for firmware
/// settings, status, etc.
mod state;
pub use state::{STATE, State};

/// persistable settings
mod settings;
pub use settings::{Settings};
pub mod persisted_settings;
pub mod storage;

/// provide enmesh LoRa support
pub mod lora;

/// provide serial console support
pub mod serial;

/// provide a UX experience
pub mod ux;

/// provide BLE support
pub mod ble;

/// support boards that allow turning off peripherals (i.e. save power)
pub trait PowerControl {
    fn power_off(&mut self);

    #[allow(async_fn_in_trait)] // usage should never use Send()
    /// must reinitialize the hardware as necessary
    async fn power_on(&mut self);
}

