/// use standard Display Errors
use display_interface::DisplayError;

// provide embedded_hal abstractions
use embedded_hal::digital::{InputPin, OutputPin};
use embedded_hal::spi::SpiDevice;
use embedded_hal::delay::DelayNs;


pub(crate) struct DisplayInterface<SPI, BSY, DC, RST>
{
    /// SPI device
    spi: SPI,
    /// Low for busy, Wait until display is ready!
    busy: BSY,
    /// Data/Command Control Pin (High for data, Low for command)
    dc: DC,
    /// Pin for Reseting
    rst: RST,
}
impl<SPI, BSY, DC, RST> DisplayInterface<SPI, BSY, DC, RST>
where
    SPI: SpiDevice,
    RST: OutputPin,
    DC: OutputPin,
    BSY: InputPin,
{
    /// Create and initialize display
    pub fn new(spi: SPI, busy: BSY, dc: DC, rst: RST) -> Self {
        Self { spi, busy, dc, rst }
    }

    /// Resets the device.
    pub(crate) fn reset(&mut self, delay: &mut impl DelayNs) {
        self.rst.set_high().unwrap();
        delay.delay_ms(100);
        self.rst.set_low().unwrap();
        delay.delay_ms(100);
        self.rst.set_high().unwrap();
    }

    /// Waits until device isn't busy anymore (busy == HIGH)
    pub(crate) fn wait_until_idle(&mut self, delay: &mut impl DelayNs) {
        while self.busy.is_high().unwrap_or(true) {
            delay.delay_ms(1)
        }
    }

    /// Basic function for sending commands
    pub(crate) fn send_command(&mut self, command: u8) -> Result<(), DisplayError> {
        // low for commands
        self.dc.set_low().map_err(|_| DisplayError::DCError)?;

        // Transfer the command over spi
        self.spi
            .write(&[command])
            .map_err(|_| DisplayError::BusWriteError)
    }

    /// Basic function for sending an array of u8-values of data over spi
    pub(crate) fn send_data(&mut self, data: &[u8]) -> Result<(), DisplayError> {
        // high for data
        self.dc.set_high().map_err(|_| DisplayError::DCError)?;

        // Transfer data (u8-array) over spi
        self.spi
            .write(data)
            .map_err(|_| DisplayError::BusWriteError)
    }

    // /// Basic function for sending a command and the data belonging to it.
    // pub(crate) fn cmd_with_data(&mut self, command: u8, data: &[u8]) -> Result<(), DisplayError> {
    //     self.cmd(command)?;
    //     self.data(data)
    // }

    // /// Basic function for sending the same byte of data (one u8) multiple times over spi
    // /// Used for setting one color for the whole frame
    // pub(crate) fn data_x_times(&mut self, val: u8, repetitions: u32) -> Result<(), DisplayError> {
    //     // high for data
    //     let _ = self.dc.set_high();
    //     // Transfer data (u8) over spi
    //     for _ in 0..repetitions {
    //         self.spi
    //             .write(&[val])
    //             .map_err(|_| DisplayError::BusWriteError)?;
    //     }
    //     Ok(())
    // }
}
