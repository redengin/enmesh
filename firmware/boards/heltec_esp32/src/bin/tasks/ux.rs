/// provide support for CMOS OLED SSD1306 screens
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
        pub sda: esp_hal::gpio::Flex<'static>,
        pub scl: esp_hal::gpio::Flex<'static>,
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
        // configure sda, scl Flex pins to support I2C
        ux_io.sda.apply_output_config(
            &esp_hal::gpio::OutputConfig::default()
                .with_drive_mode(esp_hal::gpio::DriveMode::OpenDrain),
        );
        ux_io.sda.set_input_enable(true);
        ux_io.sda.set_output_enable(true);
        ux_io.scl.apply_output_config(
            &esp_hal::gpio::OutputConfig::default()
                .with_drive_mode(esp_hal::gpio::DriveMode::OpenDrain),
        );
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
        let ssd1306 = ssd1306::Ssd1306::new(
            interface,
            ssd1306::size::DisplaySize128x64,
            ssd1306::rotation::DisplayRotation::Rotate0,
        )
        .into_buffered_graphics_mode();
        //================================================================================

        // create the screen power controller
        let screen_power_control = ScreenPowerControl {
            vext_control: ux_io.vext_control,
            reset: ux_io.oled_reset,
        };

        // create the button
        let button = button::Button::active_low(ux_io.button);

        // create the led
        let led = led::Led::active_high(ux_io.led);

        // run UX handler
        enmesh_firmware::ux::controller::run_ssd1306(
            global_state,
            ssd1306,
            screen_power_control,
            button,
            led,
        )
        .await;

        warn!("UX task ended");
    }

    pub struct ScreenPowerControl {
        /// screen powered when LOW
        pub vext_control: esp_hal::gpio::Output<'static>,
        /// hold in reset mode when LOW
        pub reset: esp_hal::gpio::Output<'static>,
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
            self.reset.set_low();
            // delay for 3 microseconds
            Timer::after_micros(3).await;
            // take out of reset
            self.reset.set_high();
        }
    }
}

pub(crate) mod screen_ssd1680 {
    // provide the shared crates via re-export
    use common::{embassy_time::Delay, *};

    // provide logging primitives
    use log::*;

    // provide access to esp32 hardware
    use soc_esp32::*;

    // provide scheduling primitives
    use embassy_sync::mutex::Mutex;
    use enmesh_firmware::prelude::*;

    /// static LoRa radio SPI bus
    static SSD1680_SPI_BUS: static_cell::StaticCell<
        Mutex<NoopRawMutex, esp_hal::spi::master::Spi<'static, esp_hal::Async>>,
    > = static_cell::StaticCell::new();

    /// convenience struct for the screen and button interfaces
    pub struct UxIo {
        pub spi: esp_hal::peripherals::SPI3<'static>,
        pub sdi: esp_hal::gpio::Flex<'static>,
        pub clk: esp_hal::gpio::Output<'static>,
        pub cs: esp_hal::gpio::Output<'static>,
        pub dc: esp_hal::gpio::Output<'static>,
        pub reset: esp_hal::gpio::Output<'static>,
        pub busy: esp_hal::gpio::Input<'static>,
        pub vext_control: esp_hal::gpio::Output<'static>,
        pub button: esp_hal::gpio::Input<'static>,
        pub led: esp_hal::gpio::Output<'static>,
    }

    #[embassy_executor::task]
    pub async fn task_ux(
        global_state: &'static RwLock<NoopRawMutex, enmesh_firmware::State>,
        mut ux_io: UxIo,
    ) {
        debug!("initializing UX...");

        // create the SPI bus
        const SSD1680_SPI_MHZ: u32 = 16; // recommended SPI frequency
        let spi = esp_hal::spi::master::Spi::new(
            ux_io.spi,
            esp_hal::spi::master::Config::default()
                .with_frequency(esp_hal::time::Rate::from_mhz(SSD1680_SPI_MHZ))
                .with_mode(esp_hal::spi::Mode::_0),
        )
        .unwrap()
        .with_sck(ux_io.clk)
        // .with_sio0(ux_io.sdi)
        .with_mosi(ux_io.sdi)
        .into_async();
        let spi_bus = SSD1680_SPI_BUS.init(Mutex::new(spi));
        let spi_device =
            embassy_embedded_hal::shared_bus::asynch::spi::SpiDevice::new(spi_bus, ux_io.cs);

        // create the screen driver
        // FIXME ssd1680 driver is too old
        // let ssd1680 = match ssd1680::driver::Ssd1680::new(
        //     spi_device,
        //     ux_io.busy,
        //     ux_io.dc,
        //     ux_io.reset,
        //     &mut embassy_time::Delay,
        // ) {
        //     Ok(driver) => driver,
        //     Err(e) => {
        //         error!("failed to initialize screen: {:?}", e);
        //         return;
        //     }
        // };

        // create the screen power controller
        let screen_power_control = ScreenPowerControl {
            vext_control: ux_io.vext_control,
            reset: ux_io.reset,
        };

        // create the button
        let button = button::Button::active_low(ux_io.button);

        // create the led
        let led = led::Led::active_high(ux_io.led);

        // run UX handler
        // enmesh_firmware::ux::controller::run(
        //     global_state,
        //     ssd1306,
        //     screen_power_control,
        //     button,
        //     led,
        // )
        // .await;

        warn!("UX task ended");
    }

    pub struct ScreenPowerControl {
        /// screen powered when LOW
        pub vext_control: esp_hal::gpio::Output<'static>,
        /// hold in reset mode when LOW
        pub reset: esp_hal::gpio::Output<'static>,
    }

    impl enmesh_firmware::PowerControl for ScreenPowerControl {
        /// disables screen power
        fn power_off(&mut self) {
            // disable screen power
            self.vext_control.set_high();
        }

        /// FIXME implement for ssd1680
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
            self.reset.set_low();
            // delay for 3 microseconds
            Timer::after_micros(3).await;
            // take out of reset
            self.reset.set_high();
        }
    }
}
