#![no_std]
#![no_main]

/// provide the shared crates via re-export
use common::*;

/// provide logging primitives from
use log::*;

/// provide enmesh firmware primitives
use enmesh_firmware::prelude::*;

/// provide access to esp32 hardware
use soc_esp32::*; // (provides the panic handler)

/// provide task implementations
mod tasks;

#[esp_rtos::main]
async fn main(spawner: embassy_executor::Spawner) {
    // create a heap for alloc support
    soc_esp32::init_heap();

    // initialize the SoC
    let peripherals = if cfg!(feature = "disable-esp32-radio") {
        // use default clock tickrate to save power
        esp_hal::init(esp_hal::Config::default())
    } else {
        // use max clock tickrate to support WiFI/BLE
        esp_hal::init(esp_hal::Config::default().with_cpu_clock(esp_hal::clock::CpuClock::max()))
    };

    // initialize logging levels
    esp_println::logger::init_logger_from_env();

    // initialize RTOS
    debug!("initializing RTOS...");
    use esp_hal::timer::timg::TimerGroup;
    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_rtos::start(timg0.timer0, peripherals.FROM_CPU_INTR0);
    // TODO by default idle hook simply runs WFI - but perhaps we want to do more to save power?
    // esp_rtos::start_with_idle_hook(timg0.timer0, sw_int.software_interrupt0, idle_hook);

    // intialize global state
    debug!("initializing global state...");
    // create globally shared state
    let state = enmesh_firmware::State {
        firmware_version: env!("CARGO_PKG_VERSION"),
        hardware_name: "Heltec", // FIXME provide a more descriptive string
        ..Default::default()
    };
    let global_state = enmesh_firmware::STATE.init(RwLock::new(state));

    // Map the hardware interfaces to peripherals
    //--------------------------------------------------------------------------------
    debug!("creating LoRa peripheral interface...");
    #[cfg(feature = "wireless_stick_v3")]
    let lora_io = tasks::lora::LoraIo {
        reset: OutputPin!(peripherals.GPIO7),
        dio: InputPin!(peripherals.GPIO26),
        // FIXME no PIN identified for BUSY in schematic
        busy: InputPin!(peripherals.GPIO13),
        spi: peripherals.SPI2,
        nss: OutputPin!(peripherals.GPIO18, esp_hal::gpio::Level::High),
        sck: OutputPin!(peripherals.GPIO5),
        mosi: OutputPin!(peripherals.GPIO27),
        miso: InputPin!(peripherals.GPIO19),
    };
    #[cfg(not(feature = "wireless_stick_v3"))]
    let lora_io = tasks::lora::LoraIo {
        // use standard LoRa pins
        reset: OutputPin!(peripherals.GPIO12),
        dio: InputPin!(peripherals.GPIO14),
        busy: InputPin!(peripherals.GPIO13),
        spi: peripherals.SPI2,
        nss: OutputPin!(peripherals.GPIO8, esp_hal::gpio::Level::High),
        sck: OutputPin!(peripherals.GPIO9),
        mosi: OutputPin!(peripherals.GPIO10),
        miso: InputPin!(peripherals.GPIO11),
    };

    #[cfg(not(feature = "disable-ux"))]
    debug!("creating UX peripherals interface...");
    #[cfg(all(
        not(feature = "disable-ux"),
        feature = "wifi_lora_32-v3",
    ))]
    let ux_io = tasks::ux::UxIo {
        button: InputPin!(peripherals.GPIO0),
        led: OutputPin!(peripherals.GPIO35),
        // start with screen powered off
        n_vext_control: Some(OutputPin!(peripherals.GPIO36, esp_hal::gpio::Level::High)),
        // start screen in RESET
        n_reset: OutputPin!(peripherals.GPIO21),
        i2c: peripherals.I2C0,
        sda: esp_hal::gpio::Flex::new(peripherals.GPIO17),
        scl: esp_hal::gpio::Flex::new(peripherals.GPIO18),
    };
    #[cfg(all(
        not(feature = "disable-ux"),
        feature = "wifi_lora_32-v4",
    ))]
    let ux_io = tasks::ux::UxIo {
        button: InputPin!(peripherals.GPIO0),
        led: OutputPin!(peripherals.GPIO35),
        // start with screen powered off
        n_vext_control: Some(OutputPin!(peripherals.GPIO40, esp_hal::gpio::Level::High)),
        // start screen in RESET
        n_reset: OutputPin!(peripherals.GPIO21),
        i2c: peripherals.I2C0,
        sda: esp_hal::gpio::Flex::new(peripherals.GPIO17),
        scl: esp_hal::gpio::Flex::new(peripherals.GPIO18),
    };
    #[cfg(all(not(feature = "disable-ux"), feature = "wireless_stick_v3",))]
    let ux_io = tasks::ux::UxIo {
        button: InputPin!(peripherals.GPIO0),
        led: OutputPin!(peripherals.GPIO25),
        n_vext_control: None,
        // start screen in RESET
        n_reset: OutputPin!(peripherals.GPIO16, esp_hal::gpio::Level::High),
        i2c: peripherals.I2C0,
        sda: esp_hal::gpio::Flex::new(peripherals.GPIO4),
        scl: esp_hal::gpio::Flex::new(peripherals.GPIO15),
    };
    #[cfg(all(not(feature = "disable-ux"), feature = "wireless_paper"))]
    let ux_io = tasks::ux::UxIo {
        button: InputPin!(peripherals.GPIO0),
        led: OutputPin!(peripherals.GPIO18),
        // start with screen powered off
        n_vext_control: Some(OutputPin!(peripherals.GPIO45, esp_hal::gpio::Level::High)),
        // start screen in RESET
        n_reset: OutputPin!(peripherals.GPIO6, esp_hal::gpio::Level::High),
        n_busy: InputPin!(peripherals.GPIO7),
        spi: peripherals.SPI3,
        sdi: esp_hal::gpio::Flex::new(peripherals.GPIO2),
        clk: OutputPin!(peripherals.GPIO3),
        cs: OutputPin!(peripherals.GPIO4),
        dc: OutputPin!(peripherals.GPIO5),
    };

    // create the tasks
    //================================================================================
    debug!("starting Persisted Settings task...");
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

    debug!("starting LoRa task...");
    spawner.spawn(tasks::lora::task_lora(global_state, lora_io).unwrap());

    #[cfg(not(feature = "disable-ux"))]
    {
        debug!("starting UX task...");
        spawner.spawn(tasks::ux::task_ux(global_state, ux_io).unwrap());
    }

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

    //--------------------------------------------------------------------------------
    debug!("creating usb serial task...");
    // hold off on starting up the serial interface so that boot logging completes
    Timer::after_secs(2).await;
    // https://dl.espressif.com/dl/schematics/SCH_ESP32-S3-DevKitC-1_V1.1_20220413.pdf#page=2
    // configure_usb_serial(&peripherals.GPIO36, &peripherals.GPIO37);
    let usb_serial_io = tasks::usb_serial::UsbSerialIo {
        uart: peripherals.UART0,
        rx: InputPin!(peripherals.GPIO44),
        tx: OutputPin!(peripherals.GPIO43),
    };
    spawner.spawn(tasks::usb_serial::task_usb_serial(global_state, usb_serial_io).unwrap());
    debug!("usb serial task created");
    //--------------------------------------------------------------------------------

    info!("enmesh firmware running...");
}

/// Thread to periodically update the persistent storage
#[embassy_executor::task]
async fn task_persisted_settings(
    global_state: &'static RwLock<NoopRawMutex, enmesh_firmware::State>,
    mut persisted_settings_manager: enmesh_firmware::persisted_settings::PersistedSettingsManager,
    mut settings_partition_a: Option<enmesh_storage::Partition>,
    mut settings_partition_b: Option<enmesh_storage::Partition>,
) {
    persisted_settings_manager
        .run(
            global_state,
            settings_partition_a.as_mut(),
            settings_partition_b.as_mut(),
        )
        .await;

    error!("Persisted Settings task ended");
}
