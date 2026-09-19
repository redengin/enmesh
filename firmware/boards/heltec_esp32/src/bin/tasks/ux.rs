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
    // display interface
    /// LOW: reset, HIGH: run
    pub n_reset: esp_hal::gpio::Output<'static>,
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
    // display interface
    /// LOW: reset, HIGH: run
    pub n_reset: esp_hal::gpio::Output<'static>,
    /// LOW: busy, HIGH: idle
    pub n_busy: esp_hal::gpio::Input<'static>,
    pub spi: esp_hal::peripherals::SPI3<'static>,
    pub sdi: esp_hal::gpio::Flex<'static>,
    pub clk: esp_hal::gpio::Output<'static>,
    pub cs: esp_hal::gpio::Output<'static>,
    pub dc: esp_hal::gpio::Output<'static>,
}

#[embassy_executor::task]
pub async fn task_ux(
    global_state: &'static RwLock<NoopRawMutex, enmesh_firmware::State>,
    ux_io: UxIo,
) {
    // create the button
    let button = button::Button::active_low(ux_io.button);

    // create the led
    let led = led::Led::active_high(ux_io.led);

    // create the screen driver
    //================================================================================
    #[cfg(feature = "_screen-ssd1306")]
    let display = display::Display::new(
        ux_io.n_vext_control,
        ux_io.n_reset,
        ux_io.i2c,
        ux_io.scl,
        ux_io.sda,
    );
    #[cfg(feature = "_screen-epd")]
    let display = display::Display::new(
        ux_io.n_vext_control,
        ux_io.n_reset,
        ux_io.n_busy,
        ux_io.spi,
        ux_io.sdi,
        ux_io.clk,
        ux_io.cs,
        ux_io.dc,
    )
    .await
    .unwrap();

    // run UX handler
    enmesh_firmware::ux::binary_color::run(global_state, display, button, led).await;

    error!("UX task ended");
}

#[cfg(feature = "_screen-ssd1306")]
mod display {
    /// provide the shared crates via re-export
    use common::*;

    /// provide logging prmititives
    use log::*;
    const TAG: &str = "[SSD1306]";

    /// provide enmesh firmware primitives
    use enmesh_firmware::prelude::*;

    /// provide access to esp32 hardware
    use soc_esp32::*;

    /// provide access to the display driver
    use ssd1306::prelude::*;
    // use ssd1306::{Ssd1306Async, mode::BufferedGraphicsModeAsync};
    use ssd1306::{Ssd1306, mode::BufferedGraphicsMode};

    pub struct Display {
        n_vext_control: Option<esp_hal::gpio::Output<'static>>,
        n_reset: esp_hal::gpio::Output<'static>,
        /// FIXME type is overspecified
        // display: Ssd1306Async<
        //     I2CInterface<soc_esp32::esp_hal::i2c::master::I2c<'static, esp_hal::Async>>,
        //     DisplaySize128x64,
        //     BufferedGraphicsModeAsync<DisplaySize128x64>,
        // >,
        // display: Ssd1306Async<
        display: ssd1306::Ssd1306<
            I2CInterface<soc_esp32::esp_hal::i2c::master::I2c<'static, esp_hal::Async>>,
            DisplaySize128x64,
            BufferedGraphicsMode<DisplaySize128x64>,
        >,
    }
    impl Display {
        pub fn new(
            n_vext_control: Option<esp_hal::gpio::Output<'static>>,
            n_reset: esp_hal::gpio::Output<'static>,
            i2c: esp_hal::peripherals::I2C0<'static>,
            mut sda: esp_hal::gpio::Flex<'static>,
            mut scl: esp_hal::gpio::Flex<'static>,
        ) -> Self {
            trace!("{TAG} configuring FLEX sda...");
            // configure sda, scl Flex pins to support I2C
            sda.apply_output_config(
                &esp_hal::gpio::OutputConfig::default()
                    .with_drive_mode(esp_hal::gpio::DriveMode::OpenDrain),
            );
            sda.set_input_enable(true);
            sda.set_output_enable(true);

            trace!("{TAG} configuring FLEX scl...");
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
            // let display = ssd1306::Ssd1306Async::new(
            let display = ssd1306::Ssd1306::new(
                ssd1306::I2CDisplayInterface::new(i2c_bus),
                ssd1306::size::DisplaySize128x64,
                ssd1306::rotation::DisplayRotation::Rotate0,
            )
            .into_buffered_graphics_mode();

            Self {
                n_vext_control,
                n_reset,
                display,
            }
        }
    }

    /// provide the shared crates via re-export
    use common::{display_interface, embedded_graphics};

    use embedded_hal_async::delay::DelayNs;

    impl enmesh_firmware::PowerControl for Display {
        fn power_off(&mut self) {
            trace!("{TAG} powering off..");
            // disable power
            if let Some(pin) = &mut self.n_vext_control {
                pin.set_high();
            }
        }

        #[allow(async_fn_in_trait)] // usage should never use Send()
        /// must reinitialize the hardware as necessary
        async fn power_on(&mut self) {
            trace!("{TAG} powering on..");
            // enable power
            if let Some(pin) = &mut self.n_vext_control {
                pin.set_low();
            }
            Delay.delay_ms(10).await;

            // place chip into RESET
            self.n_reset.set_low();
            Delay.delay_ms(10).await;

            // take chip out of RESET
            self.n_reset.set_high();
            Delay.delay_ms(1000).await;

            // intialize the display driver
            trace!("{TAG} initializing display driver..");
            // let _ = self.display.init().await
            let _ = self.display.init()
                .map_err(|e| error!("{TAG} failed to initialize display: {:?}", e));
        }
    }

    /// expose internal embedded_graphics support
    impl embedded_graphics::draw_target::DrawTarget for Display {
        type Color = embedded_graphics::pixelcolor::BinaryColor;
        type Error = display_interface::DisplayError;

        fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
        where
            I: IntoIterator<Item = embedded_graphics::prelude::Pixel<Self::Color>>,
        {
            self.display.draw_iter(pixels)
        }

        fn fill_contiguous<I>(
            &mut self,
            area: &embedded_graphics::primitives::Rectangle,
            colors: I,
        ) -> Result<(), Self::Error>
        where
            I: IntoIterator<Item = Self::Color>,
        {
            self.display.fill_contiguous(area, colors)
        }

        fn fill_solid(
            &mut self,
            area: &embedded_graphics::primitives::Rectangle,
            color: Self::Color,
        ) -> Result<(), Self::Error> {
            self.display.fill_solid(area, color)
        }

        fn clear(&mut self, color: Self::Color) -> Result<(), Self::Error> {
            self.display.clear(color)
        }
    }
    /// expose internal embedded_graphics support
    impl embedded_graphics::geometry::Dimensions for Display {
        fn bounding_box(&self) -> embedded_graphics::primitives::Rectangle {
            self.display.bounding_box()
        }
    }
    /// expose internal driver support
    impl enmesh_firmware::ux::BufferedDisplay for Display {
        async fn flush(&mut self) -> Result<(), display_interface::DisplayError> {
            self.display.flush()
        }
    }
}

#[cfg(feature = "_screen-epd")]
mod display {
    /// provide the shared crates via re-export
    use common::{display_interface::DisplayError, *};

    /// provide enmesh firmware primitives
    use enmesh_firmware::prelude::*;

    /// provide access to esp32 hardware
    use soc_esp32::*;

    pub struct Display {
        n_vext_control: Option<esp_hal::gpio::Output<'static>>,
        /// FIXME type is overspecified
        display: epd_rs::EpdDisplay<
            epd_rs::drivers::E0213A367<
                epd_rs::EpdInterface<
                    embedded_hal_bus::spi::ExclusiveDevice<
                        soc_esp32::esp_hal::spi::master::Spi<'static, esp_hal::Async>,
                        esp_hal::gpio::Output<'static>,
                        embedded_hal_bus::spi::NoDelay,
                    >,
                    esp_hal::gpio::Output<'static>,
                    esp_hal::gpio::Input<'static>,
                    esp_hal::gpio::Output<'static>,
                    common::embassy_time::Delay,
                >,
            >,
        >,
    }
    impl Display {
        pub async fn new(
            n_vext_control: Option<esp_hal::gpio::Output<'static>>,
            n_reset: esp_hal::gpio::Output<'static>,
            n_busy: esp_hal::gpio::Input<'static>,
            spi: esp_hal::peripherals::SPI3<'static>,
            sdi: esp_hal::gpio::Flex<'static>,
            clk: esp_hal::gpio::Output<'static>,
            cs: esp_hal::gpio::Output<'static>,
            dc: esp_hal::gpio::Output<'static>,
        ) -> Result<Self, DisplayError> {
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
            let display_interface =
                epd_rs::EpdInterface::new(spi_interface, n_busy, n_reset, Delay).await;

            // create the display driver
            let driver = epd_rs::drivers::E0213A367::new(display_interface).await?;

            let display = epd_rs::EpdDisplay::new(driver, epd_rs::DisplayRotation::Rotate270);

            Ok(Self {
                n_vext_control,
                display,
            })
        }
    }

    impl enmesh_firmware::PowerControl for Display
    {
        fn power_off(&mut self) {
            // disable power
            if let Some(pin) = &mut self.n_vext_control {
                pin.set_high();
            }
        }

        #[allow(async_fn_in_trait)] // usage should never use Send()
        /// must reinitialize the hardware as necessary
        async fn power_on(&mut self) {
            // enable power
            if let Some(pin) = &mut self.n_vext_control {
                pin.set_low();
            }

            // perform hardware reset and chip initialization
            let _ = self.display.init().await;
        }
    }

    impl embedded_graphics::draw_target::DrawTarget for Display {
        type Color = embedded_graphics::pixelcolor::BinaryColor;
        type Error = display_interface::DisplayError;

        fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
        where
            I: IntoIterator<Item = embedded_graphics::prelude::Pixel<Self::Color>>,
        {
            self.display.draw_iter(pixels)
        }

        fn fill_contiguous<I>(
            &mut self,
            area: &embedded_graphics::primitives::Rectangle,
            colors: I,
        ) -> Result<(), Self::Error>
        where
            I: IntoIterator<Item = Self::Color>,
        {
            self.display.fill_contiguous(area, colors)
        }

        fn fill_solid(
            &mut self,
            area: &embedded_graphics::primitives::Rectangle,
            color: Self::Color,
        ) -> Result<(), Self::Error> {
            self.display.fill_solid(area, color)
        }

        fn clear(&mut self, color: Self::Color) -> Result<(), Self::Error> {
            self.display.clear(color)
        }
    }
    impl embedded_graphics::geometry::Dimensions for Display {
        fn bounding_box(&self) -> embedded_graphics::primitives::Rectangle {
            self.display.bounding_box()
        }
    }
    impl enmesh_firmware::ux::BufferedDisplay for Display {
        async fn flush(&mut self) -> Result<(), display_interface::DisplayError> {
            self.display.refresh().await
        }
    }
}
