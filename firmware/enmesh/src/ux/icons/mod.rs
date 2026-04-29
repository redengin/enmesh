// // provide the page common crates
// use crate::ux::pages::prelude::*;
//
// /// 12 x 5 pixel image with 1 bit per pixel.
// /// The data for each row is 12 bits long and is padded with zeros on the
// /// end because each row needs to contain a whole number of bytes.
// #[rustfmt::skip]
// const BATTERY_ICON_DATA_12X9: &[u8] = &[
//     0b00000000, 0b0000_0000,
//     0b00011110, 0b0000_0000,
//     0b01111111, 0b1000_0000,
//     0b01000000, 0b1000_0000,
//     0b01000000, 0b1000_0000,
//     0b01000000, 0b1000_0000,
//     0b01000000, 0b1000_0000,
//     0b01000000, 0b1000_0000,
//     0b01000000, 0b1000_0000,
//     0b01111111, 0b1000_0000,
// ];
// pub static BatteryIcon_12x9_RAW: ImageRaw<'_, BinaryColor> =
//     ImageRaw::<BinaryColor>::new(BATTERY_ICON_DATA_12X9, 12);
