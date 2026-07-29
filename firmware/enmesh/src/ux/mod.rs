use crate::prelude::*;

/// provide controller thread runners
pub mod controller;

/// provide themes for Views
pub(crate) mod themes;

pub(crate) trait View {
    /// repaint the entire view
    fn refresh(
        &mut self,
        display: &mut impl DrawTargetExt<Color = Rgb888>,
        // FIXME should be more generic
        model: &crate::State,
        theme: &Theme,
    );

    /// update the view
    /// * only needs to update changes
    fn update(
        &mut self,
        display: &mut impl DrawTargetExt<Color = Rgb888>,
        // FIXME should be more generic
        model: &crate::State,
        theme: &Theme,
    ) {
        // default to full refresh
        self.refresh(display, model, theme);
    }

    /// handle HidEvent
    /// returns true if the event was handled and should not be bubbled up
    fn handle_event(&mut self, event: &HidEvent) -> bool;
}

/// User interaction events
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
/// active HID input durations greater than this, should generate a HidEvent::Select
pub const HID_HELD_DURATION: Duration = Duration::from_millis(500);

/// provide the pages
mod pages;
pub struct Ux {
    current_page: pages::Pages,
    needs_page_refresh: bool,
}
use pages::prelude::*;
impl Ux {
    pub fn new() -> Self {
        Self {
            current_page: pages::Pages::Page0(pages::home::Home::new()),
            needs_page_refresh: true,
        }
    }

    fn tab_bar_refresh(&self, display: &mut impl DrawTargetExt<Color = Rgb888>, theme: &Theme) {
        let _ = display.clear(theme.background.into());

        let selected_index = self.current_page.index();
        const SELECTED: &str = "^";
        const NOT_SELECTED: &str = "-";
        LinearLayout::horizontal(
            Chain::new(Text::new(
                if selected_index == 0 {
                    SELECTED
                } else {
                    NOT_SELECTED
                },
                Point::zero(),
                theme.text_style,
            ))
            .append(Text::new(
                if selected_index == 1 {
                    SELECTED
                } else {
                    NOT_SELECTED
                },
                Point::zero(),
                theme.text_style,
            ))
            .append(Text::new(
                if selected_index == 2 {
                    SELECTED
                } else {
                    NOT_SELECTED
                },
                Point::zero(),
                theme.text_style,
            )),
        )
        .with_spacing(DistributeFill(display.bounding_box().size.width))
        .arrange()
        .align_to(&display.bounding_box(), horizontal::Left, vertical::Bottom)
        .draw(display)
        .ok();
    }
}

impl View for Ux {
    fn refresh(
        &mut self,
        display: &mut impl DrawTargetExt<Color = Rgb888>,
        model: &crate::State,
        theme: &Theme,
    ) {
        // UX always uses update as it has full control of the display
        panic!("UX view users should always use View::update()")
    }

    fn update(
        &mut self,
        display: &mut impl DrawTargetExt<Color = Rgb888>,
        model: &crate::State,
        theme: &Theme,
    ) {
        // get the screen size
        let bounding_box = display.bounding_box();
        // reserve space for the tab_bar
        let tab_bar_height = theme.text_style.line_height();

        // create a cropped display for the page content (excluding the tab bar)
        let mut page_display = display.cropped(&Rectangle {
            top_left: Point::zero(),
            size: Size::new(
                bounding_box.size.width,
                bounding_box.size.height - tab_bar_height,
            ),
        });

        // paint the current page
        match self.needs_page_refresh {
            // do full refresh
            true => {
                self.current_page.refresh(&mut page_display, model, &theme);

                // refresh the tab bar inside a cropped display
                let mut tab_bar_display = display.cropped(&Rectangle {
                    top_left: Point::new(0, (bounding_box.size.height - tab_bar_height) as i32),
                    size: Size::new(bounding_box.size.width, tab_bar_height),
                });
                self.tab_bar_refresh(&mut tab_bar_display, &theme);

                self.needs_page_refresh = false;
            }
            // do simple update
            false => self.current_page.update(&mut page_display, model, &theme),
        }

        // if ble pairing show a dialog with the passkey
        match model.ble_status {
            crate::state::BleStatus::Pairing { passkey } => {
                // draw a framing rectangle
                let frame = Rectangle::new(Point::zero(), Size::new(120, 40))
                    .into_styled(
                        PrimitiveStyleBuilder::new()
                            .stroke_color(theme.color)
                            .stroke_width(1)
                            .fill_color(theme.background)
                            .build(),
                    )
                    .align_to(&bounding_box, horizontal::Center, vertical::Center);
                frame.draw(display).ok();

                // draw the dialog text
                LinearLayout::vertical(
                    Chain::new(Text::new("BLE Pairing", Point::zero(), theme.text_style)).append(
                        Text::new(
                            format!(6; "{:06}", passkey).unwrap().as_str(),
                            Point::zero(),
                            theme.h1_style,
                        ),
                    ),
                )
                .with_alignment(horizontal::Center)
                .arrange()
                .align_to(&frame, horizontal::Center, vertical::Center)
                .draw(display)
                .ok();

                // repaint the page below the dialog on next update
                self.needs_page_refresh = true;
            }
            _ => { /* no dialog */ }
        }
    }

    /// handle HidEvent
    fn handle_event(&mut self, event: &HidEvent) -> bool {
        let handled = self.current_page.handle_event(event);

        if !handled {
            match event {
                HidEvent::Next => {
                    self.current_page = self.current_page.next();
                    self.needs_page_refresh = true;
                }
                HidEvent::Previous => {
                    self.current_page = self.current_page.previous();
                    self.needs_page_refresh = true;
                }
                _ => {}
            }
        }

        // UX always handles the event
        true
    }
}

// // provide the shared crates via re-export
// use common::*;

// /// provide primitives necessary to use enmesh firmware
// use crate::prelude::*;

// /// provide embedded graphics primitives
// use embedded_graphics::prelude::*;

// pub async fn run<D: DrawTarget + embedded_graphics::geometry::OriginDimensions>(
//     global_state: &'static RwLock<NoopRawMutex, crate::State>,
//     mut screen: D,
//     mut button: impl button::ButtonState,
//     mut led: impl embedded_hal::digital::OutputPin,
// )
// where <D as embedded_graphics::draw_target::DrawTarget>::Color: From<embedded_graphics::pixelcolor::Rgb888>
// {
//     let status_led = led::StatusLed::new(led);

//     // create the UX
//     let mut ux = crate::ux::Ux::new();
//     // create our ux theme
//     let screen_size = screen.size();
//     let theme = crate::ux::themes::DefaultTheme(screen_size);

//     // monitor the button state change
//     let mut button_active_frames = 0;

//    // provide threading primitives
//     use embassy_time::Duration;
//     const FRAME_RATE: u64 = 30; // frames per second
//     let mut frame_ticker = embassy_time::Ticker::every(Duration::from_hz(FRAME_RATE));
//     loop {
//         // monitor the button
//         if let Ok(active) = button.is_active() {
//             if active {
//                 button_active_frames += 1;
//             } else {
//                 const DEBOUNCE_FRAMES: u32 = 2;
//                 if button_active_frames >= DEBOUNCE_FRAMES {
//                     let button_down_duration = button_active_frames * Duration::from_hz(FRAME_RATE);
//                     // convert core::time::Duration -> embassy_time::Duration
//                     let hid_held_duration = embassy_time::Duration::from_millis(
//                         crate::ux::HID_HELD_DURATION.as_millis() as u64,
//                     );
//                     if button_down_duration >= hid_held_duration {
//                         ux.handle_event(&crate::ux::HidEvent::Select);
//                     } else {
//                         ux.handle_event(&crate::ux::HidEvent::Next);
//                     }
//                 }
//                 // reset the button monitor
//                 button_active_frames = 0;
//             }
//         }

//         // transmute the screen to support Rgb888
//         let mut rgb_screen = screen.color_converted();

//         // update the UX
//         use crate::ux::Page;
//         let model = global_state.read().await.clone();
//         ux.refresh(&mut rgb_screen, &model, &theme);
//         screen.flush().ok(); // must call flush to commit the changes to the screen

//         // await the next cycle
//         frame_ticker.next().await;
//     }
// }
