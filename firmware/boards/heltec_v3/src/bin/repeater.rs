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
use common::embassy_time::Delay;

/// LoRa radio SPI bus
static LORA_SPI_BUS: static_cell::StaticCell<
    Mutex<CriticalSectionRawMutex, esp_hal::spi::master::Spi<'static, esp_hal::Async>>,
> = static_cell::StaticCell::new();

#[soc_esp32::esp_rtos::main]
async fn main(spawner: embassy_executor::Spawner) {
    // initialize the SoC interface
    #[cfg(feature = "enmesh_wifi")]
    let peripherals = esp_hal::init(
        esp_hal::Config::default()
            // max clocking required for WiFi
            .with_cpu_clock(esp_hal::clock::CpuClock::max()),
    );
    #[cfg(not(feature = "enmesh_wifi"))]
    let peripherals = esp_hal::init(
        esp_hal::Config::default(), // TODO do we want max performance (at the cost of extra power consumption)?
                                    // .with_cpu_clock(esp_hal::clock::CpuClock::max()),
    );

    // initialize logging
    esp_println::logger::init_logger_from_env();
    info!("initializing...");

    //==============================================================================
    debug!("initializing RTOS...");
    use esp_hal::timer::timg::TimerGroup;
    let timg0 = TimerGroup::new(peripherals.TIMG0);
    use esp_hal::interrupt::software::SoftwareInterruptControl;
    let sw_int = SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);
    esp_rtos::start(timg0.timer0, sw_int.software_interrupt0);
    debug!("RTOS initialized");
    //==============================================================================

    //==============================================================================
    debug!("creating LoRa task...");
    // heltec v3 pins https://heltec.org/wp-content/uploads/2023/09/pin.png
    let lora_io = LoraIo {
        reset: esp_hal::gpio::Output::new(
            peripherals.GPIO12,
            esp_hal::gpio::Level::Low,
            esp_hal::gpio::OutputConfig::default(),
        ),
        dio: esp_hal::gpio::Input::new(peripherals.GPIO14, esp_hal::gpio::InputConfig::default()),
        busy: esp_hal::gpio::Input::new(peripherals.GPIO13, esp_hal::gpio::InputConfig::default()),
        spi: peripherals.SPI2,
        nss: esp_hal::gpio::Output::new(
            peripherals.GPIO8,
            esp_hal::gpio::Level::High,
            esp_hal::gpio::OutputConfig::default(),
        ),
        sck: peripherals.GPIO9,
        mosi: peripherals.GPIO10,
        miso: peripherals.GPIO11,
    };
    spawner.spawn(task_lora(lora_io)).unwrap();
    debug!("LoRa task created");
    //==============================================================================

    //==============================================================================
    debug!("creating usb serial task...");
    // FIXME - haven't found how to use dev kit USB header (only pins 20/19)
    // let usb_serial_io = UsbSerialIo {
    //     usb: peripherals.USB0,
    //     d_pos: peripherals.GPIO20,
    //     d_neg: peripherals.GPIO19,
    // };
    // spawner.spawn(task_usb_serial(usb_serial_io)).unwrap();
    warn!("TODO support USB connection");
    // debug!("usb_serial task created");

    //==============================================================================

    info!("enmesh firmware running...");

    // TODO power saving during IDLE
    // Does esp32 embassy alread do this?
}

/// convenience struct for the LoRa radio IO interfaces
struct LoraIo {
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
async fn task_lora(lora_io: LoraIo) {
    info!("initializing LoRa radio...");

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
    let mut _lora_radio = lora_phy::LoRa::new(
        lora_phy::sx126x::Sx126x::new(lora_spi_device, lora_interface, sx126x_config),
        false,
        Delay,
    )
    .await
    .unwrap();
    debug!("LoRa radio instance created.");

    info!("LoRa radio initialized");

    // run the repeater handler
    // let mut repeater = meshcore_firmware::Repeater::new(lora_radio);
    // repeater.run().await;

    panic!("LoRa task ended");
}

// /// convenience structure for USB serial interfaces
// struct UsbSerialIo {
//     pub usb: esp_hal::peripherals::USB0<'static>,
//     pub d_pos: esp_hal::peripherals::GPIO20<'static>,
//     pub d_neg: esp_hal::peripherals::GPIO19<'static>,
// }

// #[embassy_executor::task]
// async fn task_usb_serial(usb_serial_io: UsbSerialIo) {
//     info!("initializing USB Serial interface...");
//     let usb = esp_hal::otg_fs::Usb::new(usb_serial_io.usb, usb_serial_io.d_pos, usb_serial_io.d_neg);
//     let mut ep_out_buffer = [0u8; 1024];
//     let usb_serial_config = esp_hal::otg_fs::asynch::Config::default();
//     let usb_serial_driver =
//         esp_hal::otg_fs::asynch::Driver::new(usb, &mut ep_out_buffer, usb_serial_config);
//     let mut embassy_usb_serial_config = embassy_usb::Config::new(0x303A, 0x3001);
//     embassy_usb_serial_config.manufacturer = Some("Espressif");
//     embassy_usb_serial_config.product = Some("USB-serial example");
//     embassy_usb_serial_config.serial_number = Some("12345678");
//     // Required for windows compatibility.
//     // https://developer.nordicsemi.com/nRF_Connect_SDK/doc/1.9.1/kconfig/CONFIG_CDC_ACM_IAD.html#help
//     embassy_usb_serial_config.device_class = 0xEF;
//     embassy_usb_serial_config.device_sub_class = 0x02;
//     embassy_usb_serial_config.device_protocol = 0x01;
//     embassy_usb_serial_config.composite_with_iads = true;

//     // create ...
//     let mut config_descriptor = [0; 256];
//     let mut bos_descriptor = [0; 256];
//     let mut control_buf = [0; 64];
//     let mut usb_state = embassy_usb::class::cdc_acm::State::new();

//     let mut builder = embassy_usb::Builder::new(
//         usb_serial_driver,
//         embassy_usb_serial_config,
//         &mut config_descriptor,
//         &mut bos_descriptor,
//         &mut [], // no msos descriptors
//         &mut control_buf,
//     );

//     // Create classes on the builder.
//     let mut class = embassy_usb::class::cdc_acm::CdcAcmClass::new(&mut builder, &mut usb_state, 64);

//     // Build the builder.
//     let mut usb_serial_device = builder.build();

//     // Run the USB device.
//     let usb_fut = usb_serial_device.run();

//     // Do stuff with the class!
//     let echo_fut = async {
//         loop {
//             class.wait_connection().await;
//             esp_println::println!("Connected");
//             // FIXME
//             // let _ = echo(&mut class).await;
//             esp_println::println!("Disconnected");
//         }
//     };
//     // TODO support serial console

//     usb_fut.await;
//     panic!("USB serial task ended");
// }
