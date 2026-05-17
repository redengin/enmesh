#![no_std]

// provide the shared crates via re-export
use common::*;
use soc_esp32::*;

// provice scheduling primitives
use embassy_time::Timer;

pub struct ScreenPowerControl {
    /// screen powered when LOW
    pub vext_control: esp_hal::gpio::Output<'static>,
    /// hold in reset mode when LOW
    pub oled_reset: esp_hal::gpio::Output<'static>,
}

impl enmesh_firmware::PowerControl for ScreenPowerControl {
    /// disables screen power
    fn power_off(&mut self) {
        // disable screen power
        self.vext_control.set_high();
    }

    /// implements power reset sequence https://cdn-shop.adafruit.com/datasheets/SSD1306.pdf#page=27Z
    /// POST: user should turn on and clear the display
    /// ```
    ///     display.init().unwrap();
    ///     // clear the display (requires flush to take effect)
    ///     display.clear_buffer();
    ///     display.flush().unwrap();
    /// ```
    async fn power_on(&mut self) {
        // enable screen power
        self.vext_control.set_low();
        // delay for 3 microseconds (allow power to stabilize)
        Timer::after_micros(3).await;

        // put into reset
        self.oled_reset.set_low();
        // delay for 3 microseconds
        Timer::after_micros(3).await;
        // take out of reset
        self.oled_reset.set_high();
    }
}