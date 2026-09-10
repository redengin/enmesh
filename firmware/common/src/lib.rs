#![no_std]

/// provide logging
/// - devices use the serial communications for console configuration
///     therefore they must support string logging (e.g. can't support defmt)
pub use log;

// embedded utils
//------------------------------------------------------------------------------
pub use static_cell;
pub use heapless;
pub use embedded_hal;
pub use embedded_hal_bus;
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

// graphics utils
//------------------------------------------------------------------------------
pub use embedded_graphics;
pub use embedded_layout;
//------------------------------------------------------------------------------

// Hardware drivers
//------------------------------------------------------------------------------
pub mod button;
pub mod led;
//..............................................................................

// LoRa hardware 
//..............................................................................
pub use lora_modulation;
pub use lora_phy;
//..............................................................................

// Display hardware
//..............................................................................
pub use display_interface;
pub use ssd1306;
//------------------------------------------------------------------------------

// BLE Host
//------------------------------------------------------------------------------
pub use trouble_host;
// pub use trouble_host_rand_core;
// pub use trouble_host_embassy_sync;
