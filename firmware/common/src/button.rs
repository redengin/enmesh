
pub trait ButtonState {
    type Error;
    fn is_active(&mut self) -> Result<bool, Self::Error>;
}

pub struct Button<PIN> {
    pin: PIN,
    active_high: bool,
}

impl<PIN: embedded_hal::digital::InputPin> Button<PIN>
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

impl<PIN: embedded_hal::digital::InputPin> ButtonState for Button<PIN> {
    type Error = PIN::Error;

    fn is_active(&mut self) -> Result<bool, Self::Error> {
        if self.active_high {
            self.pin.is_high()
        }
        else {
            self.pin.is_low()
        }
    }
}
