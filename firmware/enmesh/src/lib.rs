#![no_std]

/// provide a UX experience
pub mod ux;

/// provide enmesh LoRa support
pub mod repeater;

/// support boards that allow turning off peripherals (i.e. save power)
pub trait PowerControl {
    fn power_off(&mut self);

    #[allow(async_fn_in_trait)] // usage should never use Send()
    async fn power_on(&mut self);
}