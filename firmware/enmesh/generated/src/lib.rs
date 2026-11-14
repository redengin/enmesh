//! This crate contains the fonts and images that build.rs generates from the assets folder.
#![no_std]

pub mod fonts {
    include!(concat!(env!("OUT_DIR"), "/fonts.rs"));
}

pub mod images {
    include!(concat!(env!("OUT_DIR"), "/images.rs"));
}
