/// provide the shared crates via re-export
use common::*;

pub async fn run<DISPLAY>(
    global_state: &'static RwLock<NoopRawMutex, crate::State>,
    mut display: DISPLAY,
    button: impl button::ButtonState,
    led: impl led::LedState,
)
where
    DISPLAY: BufferedDisplay,
    <DISPLAY as embedded_graphics::draw_target::DrawTarget>::Color: From<embedded_graphics::pixelcolor::Rgb888>
    + themes::ThemeForColor,
{
    // create the status led
    let _status_led = status_led::StatusLed::new(led);

    // create a button monitor
    let mut button_monitor = ButtonMonitor::new(button);

    // create the theme
    let theme =
        <<DISPLAY as embedded_graphics::draw_target::DrawTarget>::Color as themes::ThemeForColor>
            ::theme(display.bounding_box().size);

    // create the page controller
    let mut page_controller = pages::PageController::new();

    // enable the display
    display.power_on().await;

    loop {
        // clone the current state
        let model = global_state.read().await.clone();

        if display.is_powered() {
            // update the display
            use embedded_graphics::draw_target::DrawTargetExt;
            let needs_refresh = page_controller.update(&mut display.color_converted(), &theme, &model);
            if needs_refresh {
                display.flush().await.ok();
            }
        }

        // update the status led
        // TODO

        // monitor button
        if let Some(event) = button_monitor.update().await
        {
            if display.is_powered() {
                page_controller.handle_event(&event);
            }
            else {
                display.power_on().await;
            }
        }
    }
}

/// Buffered DrawTarget require a flush() to refresh the screen
pub trait BufferedDisplay:
    embedded_graphics::draw_target::DrawTarget + crate::PowerControl
{
    #[allow(async_fn_in_trait)]
    /// sends data to the screen and triggers a screen refresh
    async fn flush(&mut self) -> Result<(), display_interface::DisplayError>;
}

/// provide support for status led
pub mod status_led;

/// provide themes
pub mod themes;

/// User interaction events
#[derive(Debug)]
pub enum HidEvent {
    /// move to next selectable item
    Next,
    /// move to the previous selectable item
    Previous,
    /// invokes the selected item's handler
    Select,
    /// finds the touched item and invokes a 'Select' event
    Touch { x: u32, y: u32 },
}

/// provide enmesh primitives
use crate::prelude::*;

/// monitors button for HID events
pub struct ButtonMonitor<BUTTON> {
    button: BUTTON,
    active_start: Option<Instant>,
}
impl<BUTTON> ButtonMonitor<BUTTON>
where
    BUTTON: button::ButtonState,
{
    pub fn new(button: BUTTON) -> Self {
        Self
        {
            button,
            active_start: None,
        }
    }

    const SCAN_PERIOD_MILLIS: u64 = 33;
    const SHORT_PRESS_DURATION: Duration = Duration::from_millis(2 * Self::SCAN_PERIOD_MILLIS);
    const LONG_PRESS_DURATION: Duration = Duration::from_millis(10 * Self::SCAN_PERIOD_MILLIS);
    fn scan_button(&mut self) -> Option<HidEvent>
    {
        if let Ok(is_active) = self.button.is_active() {
            if is_active && self.active_start.is_none() {
                // memo when the press began
                self.active_start = Some(Instant::now());
            }
            else if !is_active && self.active_start.is_some() {
                let duration = Instant::now() - self.active_start.unwrap();
                // clear the memo
                self.active_start = None;

                // determine HID Event
                if duration > Self::LONG_PRESS_DURATION {
                    return Some(HidEvent::Select)
                }
                else if duration > Self::SHORT_PRESS_DURATION {
                    return Some(HidEvent::Next)
                }
                else {
                    return None
                }
            }
        }
        None
    }

    pub async fn update(&mut self) -> Option<HidEvent>
    {
        let mut ticker = Ticker::every(Duration::from_millis(Self::SCAN_PERIOD_MILLIS));
        for _ in 0..4 {
            if let Some(event) = self.scan_button()
            {
                return Some(event);
            }
            // delay until next cycle
            ticker.next().await;
        }

        None
    }

    pub fn update_sync(&mut self, delay_ns: &mut impl embedded_hal::delay::DelayNs) -> Option<HidEvent>
    {
        for _ in 0..4 {
            if let Some(event) = self.scan_button()
            {
                return Some(event);
            }
             // delay until next cycle
            delay_ns.delay_ms(Self::SCAN_PERIOD_MILLIS as u32);
        }

        None
    }


}


/// provide screens and navigation
pub mod pages;
