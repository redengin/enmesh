use embedded_graphics::geometry::Size;

pub struct EPD {
    size: Size,
}
impl EPD {
    pub fn new(size: Size) -> Self {
        Self {
            size
        }
    }
}
