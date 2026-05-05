#![no_std]
extern crate alloc;

/// provide primitives necessary to use enmesh firmware
pub mod prelude {
    pub use common::embassy_sync::rwlock::RwLock;
    pub use common::embassy_sync::blocking_mutex::raw::NoopRawMutex;
}

/// globally shared state for firmware
/// settings, status, etc.
mod state;
pub use state::{STATE, State};

/// persistable settings
mod settings;
pub use settings::Settings;

pub mod storage;

/// provide a UX experience
pub mod ux;
// pub mod widgets;

/// provide enmesh LoRa support
pub mod lora;

/// support boards that allow turning off peripherals (i.e. save power)
pub trait PowerControl {
    fn power_off(&mut self);

    #[allow(async_fn_in_trait)] // usage should never use Send()
    async fn power_on(&mut self);
}

