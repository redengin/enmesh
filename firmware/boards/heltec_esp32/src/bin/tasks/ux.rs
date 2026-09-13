/// provide the shared crates via re-export
use common::*;

/// provide enmesh firmware primitives
use enmesh_firmware::prelude::*;

// provide access to esp32 hardware
use soc_esp32::*;

#[cfg(feature = "_screen-ssd1306")]
pub struct UxIo {
    pub button: esp_hal::gpio::Input<'static>,
    pub led: esp_hal::gpio::Output<'static>,
    /// LOW: powered, HIGH: disabled
    pub n_vext_control: Option<esp_hal::gpio::Output<'static>>,
    /// LOW: reset, HIGH: run
    pub n_reset: esp_hal::gpio::Output<'static>,
    // display interface
    pub i2c: esp_hal::peripherals::I2C0<'static>,
    pub sda: esp_hal::gpio::Flex<'static>,
    pub scl: esp_hal::gpio::Flex<'static>,
}

#[cfg(feature = "_screen-epd")]
pub struct UxIo {
    pub button: esp_hal::gpio::Input<'static>,
    pub led: esp_hal::gpio::Output<'static>,
    /// LOW: powered, HIGH: disabled
    pub n_vext_control: Option<esp_hal::gpio::Output<'static>>,
    /// LOW: reset, HIGH: run
    pub n_reset: esp_hal::gpio::Output<'static>,
    // display interface
    pub n_busy: esp_hal::gpio::Input<'static>,
    pub spi: esp_hal::peripherals::SPI3<'static>,
    pub sdi: esp_hal::gpio::Flex<'static>,
    pub clk: esp_hal::gpio::Output<'static>,
    pub cs: esp_hal::gpio::Output<'static>,
    pub dc: esp_hal::gpio::Output<'static>,
}

#[embassy_executor::task]
pub async fn task_ux(
    _global_state: &'static RwLock<NoopRawMutex, enmesh_firmware::State>,
    mut ux_io: UxIo,
) {
    // create the screen power controller
    let screen_power_controller = match ux_io.n_vext_control {
        Some(pin) => Some(ScreenPowerControl {
            // leave the screen powered off and in reset
            n_vext_control: pin,
            n_reset: ux_io.n_reset,
        }),
        None => {
            // if no power control, release reset
            ux_io.n_reset.set_low();
            None
        }
    };

    // create the button
    let button = button::Button::active_low(ux_io.button);

    // create the led
    let led = led::Led::active_high(ux_io.led);

    // create the screen driver
    //================================================================================
    #[cfg(feature = "_screen-ssd1306")]
    let display = display::Display::new(ux_io.i2c, ux_io.scl, ux_io.sda);
    // #[cfg(feature = "_screen-epd")]
    // TODO
    //     ux_io.busy, ux_io.spi, ux_io.sdi, ux_io.clk, ux_io.cs, ux_io.dc,

    // run UX handler
    // FIXME
    // enmesh_firmware::ux::controller::run_ssd1306(
    //     global_state,
    //     ssd1306,
    //     screen_power_control,
    //     button,
    //     led,
    // )
    // .await;

    warn!("UX task ended");
}

struct ScreenPowerControl {
    /// screen powered when LOW
    pub n_vext_control: esp_hal::gpio::Output<'static>,
    /// hold in reset mode when LOW
    pub n_reset: esp_hal::gpio::Output<'static>,
}
impl enmesh_firmware::PowerControl for ScreenPowerControl {
    /// disables screen power
    fn power_off(&mut self) {
        // disable screen power
        self.n_vext_control.set_high();
    }

    #[cfg(feature = "_screen-ssd1306")]
    /// implements power reset sequence https://cdn-shop.adafruit.com/datasheets/SSD1306.pdf#page=27Z
    async fn power_on(&mut self) {
        // enable screen power
        self.n_vext_control.set_low();
        // delay for 3 microseconds (allow power to stabilize)
        Timer::after_micros(3).await;

        // put into reset
        self.n_reset.set_low();
        // delay for 3 microseconds
        Timer::after_micros(3).await;
        // take out of reset
        self.n_reset.set_high();
    }

    #[cfg(feature = "_screen-epd")]
    /// FIXME implements power reset sequence ....
    async fn power_on(&mut self) {
        // enable screen power
        self.n_vext_control.set_low();
        // delay for 3 microseconds (allow power to stabilize)
        Timer::after_micros(3).await;

        // put into reset
        self.n_reset.set_low();
        // delay for 3 microseconds
        Timer::after_micros(3).await;
        // take out of reset
        self.n_reset.set_high();
    }
}

#[cfg(feature = "_screen-ssd1306")]
mod display {
    /// provide the shared crates via re-export
    use common::{display_interface, embedded_graphics};

    /// provide access to esp32 hardware
    use soc_esp32::*;

    /// provide access to the display driver
    use ssd1306::{Ssd1306Async, mode::BufferedGraphicsModeAsync, prelude::*};

    pub struct Display {
        driver: Ssd1306Async<
            I2CInterface<soc_esp32::esp_hal::i2c::master::I2c<'static, esp_hal::Async>>,
            DisplaySize128x64,
            BufferedGraphicsModeAsync<DisplaySize128x64>,
        >,
    }
    impl Display {
        pub fn new(
            i2c: esp_hal::peripherals::I2C0<'static>,
            mut sda: esp_hal::gpio::Flex<'static>,
            mut scl: esp_hal::gpio::Flex<'static>,
        ) -> Self {
            // configure sda, scl Flex pins to support I2C
            sda.apply_output_config(
                &esp_hal::gpio::OutputConfig::default()
                    .with_drive_mode(esp_hal::gpio::DriveMode::OpenDrain),
            );
            sda.set_input_enable(true);
            sda.set_output_enable(true);
            scl.apply_output_config(
                &esp_hal::gpio::OutputConfig::default()
                    .with_drive_mode(esp_hal::gpio::DriveMode::OpenDrain),
            );
            scl.set_input_enable(true);
            scl.set_output_enable(true);

            // create the i2c bus
            let i2c_bus = esp_hal::i2c::master::I2c::new(
                i2c,
                esp_hal::i2c::master::Config::default()
                    .with_frequency(esp_hal::time::Rate::from_mhz(1)), // suggested rate from ssd1306
            )
            .unwrap()
            .with_sda(sda)
            .with_scl(scl)
            .into_async();

            // create the driver instance
            let driver = ssd1306::Ssd1306Async::new(
                ssd1306::I2CDisplayInterface::new(i2c_bus),
                ssd1306::size::DisplaySize128x64,
                ssd1306::rotation::DisplayRotation::Rotate0,
            )
            .into_buffered_graphics_mode();

            Self { driver }
        }
    }
    impl embedded_graphics::draw_target::DrawTarget for Display {
        type Color = embedded_graphics::pixelcolor::BinaryColor;
        type Error = display_interface::DisplayError;

        fn fill_contiguous<I>(
            &mut self,
            area: &embedded_graphics::primitives::Rectangle,
            colors: I,
        ) -> Result<(), Self::Error>
        where
            I: IntoIterator<Item = Self::Color>,
        {
            self.driver.fill_contiguous(area, colors)
        }

        fn fill_solid(
            &mut self,
            area: &embedded_graphics::primitives::Rectangle,
            color: Self::Color,
        ) -> Result<(), Self::Error> {
            self.driver.fill_solid(area, color)
        }

        fn clear(&mut self, color: Self::Color) -> Result<(), Self::Error> {
            self.driver.clear(color)
        }

        fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
        where
            I: IntoIterator<Item = embedded_graphics::prelude::Pixel<Self::Color>>,
        {
            self.driver.draw_iter(pixels)
        }
    }
    impl embedded_graphics::geometry::Dimensions for Display {
        fn bounding_box(&self) -> embedded_graphics::primitives::Rectangle {
            self.driver.bounding_box()
        }
    }
    impl enmesh_firmware::ux::BufferedDisplay for Display {
        async fn flush(&mut self) -> Result<(), display_interface::DisplayError> {
            self.driver.flush().await
        }
    }
}

#[cfg(feature = "_screen-epd")]
mod display {
    /// provide the shared crates via re-export
    use common::*;

    /// provide enmesh firmware primitives
    use enmesh_firmware::prelude::*;

    /// provide access to esp32 hardware
    use soc_esp32::*;

    pub struct Display {
        // display: bool,
    }
    impl Display {
        pub fn new(
            n_busy: esp_hal::gpio::Input<'static>,
            n_reset: esp_hal::gpio::Output<'static>,
            spi: esp_hal::peripherals::SPI3<'static>,
            sdi: esp_hal::gpio::Flex<'static>,
            clk: esp_hal::gpio::Output<'static>,
            cs: esp_hal::gpio::Output<'static>,
            dc: esp_hal::gpio::Output<'static>,
        ) -> Self {
            // create SPI bus
            let spi_bus = esp_hal::spi::master::Spi::new(
                spi,
                esp_hal::spi::master::Config::default()
                    .with_frequency(esp_hal::time::Rate::from_mhz(20)),
            )
            .unwrap()
            .with_sck(clk)
            .with_mosi(sdi)
            .into_async();

            // create SPI interface for display
            let spi_interface = display_interface_spi::SPIInterface::new(
                embedded_hal_bus::spi::ExclusiveDevice::new_no_delay(spi_bus, cs).unwrap(),
                dc,
            );

            // create display interface
            // FIXME
            // let display_interface =
            //     epd_rs::EpdInterface::new(spi_interface, n_busy, n_reset, Delay::new());

            // // create driver
            // let driver = epd_rs::drivers::E0213A367::new(display_interface).unwrap();

            Self {
            // let spi_bus = display_interface_spi::SPIInterface::new(
            //     embedded_hal_bus::spi::ExclusiveDevice::new_no_delay(
            //         spi,
            //         cs,
            //     )
            //     .unwrap(),
            //     dc,
            // );

            // let display_interface =
            //     epd_rs::EpdInterface::new(spi_bus, n_busy, n_reset, embassy_time::Delay::new());

            // let driver = epd_rs::drivers::E0213A367::new(display_interface).unwrap();

            // // create the display
            // let display = epd_rs::EpdDrawTarget::new(driver, epd_rs::DisplayRotation::Rotate270);

                // display
            }
        }
    }
    // impl embedded_graphics::draw_target::DrawTarget for Display {
    //     type Color = embedded_graphics::pixelcolor::BinaryColor;
    //     type Error = display_interface::DisplayError;

    //     fn fill_contiguous<I>(
    //         &mut self,
    //         area: &embedded_graphics::primitives::Rectangle,
    //         colors: I,
    //     ) -> Result<(), Self::Error>
    //     where
    //         I: IntoIterator<Item = Self::Color>,
    //     {
    //         self.driver.fill_contiguous(area, colors)
    //     }

    //     fn fill_solid(
    //         &mut self,
    //         area: &embedded_graphics::primitives::Rectangle,
    //         color: Self::Color,
    //     ) -> Result<(), Self::Error> {
    //         self.driver.fill_solid(area, color)
    //     }

    //     fn clear(&mut self, color: Self::Color) -> Result<(), Self::Error> {
    //         self.driver.clear(color)
    //     }

    //     fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    //     where
    //         I: IntoIterator<Item = embedded_graphics::prelude::Pixel<Self::Color>>,
    //     {
    //         self.driver.draw_iter(pixels)
    //     }
    // }
    // impl embedded_graphics::geometry::Dimensions for Display {
    //     fn bounding_box(&self) -> embedded_graphics::primitives::Rectangle {
    //         self.driver.bounding_box()
    //     }
    // }
    // impl enmesh_firmware::ux::BufferedDisplay for Display {
    //     async fn flush(&mut self) -> Result<(), display_interface::DisplayError> {
    //         self.driver.flush().await
    //     }
    // }
}

// #[cfg(feature = "_screen-ssd1306")]
// fn create_display(
//     mut i2c: esp_hal::peripherals::I2C0<'static>,
//     mut sda: esp_hal::gpio::Flex<'static>,
//     mut scl: esp_hal::gpio::Flex<'static>,
// ) {
//     #[cfg(feature = "_screen-ssd1306")]
//     {}
//     #[cfg(feature = "_screen-ssd1306")]
//     let display_interface = ssd1306::I2CDisplayInterface::new(
//         // create the i2c bus
//         esp_hal::i2c::master::I2c::new(
//             i2c,
//             esp_hal::i2c::master::Config::default()
//                 .with_frequency(esp_hal::time::Rate::from_mhz(1)), // suggested rate from ssd1306
//         )
//         .unwrap()
//         .with_sda(sda)
//         .with_scl(scl)
//         .into_async(),
//     );
//     // display
// }

#[cfg(feature = "_screen-epd")]
fn create_display(
    busy: esp_hal::gpio::Input<'static>,
    spi: esp_hal::peripherals::SPI3<'static>,
    sdi: esp_hal::gpio::Flex<'static>,
    clk: esp_hal::gpio::Output<'static>,
    cs: esp_hal::gpio::Output<'static>,
    dc: esp_hal::gpio::Output<'static>,
) {
}

// #[cfg(feature="_screen-ssd1306")]
// /// provide support for CMOS OLED SSD1306 screens
// pub(crate) mod screen_ssd1306 {
//     // provide the shared crates via re-export
//     use common::*;

//     // provide logging primitives
//     use log::*;

//     // provide access to esp32 hardware
//     use soc_esp32::*;

//     // provide scheduling primitives
//     use enmesh_firmware::prelude::*;

//     #[allow(dead_code)]
//     /// convenience struct for the screen and button interfaces
//     pub struct UxIo {
//         pub vext_control: esp_hal::gpio::Output<'static>,
//         pub oled_reset: esp_hal::gpio::Output<'static>,
//         pub i2c: esp_hal::peripherals::I2C0<'static>,
//         pub sda: esp_hal::gpio::Flex<'static>,
//         pub scl: esp_hal::gpio::Flex<'static>,
//         pub button: esp_hal::gpio::Input<'static>,
//         pub led: esp_hal::gpio::Output<'static>,
//     }

//     #[embassy_executor::task]
//     pub async fn task_ux(
//         global_state: &'static RwLock<NoopRawMutex, enmesh_firmware::State>,
//         mut ux_io: UxIo,
//     ) {
//         debug!("initializing UX...");
//         // create the screen driver
//         //================================================================================
//         // configure sda, scl Flex pins to support I2C
//         ux_io.sda.apply_output_config(
//             &esp_hal::gpio::OutputConfig::default()
//                 .with_drive_mode(esp_hal::gpio::DriveMode::OpenDrain),
//         );
//         ux_io.sda.set_input_enable(true);
//         ux_io.sda.set_output_enable(true);
//         ux_io.scl.apply_output_config(
//             &esp_hal::gpio::OutputConfig::default()
//                 .with_drive_mode(esp_hal::gpio::DriveMode::OpenDrain),
//         );
//         ux_io.scl.set_input_enable(true);
//         ux_io.scl.set_output_enable(true);
//         let interface = ssd1306::I2CDisplayInterface::new(
//             // create the i2c bus
//             esp_hal::i2c::master::I2c::new(
//                 ux_io.i2c,
//                 esp_hal::i2c::master::Config::default()
//                     .with_frequency(esp_hal::time::Rate::from_mhz(1)), // suggested rate from ssd1306
//             )
//             .unwrap()
//             .with_sda(ux_io.sda)
//             .with_scl(ux_io.scl)
//             .into_async(),
//         );
//         // TODO as i2C bus is already async, do we need an async screen driver?
//         // let mut display = ssd1306::Ssd1306Async::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
//         let ssd1306 = ssd1306::Ssd1306::new(
//             interface,
//             ssd1306::size::DisplaySize128x64,
//             ssd1306::rotation::DisplayRotation::Rotate0,
//         )
//         .into_buffered_graphics_mode();
//         //================================================================================

//         // create the screen power controller
//         let screen_power_control = ScreenPowerControl {
//             vext_control: ux_io.vext_control,
//             reset: ux_io.oled_reset,
//         };

//         // create the button
//         let button = button::Button::active_low(ux_io.button);

//         // create the led
//         let led = led::Led::active_high(ux_io.led);

//         // run UX handler
//         // FIXME
//         // enmesh_firmware::ux::controller::run_ssd1306(
//         //     global_state,
//         //     ssd1306,
//         //     screen_power_control,
//         //     button,
//         //     led,
//         // )
//         // .await;

//         warn!("UX task ended");
//     }

//     #[allow(dead_code)]
//     pub struct ScreenPowerControl {
//         /// screen powered when LOW
//         pub vext_control: esp_hal::gpio::Output<'static>,
//         /// hold in reset mode when LOW
//         pub reset: esp_hal::gpio::Output<'static>,
//     }

//     impl enmesh_firmware::PowerControl for ScreenPowerControl {
//         /// disables screen power
//         fn power_off(&mut self) {
//             // disable screen power
//             self.vext_control.set_high();
//         }

//         /// implements power reset sequence https://cdn-shop.adafruit.com/datasheets/SSD1306.pdf#page=27Z
//         /// POST: user should turn on and clear the display
//         /// ```
//         ///     display.init().unwrap();
//         ///     // clear the display (requires flush to take effect)
//         ///     display.clear_buffer();
//         ///     display.flush().unwrap();
//         /// ```
//         async fn power_on(&mut self) {
//             // enable screen power
//             self.vext_control.set_low();
//             // delay for 3 microseconds (allow power to stabilize)
//             Timer::after_micros(3).await;

//             // put into reset
//             self.reset.set_low();
//             // delay for 3 microseconds
//             Timer::after_micros(3).await;
//             // take out of reset
//             self.reset.set_high();
//         }
//     }
// }

// pub(crate) mod screen_ssd1680 {
//     // provide the shared crates via re-export
//     // use common::{embassy_time::Delay, *};

//     // provide logging primitives
//     // use log::*;

//     // provide access to esp32 hardware
//     use soc_esp32::*;

//     // provide scheduling primitives
//     use common::*;
//     use embassy_sync::mutex::Mutex;
//     use enmesh_firmware::prelude::*;

//     /// static LoRa radio SPI bus
//     static SSD1680_SPI_BUS: static_cell::StaticCell<
//         Mutex<NoopRawMutex, esp_hal::spi::master::Spi<'static, esp_hal::Async>>,
//     > = static_cell::StaticCell::new();

//     #[allow(dead_code)]
//     /// convenience struct for the screen and button interfaces
//     pub struct UxIo {
//         pub spi: esp_hal::peripherals::SPI3<'static>,
//         pub sdi: esp_hal::gpio::Flex<'static>,
//         pub clk: esp_hal::gpio::Output<'static>,
//         pub cs: esp_hal::gpio::Output<'static>,
//         pub dc: esp_hal::gpio::Output<'static>,
//         pub reset: esp_hal::gpio::Output<'static>,
//         pub busy: esp_hal::gpio::Input<'static>,
//         pub vext_control: esp_hal::gpio::Output<'static>,
//         pub button: esp_hal::gpio::Input<'static>,
//         pub led: esp_hal::gpio::Output<'static>,
//     }

//     #[embassy_executor::task]
//     pub async fn task_ux(
//         _global_state: &'static RwLock<NoopRawMutex, enmesh_firmware::State>,
//         ux_io: UxIo,
//     ) {
//         debug!("initializing UX...");

//         // create the SPI bus
//         const SSD1680_SPI_MHZ: u32 = 16; // recommended SPI frequency
//         let spi = esp_hal::spi::master::Spi::new(
//             ux_io.spi,
//             esp_hal::spi::master::Config::default()
//                 .with_frequency(esp_hal::time::Rate::from_mhz(SSD1680_SPI_MHZ))
//                 .with_mode(esp_hal::spi::Mode::_0),
//         )
//         .unwrap()
//         .with_sck(ux_io.clk)
//         // .with_sio0(ux_io.sdi)
//         .with_mosi(ux_io.sdi)
//         .into_async();
//         let spi_bus = SSD1680_SPI_BUS.init(Mutex::new(spi));
//         let _spi_device =
//             embassy_embedded_hal::shared_bus::asynch::spi::SpiDevice::new(spi_bus, ux_io.cs);

//         // create the screen driver
//         // FIXME ssd1680 driver is too old
//         // let ssd1680 = match ssd1680::driver::Ssd1680::new(
//         //     spi_device,
//         //     ux_io.busy,
//         //     ux_io.dc,
//         //     ux_io.reset,
//         //     &mut embassy_time::Delay,
//         // ) {
//         //     Ok(driver) => driver,
//         //     Err(e) => {
//         //         error!("failed to initialize screen: {:?}", e);
//         //         return;
//         //     }
//         // };

//         // create the screen power controller
//         let _screen_power_control = ScreenPowerControl {
//             vext_control: ux_io.vext_control,
//             reset: ux_io.reset,
//         };

//         // create the button
//         let _button = button::Button::active_low(ux_io.button);

//         // create the led
//         let _led = led::Led::active_high(ux_io.led);

//         // run UX handler
//         // enmesh_firmware::ux::controller::run(
//         //     global_state,
//         //     ssd1306,
//         //     screen_power_control,
//         //     button,
//         //     led,
//         // )
//         // .await;

//         warn!("UX task ended");
//     }

//     pub struct ScreenPowerControl {
//         /// screen powered when LOW
//         pub vext_control: esp_hal::gpio::Output<'static>,
//         /// hold in reset mode when LOW
//         pub reset: esp_hal::gpio::Output<'static>,
//     }

//     impl enmesh_firmware::PowerControl for ScreenPowerControl {
//         /// disables screen power
//         fn power_off(&mut self) {
//             // disable screen power
//             self.vext_control.set_high();
//         }

//         /// FIXME implement for ssd1680
//         /// implements power reset sequence https://cdn-shop.adafruit.com/datasheets/SSD1306.pdf#page=27Z
//         /// POST: user should turn on and clear the display
//         /// ```
//         ///     display.init().unwrap();
//         ///     // clear the display (requires flush to take effect)
//         ///     display.clear_buffer();
//         ///     display.flush().unwrap();
//         /// ```
//         async fn power_on(&mut self) {
//             // enable screen power
//             self.vext_control.set_low();
//             // delay for 3 microseconds (allow power to stabilize)
//             Timer::after_micros(3).await;

//             // put into reset
//             self.reset.set_low();
//             // delay for 3 microseconds
//             Timer::after_micros(3).await;
//             // take out of reset
//             self.reset.set_high();
//         }
//     }
// }
