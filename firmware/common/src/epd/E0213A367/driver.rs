/// use standard Display Errors
use display_interface::DisplayError;

use embedded_graphics::prelude::*;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::draw_target::DrawTarget;
use embedded_hal::delay::DelayNs;
/// provide graphics primitives
use embedded_hal::digital::{InputPin, OutputPin};
use embedded_hal::spi::SpiDevice;

// provide hardware interface abstraction
use super::interface::DisplayInterface;


use super::commands;

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


	// fn set_partial_ram_area(&mut self, x: u16, y: u16, w:u16, h: u16) -> Result<(), DisplayError>
	// {
	// 	self.interface.send_command(0x11)?; // set ram entry mode
	// 	self.interface.send_data(&[0x00])?;    // x increase, y increase : normal mode
	// 	self.interface.send_command(0x44)?;
	// 	self.interface.send_data(&[
    //         // ((x + w - 1) / 8) as u8,
    //         // (x / 8) as u8,
    //         0x0f, 0
    //     ])?;
	// 	self.interface.send_command(0x45)?;
	// 	self.interface.send_data(&[
    //         // ((y + h - 1) % 256) as u8,
    //         // (y % 256) as u8,
    //         0xf9, 0
    //     ])?;
	// 	self.interface.send_command(0x4e)?;
	// 	self.interface.send_data(&[
    //         // (x / 8) as u8,
    //         // ((x + w - 1) / 8) as u8,
    //         0x0e
    //     ])?;
	// 	self.interface.send_command(0x4f)?;
	// 	self.interface.send_data(&[
    //         // (y % 256) as u8,
    //         // ((y + h - 1) % 256) as u8,
    //         0xf9
    //     ])
	// }


    // fn update()

}
impl<SPI, BSY, DC, RST> DrawTarget for Screen<SPI, BSY, DC, RST> {
    type Error = DisplayError;
    type Color = BinaryColor;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        // FIXME
        // for p in pixels.into_iter() {
        //     self.draw_helper(WIDTH.into(), HEIGHT.into(), p)?;
        // }
        Ok(())
    }
}
impl<SPI, BSY, DC, RST> OriginDimensions for Screen<SPI, BSY, DC, RST> {
    fn size(&self) -> Size {
        //if display is rotated 90 deg or 270 then swap height and width
        // match self.rotation() {
        //     DisplayRotation::Rotate0 | DisplayRotation::Rotate180 => {
        //         Size::new(WIDTH.into(), HEIGHT.into())
        //     }
        //     DisplayRotation::Rotate90 | DisplayRotation::Rotate270 => {
        //         Size::new(HEIGHT.into(), WIDTH.into())
        //     }
        // }
        Size::new(122, 250)
    }
}
