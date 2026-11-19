#![cfg_attr(not(feature = "std"), no_std)]

// The driver module needs this crate. The driver module is not enabled yet.
use display_interface as _;

// #![warn(missing_docs)]
// #![warn(missing_debug_implementations)]
// #![warn(missing_copy_implementations)]


pub mod ssd1680;