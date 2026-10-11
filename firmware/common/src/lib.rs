#![no_std]

/// provide logging
/// - devices use the serial communications for console configuration
///     therefore they must support string logging (e.g. can't support defmt)
pub use log;

// embedded utils
//------------------------------------------------------------------------------
pub use embedded_hal;
pub use embedded_hal_async;
pub use embedded_hal_bus;
pub use static_cell;
pub use heapless;
pub use embedded_graphics;
pub use embedded_layout;
//------------------------------------------------------------------------------

// embassy RTOS support
//------------------------------------------------------------------------------
pub use embassy_executor;
pub use embassy_time;
pub use embassy_embedded_hal;
pub use embassy_sync;
pub use embassy_futures;
pub use embassy_usb;
//------------------------------------------------------------------------------

// LoRa support
//..............................................................................
pub use lora_modulation;
pub use lora_phy;
//..............................................................................

// graphics support
//------------------------------------------------------------------------------
pub use display_interface;
pub use profont;
pub use tinyqoi;
//------------------------------------------------------------------------------

// Hardware drivers
//------------------------------------------------------------------------------
pub mod button;
pub mod led;
//..............................................................................

// BLE Host
//------------------------------------------------------------------------------
pub use trouble_host;
