// provide the shared crates via re-export
use common::*;

use embedded_graphics::prelude::*;
use embedded_graphics::pixelcolor::Rgb888;


pub struct ClippedText<'a, S> {
    text: embedded_graphics::text::Text<'a, S>,

}