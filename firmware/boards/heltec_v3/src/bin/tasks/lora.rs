// provide the shared crates via re-export
use common::*;
use soc_esp32::*;

// provide logging primitives
use log::*;

// provide scheduling primitives
use embassy_time::Delay;

/// static LoRa radio SPI bus
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, mutex::Mutex};
static LORA_SPI_BUS: static_cell::StaticCell<
    Mutex<CriticalSectionRawMutex, esp_hal::spi::master::Spi<'static, esp_hal::Async>>,
> = static_cell::StaticCell::new();

/// convenience struct for the LoRa radio IO interfaces
pub struct LoraIo {
    pub reset: esp_hal::gpio::Output<'static>,
    pub dio: esp_hal::gpio::Input<'static>,
    pub busy: esp_hal::gpio::Input<'static>,
    pub spi: esp_hal::peripherals::SPI2<'static>,
    pub nss: esp_hal::gpio::Output<'static>,
    pub sck: esp_hal::peripherals::GPIO9<'static>,
    pub mosi: esp_hal::peripherals::GPIO10<'static>,
    pub miso: esp_hal::peripherals::GPIO11<'static>,
}
#[embassy_executor::task]
pub async fn task_lora(lora_io: LoraIo) {
    debug!("initializing LoRa radio...");

    debug!("creating LoRa SPI bus...");
    const SX1262_SPI_MHZ: u32 = 16; // recommended SPI frequency
    let lora_spi = esp_hal::spi::master::Spi::new(
        lora_io.spi,
        esp_hal::spi::master::Config::default()
            .with_frequency(esp_hal::time::Rate::from_mhz(SX1262_SPI_MHZ))
            .with_mode(esp_hal::spi::Mode::_0),
    )
    .unwrap()
    .with_sck(lora_io.sck)
    .with_mosi(lora_io.mosi)
    .with_miso(lora_io.miso)
    .into_async();
    let lora_spi_bus = LORA_SPI_BUS.init(Mutex::new(lora_spi));
    let lora_spi_device =
        embassy_embedded_hal::shared_bus::asynch::spi::SpiDevice::new(lora_spi_bus, lora_io.nss);
    debug!("LoRa SPI bus created");

    debug!("creating LoRa radio instance...");
    let sx126x_config = lora_phy::sx126x::Config {
        chip: lora_phy::sx126x::Sx1262,
        // TODO are these the correct parameters?
        tcxo_ctrl: Some(lora_phy::sx126x::TcxoCtrlVoltage::Ctrl1V7),
        use_dcdc: false,
        rx_boost: true,
    };
    let lora_interface = lora_phy::iv::GenericSx126xInterfaceVariant::new(
        lora_io.reset,
        lora_io.dio,
        lora_io.busy,
        None,
        None,
    )
    .unwrap();
    let lora_radio = lora_phy::LoRa::new(
        lora_phy::sx126x::Sx126x::new(lora_spi_device, lora_interface, sx126x_config),
        false,
        Delay,
    )
    .await
    .unwrap();

    info!("LoRa radio initialized");

    // run the repeater handler
    // enmesh::lora::run(lora_radio).await;
    enmesh_firmware::lora::run(lora_radio).await;

    // panic!("LoRa task ended");
}
