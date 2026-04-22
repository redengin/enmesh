#![no_std]

/// provide storage traits
pub mod storage;

/// globally shared state for firmware
/// configuration, status, etc.
mod state;
pub use state::State;

/// provide a UX experience
pub mod ux;

/// provide enmesh LoRa support
pub mod lora;

/// support boards that allow turning off peripherals (i.e. save power)
pub trait PowerControl {
    fn power_off(&mut self);

    #[allow(async_fn_in_trait)] // usage should never use Send()
    async fn power_on(&mut self);
}