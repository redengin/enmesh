#![no_std]
#![no_main]

// provide the shared crates via re-export
use common::*;
use soc_esp32::*; // (provides the panic handler)

// provide logging primitives
use log::*;

/// provide storage (for settings and stored LoRa traffic)
mod storage;

/// provide task implementations
mod tasks;

#[esp_rtos::main]
async fn main(spawner: embassy_executor::Spawner) {
    // initialize the SoC
    let peripherals = if cfg!(feature = "esp-radio") {
        esp_hal::init(
            esp_hal::Config::default()
                // max clocking required for esp_radio
                .with_cpu_clock(esp_hal::clock::CpuClock::max()),
        )
    } else {
        esp_hal::init(esp_hal::Config::default())
    };

    // initialize logging
    esp_println::logger::init_logger_from_env();
    info!("initializing...");

    debug!("initializing RTOS...");
    use esp_hal::timer::timg::TimerGroup;
    let timg0 = TimerGroup::new(peripherals.TIMG0);
    use esp_hal::interrupt::software::SoftwareInterruptControl;
    let sw_int = SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);
    esp_rtos::start(timg0.timer0, sw_int.software_interrupt0);
    // TODO by default idle hook simply runs WFI - but perhaps we want to do more to save power?
    // esp_rtos::start_with_idle_hook(timg0.timer0, sw_int.software_interrupt0, idle_hook);
    debug!("RTOS initialized");

    debug!("initializing storage...");
    let storage = storage::AppPartitions::new(peripherals.FLASH);
    debug!("storage initialized");
    debug!("initializing state...");
    let initial_state = enmesh_firmware::state::State::new();
    let global_state = embassy_sync::blocking_mutex::NoopMutex::new(initial_state);
    debug!("state initialized");

    // create the tasks

    debug!("creating LoRa task...");
    // heltec v3 pins https://heltec.org/wp-content/uploads/2023/09/pin.png
    let lora_peripherals = tasks::lora::LoraIo {
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
    spawner.spawn(tasks::lora::task_lora(lora_peripherals).unwrap());
    debug!("LoRa task created");

    debug!("creating usb serial task...");
    // https://dl.espressif.com/dl/schematics/SCH_ESP32-S3-DevKitC-1_V1.1_20220413.pdf#page=2
    configure_usb_serial(&peripherals.GPIO36, &peripherals.GPIO37);
    let usb_serial_io = tasks::usb_serial::UsbSerialIo {
        // usb: peripherals.USB0,
        // d_neg: peripherals.GPIO19,
        // d_pos: peripherals.GPIO20,
        uart: peripherals.UART0,
        rx: peripherals.GPIO44,
        tx: peripherals.GPIO43,
    };
    spawner.spawn(tasks::usb_serial::task_usb_serial(usb_serial_io).unwrap());
    debug!("usb serial task created");

    debug!("creating screen task...");
    // heltec v3 pins https://heltec.org/wp-content/uploads/2023/09/pin.png
    let screen_io = tasks::ux::UxIo {
        vext_control: peripherals.GPIO36,
        oled_reset: peripherals.GPIO21,
        i2c: peripherals.I2C0,
        sda: peripherals.GPIO17,
        scl: peripherals.GPIO18,
        button: peripherals.GPIO0,
    };
    spawner.spawn(tasks::ux::task_ux(screen_io).unwrap());
    debug!("screen task created");


    if cfg!(feature = "esp-radio") {
        // create a heap for esp_radio (bluetooth and/or wifi support)
        soc_esp32::init_heap();
    }

    if cfg!(feature = "wifi-bridge") {
        debug!("creating enmesh WiFi bridge task...");
        spawner.spawn(tasks::wifi::task_wifi_bridge(peripherals.WIFI).unwrap());
        debug!("enmesh WiFi bridge task created");
    }

    if cfg!(feature = "ble-companion") {
        debug!("creating enmesh ble compantion task...");
        spawner.spawn(tasks::ble::task_ble_companion(peripherals.BT).unwrap());
        debug!("enmesh ble companion task created");
    }

    info!("enmesh firmware running...");
}

// TODO move elsewhere - either a CP2102 crate or an internal crate
/// use CP2102 magic to change the USB parameters
fn configure_usb_serial(
    _rxd: &esp_hal::peripherals::GPIO36,
    _txd: &esp_hal::peripherals::GPIO37,
) {
    // TODO use magic to set the CP2102 'vendor':'product' 'label'
    // embassy_usb_serial_config.manufacturer = Some("Espressif");
    // embassy_usb_serial_config.product = Some("USB-serial example");
    // embassy_usb_serial_config.serial_number = Some("12345678");
    // // Required for windows compatibility.
    // // https://developer.nordicsemi.com/nRF_Connect_SDK/doc/1.9.1/kconfig/CONFIG_CDC_ACM_IAD.html#help
    // embassy_usb_serial_config.device_class = 0xEF;
    // embassy_usb_serial_config.device_sub_class = 0x02;
    // embassy_usb_serial_config.device_protocol = 0x01;
    // embassy_usb_serial_config.composite_with_iads = true;
}