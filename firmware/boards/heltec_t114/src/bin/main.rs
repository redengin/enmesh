#![no_std]
#![no_main]

// provide the shared crates via re-export
use common::*;

// provide access to esp32 hardware
use soc_esp32::*; // (provides the panic handler)

// provide logging primitives
use log::*;

/// provide task implementations
mod tasks;

/// static non-volatile memory
static STORAGE: static_cell::StaticCell<
    soc_esp32::enmesh_storage::EnmeshStorage
> = static_cell::StaticCell::new();

#[esp_rtos::main]
async fn main(spawner: embassy_executor::Spawner) {
    // initialize the SoC
    let peripherals = if cfg!(feature = "_esp-radio") {
        esp_hal::init(
            esp_hal::Config::default()
                // max clocking required for esp_radio
                .with_cpu_clock(esp_hal::clock::CpuClock::max()),
        )
    } else {
        // use default clockick to save power
        esp_hal::init(esp_hal::Config::default())
    };

    // initialize logging
    esp_println::logger::init_logger_from_env();

    debug!("initializing RTOS...");
    use esp_hal::timer::timg::TimerGroup;
    let timg0 = TimerGroup::new(peripherals.TIMG0);
    use esp_hal::interrupt::software::SoftwareInterruptControl;
    let sw_int = SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);
    esp_rtos::start(timg0.timer0, sw_int.software_interrupt0);
    // TODO by default idle hook simply runs WFI - but perhaps we want to do more to save power?
    // esp_rtos::start_with_idle_hook(timg0.timer0, sw_int.software_interrupt0, idle_hook);

    debug!("initializing storage...");
    use enmesh_firmware::storage::EnmeshStorage;    // must have trait in scope to use
    // let storage = soc_esp32::enmesh_storage::EnmeshStorage::open(peripherals.FLASH);
    let storage = STORAGE.init(enmesh_storage::EnmeshStorage::open(peripherals.FLASH));

    debug!("initializing global state...");
    // create globally shared state
    use enmesh_firmware::prelude::*;
    let state = enmesh_firmware::State{
        firmware_version: env!("CARGO_PKG_VERSION"),
        ..Default::default()
    };
    let global_state = enmesh_firmware::STATE.init(RwLock::new(state));
    let persisted_settings = enmesh_firmware::PersistedSettings::new(
        global_state,
        storage.settings_partition_a(),
        storage.settings_partition_b(),
    );

    debug!("initializing settings...");
    // let settings = enmesh_firmware::Settings::load(storage);
    // let _global_state = embassy_sync::blocking_mutex::NoopMutex::new(initial_state);



    // create a heap for alloc support
    soc_esp32::init_heap();


    // create the tasks
    //--------------------------------------------------------------------------------
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
    spawner.spawn(tasks::lora::task_lora(global_state, lora_peripherals).unwrap());
    debug!("LoRa task created");

    debug!("creating screen task...");
    // heltec v3 pins https://heltec.org/wp-content/uploads/2023/09/pin.png
    let screen_io = tasks::ux::UxIo {
        vext_control: peripherals.GPIO36,
        oled_reset: peripherals.GPIO21,
        i2c: peripherals.I2C0,
        sda: peripherals.GPIO17,
        scl: peripherals.GPIO18,
        button: peripherals.GPIO0,
        led: peripherals.GPIO35,
    };
    spawner.spawn(tasks::ux::task_ux(global_state, screen_io).unwrap());
    debug!("screen task created");

    debug!("creating usb serial task...");
    // https://dl.espressif.com/dl/schematics/SCH_ESP32-S3-DevKitC-1_V1.1_20220413.pdf#page=2
    // configure_usb_serial(&peripherals.GPIO36, &peripherals.GPIO37);
    let usb_serial_io = tasks::usb_serial::UsbSerialIo {
        uart: peripherals.UART0,
        rx: peripherals.GPIO44,
        tx: peripherals.GPIO43,
    };
    spawner.spawn(tasks::usb_serial::task_usb_serial(global_state, usb_serial_io).unwrap());
    debug!("usb serial task created");

    if cfg!(feature = "wifi-bridge") {
        debug!("creating enmesh WiFi bridge task...");
        spawner.spawn(tasks::wifi::task_wifi_bridge(global_state, peripherals.WIFI).unwrap());
        debug!("enmesh WiFi bridge task created");
    }

    if cfg!(feature = "ble-companion") {
        debug!("creating enmesh ble compantion task...");
        spawner.spawn(tasks::ble::task_ble_companion(global_state, peripherals.BT).unwrap());
        debug!("enmesh ble companion task created");
    }

    info!("enmesh firmware running...");
}