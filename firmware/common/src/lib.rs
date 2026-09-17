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
pub use embedded_hal_async;
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
pub use profont;
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
//------------------------------------------------------------------------------

// BLE Host
//------------------------------------------------------------------------------
pub use trouble_host;
