// provide the shared crates via re-export
use common::*;

// provide logging primitives
use log::*;

use soc_esp32::*;

// provide scheduling primitives
use enmesh_firmware::prelude::*;

/// convenience structure for USB serial interfaces
pub(crate) struct UsbSerialIo {
    pub uart: esp_hal::peripherals::UART0<'static>,
    pub rx: esp_hal::peripherals::GPIO44<'static>,
    pub tx: esp_hal::peripherals::GPIO43<'static>,
}

#[embassy_executor::task]
pub(crate) async fn task_usb_serial(
    global_state: &'static RwLock<NoopRawMutex, enmesh_firmware::State>,
    usb_serial_io: UsbSerialIo,
) {
    // hold off on starting up the serial interface so that boot logging completes
    Timer::after_secs(2).await;

    debug!("initializing usb serial...");
    let esp_serial = esp_hal::uart::Uart::new(
        usb_serial_io.uart,
        esp_hal::uart::Config::default().with_baudrate(115_200), // match Meshcore baudrate
    )
    .unwrap()
    .with_rx(usb_serial_io.rx)
    .with_tx(usb_serial_io.tx)
    .into_async();

    let serial = EnmeshSerial::new(esp_serial);
    enmesh_firmware::serial::run(&global_state, serial).await;

    error!("usb serial thread ended");
}

struct EnmeshSerial {
    serial: esp_hal::uart::Uart<'static, esp_hal::Async>,
}
impl EnmeshSerial {
    fn new(serial: esp_hal::uart::Uart<'static, esp_hal::Async>) -> Self {
        Self { serial }
    }
}
impl enmesh_firmware::serial::Serial for EnmeshSerial {
    type RxError = esp_hal::uart::RxError;
    type TxError = esp_hal::uart::TxError;

    async fn read_async(&mut self, buffer: &mut [u8]) -> Result<usize, Self::RxError>
    {
        self.serial.read_async(buffer).await
    }

    async fn write_async(&mut self, buffer: &[u8]) -> Result<usize, Self::TxError>
    {
        self.serial.write_async(buffer).await
    }
}