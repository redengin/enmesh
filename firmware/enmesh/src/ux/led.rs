use common::*;

// use embassy_time::Duration;

pub(crate) struct StatusLed<LED> {
    led: LED,
    mode: LedStatusMode,
}
impl<LED> StatusLed<LED>
where
    LED: embedded_hal::digital::OutputPin,
{
    pub(crate) fn new(mut led: LED) -> Self {
        Self {
            led,
            mode: LedStatusMode::OFF,
        }
    }

    pub(crate) fn set_mode(&mut self, mode: LedStatusMode)
    {
        self.mode = mode;
    }

    pub(crate) fn update(&mut self) {
        match self.mode {
            LedStatusMode::OFF => {
                let _ = self.led.set_low();
            }
            LedStatusMode::ON => {
                let _ = self.led.set_high();
            }
         }
    }
}

pub enum LedStatusMode {
    OFF,
    ON,
    // Blink{period: Duration},
}