


pub(crate) mod screen_ssd1306 {
    // provide the shared crates via re-export
    use common::*;

    // provide logging primitives
    use log::*;

    // provide access to esp32 hardware
    use soc_esp32::*;

    // provide scheduling primitives
    use enmesh_firmware::prelude::*;

    /// convenience struct for the screen and button interfaces
    pub struct UxIo {
        pub vext_control: esp_hal::gpio::Output<'static>,
        pub oled_reset: esp_hal::gpio::Output<'static>,
        pub i2c: esp_hal::peripherals::I2C0<'static>,
        // FIXME
        pub sda: esp_hal::gpio::Flex<'static>,
        pub scl: esp_hal::gpio::Flex<'static>,
        // pub sda: esp_hal::peripherals::GPIO17<'static>,
        // pub scl: esp_hal::peripherals::GPIO18<'static>,
        pub button: esp_hal::gpio::Input<'static>,
        pub led: esp_hal::gpio::Output<'static>,
    }

    #[embassy_executor::task]
    pub async fn task_ux(
        global_state: &'static RwLock<NoopRawMutex, enmesh_firmware::State>,
        mut ux_io: UxIo,
    ) {
        debug!("initializing UX...");
        // create the screen driver
        //================================================================================
        // transmute the flex to support I2C
        ux_io.sda.apply_output_config(&esp_hal::gpio::OutputConfig::default().with_drive_mode(esp_hal::gpio::DriveMode::OpenDrain));
        ux_io.sda.set_input_enable(true);
        ux_io.sda.set_output_enable(true);
        ux_io.scl.apply_output_config(&esp_hal::gpio::OutputConfig::default().with_drive_mode(esp_hal::gpio::DriveMode::OpenDrain));
        ux_io.scl.set_input_enable(true);
        ux_io.scl.set_output_enable(true);
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
        // TODO as i2C bus is already async, do we need an async screen driver?
        // let mut display = ssd1306::Ssd1306Async::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        let ssd1306 = ssd1306::Ssd1306::new(interface, 
            ssd1306::size::DisplaySize128x64, ssd1306::rotation::DisplayRotation::Rotate0)
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
        enmesh_firmware::ux::controller::run_ssd1306(global_state, ssd1306, screen_power_control, button, led)
            .await;

        warn!("UX task ended");
    }
}
