#![no_std]

// provide logging
pub use log;

// embedded utils
//------------------------------------------------------------------------------
pub use static_cell;
pub use heapless;
pub use embedded_hal;
pub use embedded_hal_bus;
pub use embedded_graphics;
pub use embedded_layout;
// pub use embedded_storage;
//------------------------------------------------------------------------------

// embassy RTOS support
//------------------------------------------------------------------------------
pub use embassy_embedded_hal;
pub use embassy_executor;
pub use embassy_time;
pub use embassy_sync;
pub use embassy_futures;
pub use embassy_usb;
//------------------------------------------------------------------------------

// Hardware drivers
//------------------------------------------------------------------------------
pub mod button;
pub mod led;
//..............................................................................
pub use lora_modulation;
pub use lora_phy;
pub use display_interface;
pub use ssd1306;
pub use ssd1680;
//------------------------------------------------------------------------------

// BLE Host
//------------------------------------------------------------------------------
pub use trouble_host;
// pub use trouble_host_rand_core;
// pub use trouble_host_embassy_sync;
