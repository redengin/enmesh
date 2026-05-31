// use the common page crates
use crate::ux::pages::prelude::*;

#[derive(PartialEq, Eq)]
enum Pages {
    Home,
    MeshCore,
    Meshtastic,
    // Hibernate,
}
impl Pages {
    fn next(&self) -> Self {
        match self {
            Pages::Home => Pages::MeshCore,
            Pages::MeshCore => Pages::Meshtastic,
            Pages::Meshtastic => Pages::Home,
        }
    }

    fn previous(&self) -> Self {
        match self {
            Pages::Home => Pages::Meshtastic,
            Pages::Meshtastic => Pages::MeshCore,
            Pages::MeshCore => Pages::Home,
        }
    }
}

pub struct Ux {
    /// use enum to track current page
    current_page: Pages,
    // pages
    home_page: pages::Home,
    meshcore_page: pages::MeshCore,
    meshtastic_page: pages::Meshtastic,
    // FIXME
    // hibernate_page: pages::Hibernate,
}

impl Ux {
    pub fn new() -> Self {
        Self {
            current_page: Pages::Home,
            home_page: pages::Home::new(),
            meshcore_page: pages::MeshCore::new(),
            meshtastic_page: pages::Meshtastic::new(),
            // hibernate_page: pages::Home::new(),
        }
    }

    fn tab_bar_refresh(&self, display: &mut impl DrawTargetExt<Color = Rgb888>, theme: &Theme) {
        let _ = display.clear(theme.background.into());

        const SELECTED: &str = "^";
        const NOT_SELECTED: &str = "-";
        LinearLayout::horizontal(
            Chain::new(Text::new(
                if self.current_page == Pages::Home {
                    SELECTED
                } else {
                    NOT_SELECTED
                },
                Point::zero(),
                theme.text_style,
            ))
            .append(Text::new(
                if self.current_page == Pages::MeshCore {
                    SELECTED
                } else {
                    NOT_SELECTED
                },
                Point::zero(),
                theme.text_style,
            ))
            .append(Text::new(
                if self.current_page == Pages::Meshtastic {
                    SELECTED
                } else {
                    NOT_SELECTED
                },
                Point::zero(),
                theme.text_style,
            )), // .append(Text::new(
                //     if self.current_page == Pages::Hibernate{ SELECTED } else { NOT_SELECTED },
                //     Point::zero(), theme.text_style,
                // )),
        )
        .with_spacing(DistributeFill(display.bounding_box().size.width))
        .arrange()
        .align_to(&display.bounding_box(), horizontal::Left, vertical::Bottom)
        .draw(display)
        .ok();
    }
}

impl Page for Ux {
    /// repaint the whole screen
    fn refresh(
        &mut self,
        screen: &mut impl DrawTargetExt<Color = Rgb888>,
        model: &crate::State,
        theme: &Theme,
    ) {
        // get the screen size
        let bounding_box = screen.bounding_box();
        // reserve space for the tab_bar
        let tab_bar_height = theme.text_style.line_height();

        // create a cropped display for the page content (excluding the tab bar)
        let mut page_display = screen.cropped(&Rectangle {
            top_left: Point::zero(),
            size: Size::new(
                bounding_box.size.width,
                bounding_box.size.height - tab_bar_height,
            ),
        });

        // refresh the current page
        match self.current_page {
            Pages::Home => self.home_page.refresh(&mut page_display, model, &theme),
            Pages::MeshCore => self.meshcore_page.refresh(&mut page_display, model, &theme),
            Pages::Meshtastic => self
                .meshtastic_page
                .refresh(&mut page_display, model, &theme),
            // Pages::Hibernate => self
            //     .hibernate_page
            //     .refresh(&mut page_display, model, &theme),
        }

        // refresh the tab bar inside a cropped display
        let mut tab_bar_display = screen.cropped(&Rectangle {
            top_left: Point::new(0, (bounding_box.size.height - tab_bar_height) as i32),
            size: Size::new(bounding_box.size.width, tab_bar_height),
        });
        self.tab_bar_refresh(&mut tab_bar_display, &theme);

        // if ble pairing show a dialog with the passkey
        match model.ble_status {
            crate::state::BleStatus::Pairing { passkey } => {
                let style = PrimitiveStyleBuilder::new()
                    .stroke_color(theme.color)
                    .stroke_width(1)
                    .fill_color(theme.background)
                    .build();

                let frame = Rectangle::new(Point::zero(), Size::new(120, 40))
                    .into_styled(style)
                    .align_to(&bounding_box, horizontal::Center, vertical::Center);
                frame.draw(screen) .ok();
                let passkey_style = MonoTextStyle::new(&FONT_10X20, theme.color);
                LinearLayout::vertical(
                    Chain::new(Text::new("BLE Pairing", Point::zero(), theme.text_style))
                        .append(Text::new(format!(6; "{passkey}").unwrap().as_str(), Point::zero(), passkey_style))
                )
                .with_alignment(horizontal::Center)
                .arrange()
                .align_to(&frame, horizontal::Center, vertical::Center)
                .draw(screen).ok();
            }
            _ => { /* no dialog */ }
        }
    }

    /// handle HidEvent
    fn handle_event(&mut self, event: &HidEvent) -> bool {
        let handled = match self.current_page {
            Pages::Home => self.home_page.handle_event(&event),
            // FIXME handle all pages
            _ => false,
        };
        if !handled {
            match event {
                HidEvent::Next => {
                    self.current_page = self.current_page.next();
                }
                HidEvent::Previous => {
                    self.current_page = self.current_page.previous();
                }
                _ => {}
            }
        }
        // UX always handles the event
        true
    }

    /// update the display
    /// * only needs to update changed items
    fn update(
        &mut self,
        display: &mut impl DrawTargetExt<Color = Rgb888>,
        model: &crate::State,
        theme: &Theme,
    ) {
        // FIXME for now just do a full refresh
        self.refresh(display, model, theme);
    }
}
