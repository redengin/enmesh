// provide the shared crates via re-export
use common::{embassy_time::Duration, *};

// use embedded_graphics::pixelcolor::{PixelColor, Rgb888};
/// UX designed for RGB888
/// * uses embedded_graphics::draw_target::ColorCoverted to support all screens
// use embedded_graphics::prelude::*; // provide common traits
use embedded_graphics::pixelcolor::BinaryColor;
use enmesh_firmware::ux::pages;
// use enmesh_firmware::ux::ButtonMonitor;

fn main() -> Result<(), std::convert::Infallible> {
    use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay};

    // create a simulated screen per the hardware
    if cfg!(feature = "example_ux-large") {
        // create a native window for the simulation
        let output_settings = OutputSettingsBuilder::new().scale(2).build();
        let window = embedded_graphics_simulator::Window::new(
            "Large Display (SPACEBAR as button)",
            &output_settings,
        );

        // create a simulation display
        let display_size = embedded_graphics::geometry::Size::new(250, 122);
        let display: SimulatorDisplay<BinaryColor> = SimulatorDisplay::new(display_size);

        // start the UX simulation
        run(window, display);
    } else {
        // create a native window for the simulation
        let output_settings = OutputSettingsBuilder::new()
            .theme(embedded_graphics_simulator::BinaryColorTheme::OledBlue)
            .build();
        let window = embedded_graphics_simulator::Window::new(
            "Small Display (SPACEBAR as button)",
            &output_settings,
        );

        // create a simulation screen
        let display_size = embedded_graphics::geometry::Size::new(128, 64);
        let display: SimulatorDisplay<BinaryColor> = SimulatorDisplay::new(display_size);

        // start the UX simulation
        run(window, display);
    }

    Ok(())
}

fn run(
    mut window: embedded_graphics_simulator::Window,
    mut display: embedded_graphics_simulator::SimulatorDisplay<BinaryColor>,
) {
    // create theme for pages
    use common::embedded_graphics::geometry::OriginDimensions;
    let theme = enmesh_firmware::ux::binary_color::themes::Theme::new(display.size());
    // create the page controller
    let mut page_controller = pages::PageController::new();

    // create the enmesh State (used as MVC model)
    let state = enmesh_firmware::State::new();
 
    // create a simulated button
    use embedded_graphics_simulator::sdl2::Keycode;
    const SIMULATED_BUTTON: Keycode = Keycode::SPACE; // use spacebar as button
    // let mut simulated_button = SimulatedButton { active: false };
    // use enmesh_firmware::ux::ButtonMonitor;
    // let mut button_monitor = ButtonMonitor::new(simulated_button);

    /// provide ux primitives
    use enmesh_firmware::ux::HidEvent;
    'running: loop {
        // update the display
        page_controller.update(&mut display, &state);

        // update the native window to gather events
        window.update(&display);

        // handle Simulator events
        use embedded_graphics_simulator::SimulatorEvent;  // provide trait access
        for event in window.events() {
            match event {
                // stop running upon Quit
                SimulatorEvent::Quit => {
                    break 'running;
                }

                // handle simulated button DOWN
                SimulatorEvent::KeyDown {
                    keycode,
                    keymod: _,
                    repeat,
                } => {
                    if (keycode == SIMULATED_BUTTON) && !repeat {
                    //     // record the event timestamp to determine type of interaction
                    //     simulated_button.set_active(true);
                    }
                }
                // handle simulated button UP, and standard keyboard ux
                SimulatorEvent::KeyUp {
                    keycode,
                    keymod,
                    repeat,
                } => {
                    if (keycode == SIMULATED_BUTTON) && !repeat {
                        // record the event timestamp to determine type of interaction
                        // simulated_button_state = false;
                    }
                    // handle standard keyboard ux
                    else if keycode == Keycode::TAB {
                        use embedded_graphics_simulator::sdl2::Mod;
                        // handle the event by the UX
                        if keymod.contains(Mod::LSHIFTMOD) || keymod.contains(Mod::RSHIFTMOD) {
                            page_controller.handle_event(&HidEvent::Previous);
                        } else {
                            page_controller.handle_event(&HidEvent::Next);
                        }
                    } else if (keycode == Keycode::RETURN) || (keycode == Keycode::RETURN2) {
                        page_controller.handle_event(&HidEvent::Select);
                    }
                }

                // handle touch/mouse-click events
                SimulatorEvent::MouseButtonDown {
                    mouse_btn,
                    point: _,
                } => {
                    use embedded_graphics_simulator::sdl2::MouseButton;
                    if mouse_btn == MouseButton::Left {
                        // handle event by the UX
                        // ux.handle_event(&enmesh_firmware::ux::HidEvent::Touch {
                        //     x: point.x as u32,
                        //     y: point.y as u32,
                        // });
                    }
                }

                // ignore all other events
                _ => {}
            }
        }

        // sleep for a frame period
        const FRAME_PERIOD: std::time::Duration = std::time::Duration::from_millis(100);
        std::thread::sleep(FRAME_PERIOD);
    }
}

// struct SimulatedButton {
//     pub active: bool,
// }
// impl SimulatedButton {
//     pub fn set_active(&mut self, active: bool) 
//     {
//         self.active = active;
//     }
// }

// impl common::button::ButtonState for SimulatedButton {
//     type Error = ();

//     fn is_active(&mut self) -> Result<bool, Self::Error> {
//         Ok(self.active)
//     }

// }