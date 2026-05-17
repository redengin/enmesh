
pub trait LedState {
    type Error;
    fn on(&mut self) -> Result<(), Self::Error>;
    fn off(&mut self) -> Result<(), Self::Error>;
}

pub struct Led<PIN> {
    pin: PIN,
    active_high: bool,
}

impl<PIN: embedded_hal::digital::OutputPin> Led<PIN>
{
    pub fn active_high(pin: PIN) -> Self {
        Self {
            pin,
            active_high: true,
        }
    }

    pub fn active_low(pin: PIN) -> Self {
        Self {
            pin,
            active_high: false,
        }
    }
}

impl<PIN: embedded_hal::digital::OutputPin> LedState for Led<PIN> {
    type Error = PIN::Error;

    fn on(&mut self) -> Result<(), Self::Error> {
        match self.active_high
        {
            true => self.pin.set_high(),
            false => self.pin.set_low()
        }
    }

    fn off(&mut self) -> Result<(), Self::Error> {
        match self.active_high
        {
            true => self.pin.set_low(),
            false => self.pin.set_high()
        }
    }
}
