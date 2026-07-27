// use embassy_time::Duration;

pub(crate) struct StatusLed<LED> {
    led: LED,
    mode: LedStatusMode,
}
// impl<LED> StatusLed<LED>
// where
//     LED: embedded_hal::digital::OutputPin,
// {
//     pub(crate) fn new(led: LED) -> Self {
//         Self {
//             led,
//             mode: LedStatusMode::OFF,
//         }
//     }

//     pub(crate) fn set_mode(&mut self, mode: LedStatusMode)
//     {
//         self.mode = mode;
//     }

//     pub(crate) fn update(&mut self) {
//         match self.mode {
//             LedStatusMode::OFF => {
//                 let _ = self.led.set_low();
//             }
//             LedStatusMode::ON => {
//                 let _ = self.led.set_high();
//             }
//          }
//     }
// }
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

    pub(crate) fn set_mode(&mut self, mode: LedStatusMode)
    {
        self.mode = mode;
    }

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

pub enum LedStatusMode {
    OFF,
    ON,
    // Blink{period: Duration},
}