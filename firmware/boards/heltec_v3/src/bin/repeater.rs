#![no_std]
#![no_main]

// provide the shared crates via re-export
use common::*;
// use meshcore_firmware::*;

// use soc crate that provides the panic handler
use soc_esp32::*;

// provide logging primitives
use log::*;

// provice scheduling primitives
use common::embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use common::embassy_sync::mutex::Mutex;
// use common::embassy_time::{Delay, Duration, Timer};
use common::embassy_time::{Delay};

/// LoRa radio SPI bus
static LORA_SPI_BUS: static_cell::StaticCell<
    Mutex<CriticalSectionRawMutex, esp_hal::spi::master::Spi<'static, esp_hal::Async>>,
> = static_cell::StaticCell::new();

#[soc_esp32::esp_rtos::main]
async fn main(spawner: embassy_executor::Spawner) {
    // initialize the SoC interface
    let peripherals = esp_hal::init(
        esp_hal::Config::default(),
        // TODO do we want max performance?
        // .with_cpu_clock(esp_hal::clock::CpuClock::max()),
    );

    // initialize logging
    esp_println::logger::init_logger_from_env();
    info!("initializing...");

    //==============================================================================
    info!("initializing RTOS...");
    use esp_hal::timer::timg::TimerGroup;
    let timg0 = TimerGroup::new(peripherals.TIMG0);
    use esp_hal::interrupt::software::SoftwareInterruptControl;
    let sw_int = SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);
    esp_rtos::start(timg0.timer0, sw_int.software_interrupt0);
    info!("RTOS initialized");
    //==============================================================================

    //==============================================================================
    info!("initializing LoRA interface...");
    // heltec v3 pins https://heltec.org/wp-content/uploads/2023/09/pin.png
    // configure GPIO pins
    let lora_reset = esp_hal::gpio::Output::new(
        peripherals.GPIO12,
        esp_hal::gpio::Level::Low,
        esp_hal::gpio::OutputConfig::default(),
    );
    let lora_dio =
        esp_hal::gpio::Input::new(peripherals.GPIO14, esp_hal::gpio::InputConfig::default());
    let lora_busy =
        esp_hal::gpio::Input::new(peripherals.GPIO13, esp_hal::gpio::InputConfig::default());
    // configure SPI interface
    let lora_nss = esp_hal::gpio::Output::new(
        peripherals.GPIO8,
        esp_hal::gpio::Level::High,
        esp_hal::gpio::OutputConfig::default(),
    );
    let lora_sck = peripherals.GPIO9;
    let lora_mosi = peripherals.GPIO10;
    let lora_miso = peripherals.GPIO11;
    info!("LoRa interface initialized");
    //==============================================================================

    //==============================================================================
    info!("initializing USB Serial interface...");
    // TODO support serial console
    warn!("USB serial interface not implemented");
    // warn!("USB serial interface initialized");
    //==============================================================================

    //==============================================================================
    // initialize the tasks
    info!("creating mesh task...");
    spawner
        .spawn(task_mesh(
            lora_reset,
            lora_dio,
            lora_busy,
            peripherals.SPI2,
            lora_nss,
            lora_sck,
            lora_mosi,
            lora_miso,
        ))
        .unwrap();
    info!("mesh task created");
    //==============================================================================

    // TODO power saving during IDLE
    // Does esp32 embassy alread do this?
}

#[embassy_executor::task]
async fn task_mesh(
    lora_reset: esp_hal::gpio::Output<'static>,
    lora_dio: esp_hal::gpio::Input<'static>,
    lora_busy: esp_hal::gpio::Input<'static>,
    // lora_spi_device: embassy_embedded_hal::shared_bus::asynch::spi::SpiDevice<'static, >,
    spi: esp_hal::peripherals::SPI2<'static>,
    lora_nss: esp_hal::gpio::Output<'static>,
    lora_sck: esp_hal::peripherals::GPIO9<'static>,
    lora_mosi: esp_hal::peripherals::GPIO10<'static>,
    lora_miso: esp_hal::peripherals::GPIO11<'static>,
) {
    info!("initializing LoRa radio...");
    // create the SPI bus
    const SX1262_SPI_MHZ: u32 = 16; // recommended SPI frequency
    let lora_spi = esp_hal::spi::master::Spi::new(
        spi,
        esp_hal::spi::master::Config::default()
            .with_frequency(esp_hal::time::Rate::from_mhz(SX1262_SPI_MHZ))
            .with_mode(esp_hal::spi::Mode::_0),
    )
    .unwrap()
    .with_sck(lora_sck)
    .with_mosi(lora_mosi)
    .with_miso(lora_miso)
    .into_async();
    let lora_spi_bus = LORA_SPI_BUS.init(Mutex::new(lora_spi));
    let lora_spi_device =
        embassy_embedded_hal::shared_bus::asynch::spi::SpiDevice::new(lora_spi_bus, lora_nss);
    // create a lora radio instance
    let sx126x_config = lora_phy::sx126x::Config {
        chip: lora_phy::sx126x::Sx1262,
        // TODO are these the correct parameters?
        tcxo_ctrl: Some(lora_phy::sx126x::TcxoCtrlVoltage::Ctrl1V7),
        use_dcdc: false,
        rx_boost: true,
    };
    let lora_interface = lora_phy::iv::GenericSx126xInterfaceVariant::new(
        lora_reset, lora_dio, lora_busy, None, None,
    )
    .unwrap();
    let mut _lora_radio = lora_phy::LoRa::new(
        lora_phy::sx126x::Sx126x::new(lora_spi_device, lora_interface, sx126x_config),
        false,
        Delay,
    )
    .await
    .unwrap();
    info!("LoRa radio initialized");

    // run the repeater handler
    // let mut repeater = meshcore_firmware::Repeater::new(lora_radio);
    // repeater.run().await;

    error!("repeater handler stopped");

}

// #[embassy_executor::task]
// async fn task_ble_host(connector: esp_radio::ble::controller::BleConnector<'static>) {
//     use trouble_host::prelude::ExternalController;
//     let controller: ExternalController<_, 20> = ExternalController::new(connector);

//     // get the MAC
//     let mac_address = esp_hal::efuse::Efuse::read_base_mac_address();
//     // FIXME this code smells, there must be a more syntantically way
//     let mut mac: [u8; 6] = [0xff; 6];
//     for i in 0..mac_address.as_bytes().len() {
//         mac[i] = mac_address.as_bytes()[i];
//         if i > mac.len() { break; }
//     }

//     info!("Creating random number generator for BLE security");
//     let mut trng = esp_hal::rng::Trng::try_new().unwrap();

//     // should run forever
//     meshcore_firmware::app_interface::ble::run(controller, mac, &mut trng).await;

//     error!("BLE host stopped");
// }
