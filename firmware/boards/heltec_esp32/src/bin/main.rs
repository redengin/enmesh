#![no_std]
#![no_main]

// provide the shared crates via re-export
use common::*;

// provide access to esp32 hardware
use soc_esp32::*; // (provides the panic handler)

// provide logging primitives
use log::*;

// provide enmesh firmware primitives
use enmesh_firmware::prelude::*;

/// provide task implementations
mod tasks;

#[esp_rtos::main]
async fn main(spawner: embassy_executor::Spawner) {
    // initialize the SoC
    let peripherals = if cfg!(feature = "disable-esp32-radio") {
        // use default clock tickrate to save power
        esp_hal::init(esp_hal::Config::default())
    } else {
        // use max clock tickrate to support WiFI/BLE
        esp_hal::init(esp_hal::Config::default().with_cpu_clock(esp_hal::clock::CpuClock::max()))
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

    debug!("initializing global state...");
    // create globally shared state
    let state = enmesh_firmware::State {
        firmware_version: env!("CARGO_PKG_VERSION"),
        hardware_name: "Heltec T114",
        ..Default::default()
    };
    let global_state = enmesh_firmware::STATE.init(RwLock::new(state));
    debug!("initializing storage...");
    let mut storage = soc_esp32::enmesh_storage::EnmeshStorage::open(peripherals.FLASH);
    let persisted_settings_manager =
        enmesh_firmware::persisted_settings::PersistedSettingsManager::init(
            global_state,
            storage.settings_partition_a.as_mut(),
            storage.settings_partition_b.as_mut(),
        )
        .await;

    spawner.spawn(
        task_persisted_settings(
            global_state,
            persisted_settings_manager,
            storage.settings_partition_a,
            storage.settings_partition_b,
        )
        .unwrap(),
    );

    // create a heap for alloc support
    soc_esp32::init_heap();

    // create the tasks
    //================================================================================

    // LoRa pin mapping & task
    //--------------------------------------------------------------------------------
    debug!("creating LoRa task...");
    // make sure that we know how to map the LoRa pins
    #[cfg(not(any(
        feature = "wifi_lora_32",
        feature = "wireless_stick_v2",
        // feature = "wireless_stick_v3",
        feature = "wireless_tracker",
        feature = "wireless_paper"
    )))]
    compile_error!(
        "LoRa pins unknown - board feature must be defined (use wifi_lora_32 for generic support)"
    );
    #[cfg(any(
        feature = "wifi_lora_32",
        feature = "wireless_stick_v2",
        feature = "wireless_tracker",
        feature = "wireless_paper"
    ))]
    // use the Heltec standard LoRa pin mapping
    let lora_io = tasks::lora::LoraIo {
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
        sck: esp_hal::gpio::Output::new(
            peripherals.GPIO9,
            esp_hal::gpio::Level::Low,
            esp_hal::gpio::OutputConfig::default(),
        ),
        mosi: esp_hal::gpio::Output::new(
            peripherals.GPIO10,
            esp_hal::gpio::Level::Low,
            esp_hal::gpio::OutputConfig::default(),
        ),
        miso: esp_hal::gpio::Input::new(peripherals.GPIO11, esp_hal::gpio::InputConfig::default()),
    };
    #[cfg(any(feature = "wireless_stick_v3",))]
    let lora_io = tasks::lora::LoraIo {
        reset: esp_hal::gpio::Output::new(
            peripherals.GPIO17,
            esp_hal::gpio::Level::Low,
            esp_hal::gpio::OutputConfig::default(),
        ),
        dio: esp_hal::gpio::Input::new(peripherals.GPIO26, esp_hal::gpio::InputConfig::default()),
        busy: esp_hal::gpio::Input::new(
            // FIXME no PIN identified for BUSY in schematic
            peripherals.GPIO13,
            esp_hal::gpio::InputConfig::default(),
        ),
        spi: peripherals.SPI2,
        nss: esp_hal::gpio::Output::new(
            peripherals.GPIO18,
            esp_hal::gpio::Level::High,
            esp_hal::gpio::OutputConfig::default(),
        ),
        sck: esp_hal::gpio::Output::new(
            peripherals.GPIO5,
            esp_hal::gpio::Level::Low,
            esp_hal::gpio::OutputConfig::default(),
        ),
        mosi: esp_hal::gpio::Output::new(
            peripherals.GPIO27,
            esp_hal::gpio::Level::Low,
            esp_hal::gpio::OutputConfig::default(),
        ),
        miso: esp_hal::gpio::Input::new(peripherals.GPIO19, esp_hal::gpio::InputConfig::default()),
    };

    spawner.spawn(tasks::lora::task_lora(global_state, lora_io).unwrap());
    debug!("LoRa task created");

    // Screen pin mapping & task
    //--------------------------------------------------------------------------------
    if cfg!(feature = "_use_screen") {
        debug!("creating screen task...");
        // heltec t114 pins https://heltec.org/wp-content/uploads/2023/09/pin.png
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
    }
    debug!("screen task created");

    // USB serial pin mapping & task
    //--------------------------------------------------------------------------------
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

    // Wifi and BLE pin mapping & tasks
    //--------------------------------------------------------------------------------
    if cfg!(not(feature = "disable-esp32-radio")) {
        // debug!("creating enmesh WiFi bridge task...");
        // spawner.spawn(tasks::wifi::task_wifi_bridge(global_state, peripherals.WIFI).unwrap());
        // debug!("enmesh WiFi bridge task created");

        // debug!("creating enmesh ble compantion task...");
        // spawner.spawn(tasks::ble::task_ble_companion(global_state, peripherals.BT).unwrap());
        // debug!("enmesh ble companion task created");
    }

    info!("enmesh firmware running...");
}

#[embassy_executor::task]
pub async fn task_persisted_settings(
    global_state: &'static RwLock<NoopRawMutex, enmesh_firmware::State>,
    mut persisted_settings_manager: enmesh_firmware::persisted_settings::PersistedSettingsManager,
    mut settings_partition_a: Option<enmesh_storage::Partition>,
    mut settings_partition_b: Option<enmesh_storage::Partition>,
) {
    debug!("creating persisted settings task...");

    persisted_settings_manager
        .run(
            global_state,
            settings_partition_a.as_mut(),
            settings_partition_b.as_mut(),
        )
        .await;

    error!("persisted settings task ended");
}
