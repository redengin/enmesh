// provide the shared crates via re-export
use common::*;

use embedded_graphics::pixelcolor::{PixelColor, Rgb888};
/// UX designed for RGB888
/// * uses embedded_graphics::draw_target::ColorCoverted to support all screens
use embedded_graphics::prelude::*; // provide common traits

fn main() -> Result<(), std::convert::Infallible> {
    use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay};

    // create a simulated screen per the hardware
    if cfg!(feature = "ux-heltec") {
        // create a native window for the simulation
        let output_settings = OutputSettingsBuilder::new()
            .theme(embedded_graphics_simulator::BinaryColorTheme::OledBlue)
            .build();
        let window = embedded_graphics_simulator::Window::new(
            "Heltec User Interface (SPACEBAR as button)",
            &output_settings,
        );

        // create a simulation screen
        let screen_size = Size::new(128, 64);
        let screen: SimulatorDisplay<embedded_graphics::pixelcolor::BinaryColor> =
            SimulatorDisplay::new(screen_size);

        // start the UX simulation
        run(window, screen);
    }
    // create a default simulated screen
    else {
        // create a native window for the simulation
        let output_settings = OutputSettingsBuilder::new().scale(2).build();
        let window = embedded_graphics_simulator::Window::new(
            "User Interface (SPACEBAR as button)",
            &output_settings,
        );

        // create a simulation display
        let screen_size = Size::new(320, 240);
        let screen: SimulatorDisplay<Rgb888> = SimulatorDisplay::new(screen_size);

        // start the UX simulation
        run(window, screen);
    }

    Ok(())
}

use enmesh_firmware::ux::Page;
fn run<Color>(
    mut window: embedded_graphics_simulator::Window,
    mut screen: embedded_graphics_simulator::SimulatorDisplay<Color>,
) where
    // allow UX to use max colors (which will be converted into actual display Color)
    Color: PixelColor + Into<Rgb888> + From<Rgb888>,
{
    // create our enmesh State (used as Ux model)
    let state = enmesh_firmware::State::new();
    // create our enmesh ux instance
    let mut ux = enmesh_firmware::ux::Ux::new();
    // create our enmesh ux theme
    let screen_size = screen.size();
    let theme = enmesh_firmware::ux::themes::DefaultTheme(screen_size);
    // refresh the simulated display 
    let mut rgb_screen = screen.color_converted();
    ux.refresh(&mut rgb_screen, &state, &theme);

    // create a simulation button
    use embedded_graphics_simulator::sdl2::Keycode;
    const SIMULATED_BUTTON: Keycode = Keycode::SPACE; // use spacebar as button

    let mut button_down_time: Option<std::time::Instant> = None;
    'running: loop {
        // update the native window to gather events
        window.update(&screen);

        // handle UX events
        use embedded_graphics_simulator::SimulatorEvent;
        for event in window.events() {
            match event {
                // stop running upon Quit
                SimulatorEvent::Quit => {
                    break 'running;
                }

                // handle simulated embedded button DOWN
                SimulatorEvent::KeyDown {
                    keycode,
                    keymod: _,
                    repeat,
                } => {
                    if (keycode == SIMULATED_BUTTON) && !repeat {
                        // record the event timestamp to determine type of interaction
                        button_down_time = Some(std::time::Instant::now());
                    }
                }
                // handle simulated embedded button UP, and standard keyboard ux
                SimulatorEvent::KeyUp {
                    keycode,
                    keymod,
                    repeat,
                } => {
                    if (keycode == SIMULATED_BUTTON) && !repeat {
                        if let Some(start) = button_down_time {
                            // handle button press
                            let elapsed_millis = std::time::Instant::now() - start;
                            // handle the event by the UX
                            if elapsed_millis >= enmesh_firmware::ux::HID_HELD_DURATION {
                                ux.handle_event(&enmesh_firmware::ux::HidEvent::Select);
                            } else {
                                ux.handle_event(&enmesh_firmware::ux::HidEvent::Next);
                            }
                            // reset the start time
                            button_down_time = None;
                        }
                    }
                    // handle standard keyboard ux
                    else if keycode == Keycode::TAB {
                        use embedded_graphics_simulator::sdl2::Mod;
                        // handle the event by the UX
                        if keymod.contains(Mod::LSHIFTMOD) || keymod.contains(Mod::RSHIFTMOD) {
                            ux.handle_event(&enmesh_firmware::ux::HidEvent::Previous);
                        } else {
                            ux.handle_event(&enmesh_firmware::ux::HidEvent::Next);
                        }
                    } else if (keycode == Keycode::RETURN) || (keycode == Keycode::RETURN2) {
                        ux.handle_event(&enmesh_firmware::ux::HidEvent::Select);
                    }
                }

                // handle touch/mouse-click events
                SimulatorEvent::MouseButtonDown { mouse_btn, point } => {
                    use embedded_graphics_simulator::sdl2::MouseButton;
                    if mouse_btn == MouseButton::Left {
                        // handle event by the UX
                        ux.handle_event(&enmesh_firmware::ux::HidEvent::Touch {
                            x: point.x as u32,
                            y: point.y as u32,
                        });
                    }
                }

                // ignore all other events
                _ => {}
            }
        }

        // update the simulated display
        let mut rgb_screen = screen.color_converted();
        ux.update(&mut rgb_screen, &state, &theme);

        // sleep for a frame period
        const FPS_HZ: u64 = 10;
        const FRAME_PERIOD_MILLIS: u64 = 1000 / FPS_HZ;
        std::thread::sleep(std::time::Duration::from_millis(FRAME_PERIOD_MILLIS));
    }
}
