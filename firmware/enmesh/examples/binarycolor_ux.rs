// provide the shared crates via re-export
use common::*;

// use embedded_graphics::pixelcolor::{PixelColor, Rgb888};
/// UX designed for RGB888
/// * uses embedded_graphics::draw_target::ColorCoverted to support all screens
// use embedded_graphics::prelude::*; // provide common traits
use embedded_graphics::pixelcolor::BinaryColor;
// use enmesh_firmware::ux::ButtonMonitor;

fn main() -> Result<(), std::convert::Infallible> {
    use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay};

    // create a simulated screen per the hardware
    if cfg!(feature = "example_ux-large") {
        // create a native window for the simulation
        let output_settings = OutputSettingsBuilder::new().scale(2).build();
        let window = embedded_graphics_simulator::Window::new(
            "User Interface (SPACEBAR as button)",
            &output_settings,
        );

        // create a simulation display
        let screen_size = embedded_graphics::geometry::Size::new(250, 122);
        let screen: SimulatorDisplay<BinaryColor> = SimulatorDisplay::new(screen_size);

        // start the UX simulation
        run(window, screen);
    } else {
        // create a native window for the simulation
        let output_settings = OutputSettingsBuilder::new()
            .theme(embedded_graphics_simulator::BinaryColorTheme::OledBlue)
            .build();
        let window = embedded_graphics_simulator::Window::new(
            "Heltec User Interface (SPACEBAR as button)",
            &output_settings,
        );

        // create a simulation screen
        let screen_size = embedded_graphics::geometry::Size::new(128, 64);
        let screen: SimulatorDisplay<BinaryColor> = SimulatorDisplay::new(screen_size);

        // start the UX simulation
        run(window, screen);
    }

    Ok(())
}

fn run(
    mut window: embedded_graphics_simulator::Window,
    mut screen: embedded_graphics_simulator::SimulatorDisplay<BinaryColor>,
) {
    // create a simulated button
    use embedded_graphics_simulator::sdl2::Keycode;
    const SIMULATED_BUTTON: Keycode = Keycode::SPACE; // use spacebar as button
    let mut simulated_button = SimulatedButton { active: false };
    // let button_monitor = ButtonMonitor::new(&simulated_button);

    //     // create our enmesh State (used as Ux model)
    //     let state = enmesh_firmware::State::new();
    //     // create our enmesh ux instance
    //     let mut ux = enmesh_firmware::ux::Ux::new();
    //     // create our enmesh ux theme
    //     let screen_size = screen.size();
    //     let theme = enmesh_firmware::ux::themes::Theme::new(screen_size);
    //     // refresh the simulated display
    //     let mut rgb_screen = screen.color_converted();
    //     use enmesh_firmware::ux::View;
    //     ux.update(&mut rgb_screen, &state, &theme);

    //     let mut button_down_time: Option<std::time::Instant> = None;
    'running: loop {
        // update the native window to gather events
        window.update(&screen);

        // handle Simulator events
        use embedded_graphics_simulator::SimulatorEvent;
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
                        // record the event timestamp to determine type of interaction
                        simulated_button.active = true;
                    }
                }
                // handle simulated button UP, and standard keyboard ux
                SimulatorEvent::KeyUp {
                    keycode,
                    keymod: _,
                    repeat,
                } => {
                    if (keycode == SIMULATED_BUTTON) && !repeat {
                        // record the event timestamp to determine type of interaction
                        simulated_button.active = false;
                    }
                    // handle standard keyboard ux
                    // else if keycode == Keycode::TAB {
                    //     use embedded_graphics_simulator::sdl2::Mod;
                    //     // handle the event by the UX
                    //     if keymod.contains(Mod::LSHIFTMOD) || keymod.contains(Mod::RSHIFTMOD) {
                    //         ux.handle_event(&enmesh_firmware::ux::HidEvent::Previous);
                    //     } else {
                    //         ux.handle_event(&enmesh_firmware::ux::HidEvent::Next);
                    //     }
                    // } else if (keycode == Keycode::RETURN) || (keycode == Keycode::RETURN2) {
                    //     ux.handle_event(&enmesh_firmware::ux::HidEvent::Select);
                    // }
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

        // update the simulated display
        // let mut rgb_screen = screen.color_converted();
        // ux.update(&mut rgb_screen, &state, &theme);

        // sleep for a frame period
        const FPS_HZ: u64 = 10;
        const FRAME_PERIOD_MILLIS: u64 = 1000 / FPS_HZ;
        std::thread::sleep(std::time::Duration::from_millis(FRAME_PERIOD_MILLIS));
    }
}

struct SimulatedButton {
    pub active: bool,
}
impl common::button::ButtonState for SimulatedButton {
    type Error = ();

    fn is_active(&mut self) -> Result<bool, Self::Error> {
        Ok(self.active)
    }
}
