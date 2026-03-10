// provide the shared crates via re-export
use common::*;

// provide logging primitives
use log::*;

// use soc crate (it provides the panic handler)
use soc_esp32::*;


/// convenience structure for USB serial interfaces
pub(crate) struct UsbSerialIo {
    pub usb: esp_hal::peripherals::USB0<'static>,
    pub d_neg: esp_hal::peripherals::GPIO19<'static>,
    pub d_pos: esp_hal::peripherals::GPIO20<'static>,
}

#[embassy_executor::task]
pub(crate) async fn task_usb_serial(usb_serial_io: UsbSerialIo) {
    // FIXME is this already wired to stdio?
}
