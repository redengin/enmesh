#![cfg_attr(not(feature = "std"), no_std)]
//! A Rust port of the [Reticulum Python reference implementation](https://github.com/markqvist/reticulum),
//! the cryptography-based networking stack for building unstoppable
//! networks with LoRa, Packet Radio, WiFi and everything in between.
//!
//! More resources:
//!
//! * [Homepage](https://reticulum.network/)
//! * [Manual](https://reticulum.network/manual/index.html)
//! * [unsigned.io](https://unsigned.io/software/index.html)

pub struct Config {
    /// allow operation as a Transport Node
    pub enable_transport: bool,
    /// `announcements` will be retransmitted 'm+1' times
    pub m: u8,
    /// `announcement` bandwidth allocation
    pub announcement_bandwidth_percent: u8,
    /// `announcements` will be retried 'r' times
    pub r: u8,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            enable_transport: false,
            m: 128,
            r: 1,
            announcement_bandwidth_percent: 2,
        }
    }
}


// pub mod identity;
// pub mod packet;
// pub mod destination;

// utiltities
//------------------------------------------------------------------------------
pub mod error;
pub mod hash;
pub mod crypt;
pub mod buffer;
