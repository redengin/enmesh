/// use standard Display Errors
use display_interface::DisplayError;

use embedded_hal::delay::DelayNs;
/// provide graphics primitives
use embedded_hal::digital::{InputPin, OutputPin};
use embedded_hal::spi::SpiDevice;

// provide hardware interface abstraction
use super::interface::DisplayInterface;

pub struct Screen<SPI, BSY, RST, DC> {
    interface: DisplayInterface<SPI, BSY, RST, DC>,
    // dimensions: Size,  122x250
}
impl<SPI, BSY, DC, RST> Screen<SPI, BSY, DC, RST>
where
    SPI: SpiDevice,
    RST: OutputPin,
    DC: OutputPin,
    BSY: InputPin,
{
    pub fn new(
        spi: SPI,
        busy: BSY,
        dc: DC,
        rst: RST,
        delay: &mut impl DelayNs,
    ) -> Result<Self, DisplayError>
    where
        Self: Sized,
    {
        let interface = DisplayInterface::new(spi, busy, dc, rst);
        let mut this = Self { interface };
        this.init(delay)?;
        Ok(this)
    }

    /// Reset and Initialize the controller
    pub fn init(&mut self, delay: &mut impl DelayNs) -> Result<(), DisplayError> {

        // reset the chip
        self.interface.reset(delay);
        self.interface.wait_until_idle(delay);


        // https://github.com/HelTecAutomation/Heltec_ESP32/blob/55bf1a5fe0ed102c807b1fcb550a1a8ea31d6bd3/src/HT_E0213A367.h#L267
        //----------------------------------------------------------------------
        // soft reset
        self.interface.send_command(0x12)?;
        self.interface.wait_until_idle(delay);
        // configure output control
        self.interface.send_command(0x01)?;
        self.interface.send_data(&[0xF9])?;
        self.interface.send_data(&[0x00])?;
        // configure border waveform
        self.interface.send_command(0x3C)?;
        self.interface.send_data(&[0x01])?;
        self.interface.send_command(0x18)?;
        self.interface.send_data(&[0x80])?;
        // configure waveform ID
        self.interface.send_command(0x37)?;
        self.interface.send_data(&[0x40])?;
        self.interface.send_data(&[0x80])?;
        self.interface.send_data(&[0x03])?;
        self.interface.send_data(&[0x0E])?;
        // set ram entry mode
        self.interface.send_command(0x11)?;
        self.interface.send_data(&[0x00])?;
        self.interface.send_command(0x44)?;
        self.interface.send_data(&[0x0f])?;
        self.interface.send_data(&[0])?;
        self.interface.send_command(0x45)?;
        self.interface.send_data(&[0xf9])?;
        self.interface.send_data(&[0])?;
        self.interface.send_command(0x4e)?;
        self.interface.send_data(&[0x0e])?;
        self.interface.send_command(0x4f)?;
        self.interface.send_data(&[0xf9])?;

        Ok(())
    }



}
