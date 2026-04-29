// provide the shared crates via re-export
use common::*;
use soc_esp32::*;

// provide logging primitives
use log::*;

/// convenience struct for the screen and button interfaces
pub struct UxIo {
    /// screen powered when LOW
    pub vext_control: esp_hal::peripherals::GPIO36<'static>,
    /// hold in reset mode when LOW
    pub oled_reset: esp_hal::peripherals::GPIO21<'static>,
    pub i2c: esp_hal::peripherals::I2C0<'static>,
    pub sda: esp_hal::peripherals::GPIO17<'static>,
    pub scl: esp_hal::peripherals::GPIO18<'static>,
    /// LOW when pressed, else HIGH
    pub button: esp_hal::peripherals::GPIO0<'static>,
    pub led: esp_hal::peripherals::GPIO35<'static>,
}
#[embassy_executor::task]
pub async fn task_ux(ux_io: UxIo) {
    debug!("initializing user interface...");

    // create the screen driver
    //================================================================================
    let interface = ssd1306::I2CDisplayInterface::new(
        // create the i2c bus
        esp_hal::i2c::master::I2c::new(
            ux_io.i2c,
            esp_hal::i2c::master::Config::default()
                .with_frequency(esp_hal::time::Rate::from_mhz(1)), // suggested rate from ssd1306
        )
        .unwrap()
        .with_sda(ux_io.sda)
        .with_scl(ux_io.scl)
        .into_async(),
    );
    use ssd1306::{Ssd1306, prelude::*};
    // TODO as i2C bus is already async, do we need an async screen driver?
    // let mut display = ssd1306::Ssd1306Async::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
    let ssd1306 = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    //================================================================================

    // create the screen power controller
    let screen_power_control = enmesh_firmware_heltec_v3::ScreenPowerControl {
        vext_control: esp_hal::gpio::Output::new(
            ux_io.vext_control,
            esp_hal::gpio::Level::Low, // disable power by setting HIGH
            esp_hal::gpio::OutputConfig::default(),
        ),
        oled_reset: esp_hal::gpio::Output::new(
            ux_io.oled_reset,
            esp_hal::gpio::Level::High, // put into reset by setting LOW
            esp_hal::gpio::OutputConfig::default(),
        ),
    };

    // create the button
    let button = common::Button::active_low(esp_hal::gpio::Input::new(
        ux_io.button,
        esp_hal::gpio::InputConfig::default(),
    ));

    // create the led
    let led = esp_hal::gpio::Output::new(
        ux_io.led,
        esp_hal::gpio::Level::Low,
        esp_hal::gpio::OutputConfig::default(),
    );

    // run UX handler
    info!("UX task started");
    enmesh_firmware::ux::ssd1306::run(ssd1306, screen_power_control, button, led).await;

    warn!("UX task ended");
}
