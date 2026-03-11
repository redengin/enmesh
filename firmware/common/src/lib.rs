#![no_std]

// provide logging
pub use log;

// embedded utils
//------------------------------------------------------------------------------
pub use static_cell;
//------------------------------------------------------------------------------

// embassy RTOS support
//------------------------------------------------------------------------------
pub use embassy_executor;
pub use embassy_time;
pub use embassy_sync;
pub use embassy_embedded_hal;
//------------------------------------------------------------------------------

// Hardware drivers
//------------------------------------------------------------------------------
pub use lora_phy;
pub use ssd1306;
//------------------------------------------------------------------------------

// provide BLE Host
pub use trouble_host;
