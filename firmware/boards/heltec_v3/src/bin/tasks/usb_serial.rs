// provide the shared crates via re-export
use common::*;

// provide logging primitives
// use log::*;

use soc_esp32::*;


/// convenience structure for USB serial interfaces
pub(crate) struct UsbSerialIo {
    pub uart: esp_hal::peripherals::UART0<'static>,
    pub rx: esp_hal::peripherals::GPIO44<'static>,
    pub tx: esp_hal::peripherals::GPIO43<'static>,
    // usb: peripherals.USB0,
    // d_neg: peripherals.GPIO19,
    // d_pos: peripherals.GPIO20,
}

#[embassy_executor::task]
pub(crate) async fn task_usb_serial(usb_serial_io: UsbSerialIo) {

    let mut serial = esp_hal::uart::Uart::new(
        usb_serial_io.uart,
        esp_hal::uart::Config::default()
            .with_baudrate(115_200)     // match Meshcore baudrate
    )
    .unwrap()
    .with_rx(usb_serial_io.rx)
    .with_tx(usb_serial_io.tx)
    .into_async();

    // FIXME just do a simple echo for now
    loop {
        let mut buffer = [0u8; 1];
        if let Ok(read) = serial.read_async(&mut buffer).await {
            // echo
            let _ = serial.write_async(&buffer[0..read]).await;
        }
    }
}
