// provide the shared crates via re-export
use common::*;

// provide access to esp32 hardware
use soc_esp32::*;

// provide logging primitives
use log::*;

// provide scheduling primitives
use enmesh_firmware::prelude::*;

/// convenience struct for the screen and button interfaces
pub struct UxIo {
    pub vext_control: esp_hal::gpio::Output<'static>,
    pub oled_reset: esp_hal::gpio::Output<'static>,
    pub i2c: esp_hal::peripherals::I2C0<'static>,
    pub sda: esp_hal::gpio::Flex<'static>,
    pub scl: esp_hal::gpio::Flex<'static>,
    pub button: esp_hal::gpio::Input<'static>,
    pub led: esp_hal::gpio::Output<'static>,
}
#[embassy_executor::task]
pub async fn task_ux(
    global_state: &'static RwLock<NoopRawMutex, enmesh_firmware::State>,
    ux_io: UxIo,
) {
    debug!("initializing UX...");
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
    let screen_power_control = enmesh_firmware_heltec_esp32::ScreenPowerControl {
        vext_control: ux_io.vext_control,
        oled_reset: ux_io.oled_reset,
    };

    // create the button
    let button = button::Button::active_low(ux_io.button);

    // create the led
    let led = led::Led::active_high(ux_io.led);

    // run UX handler
    enmesh_firmware::ux::ssd1306::run(global_state, ssd1306, screen_power_control, button, led).await;

    warn!("UX task ended");
}
