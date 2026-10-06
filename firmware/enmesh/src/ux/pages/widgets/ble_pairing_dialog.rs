/// provide page primitives
use crate::ux::pages::prelude::*;


pub struct BlePairingDialog {
    pub passkey: u32,
}

impl Drawable for BlePairingDialog {
    type Color = Rgb888;
    type Output = ();
    
    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color> {
        todo!()
    }
}

