// use embassy_time::Duration;

#[allow(dead_code)]
pub(crate) struct StatusLed<LED> {
    led: LED,
    mode: LedStatusMode,
}
impl<LED> StatusLed<LED>
where
    LED: common::led::LedState
{
    pub(crate) fn new(led: LED) -> Self {
        Self {
            led,
            mode: LedStatusMode::OFF,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn set_mode(&mut self, mode: LedStatusMode)
    {
        self.mode = mode;
    }

    #[allow(dead_code)]
    pub(crate) fn update(&mut self) {
        match self.mode {
            LedStatusMode::OFF => {
                let _ = self.led.off();
            }
            LedStatusMode::ON => {
                let _ = self.led.on();
            }
         }
    }
}

#[allow(dead_code)]
pub enum LedStatusMode {
    OFF,
    ON,
    // Blink{period: Duration},
}