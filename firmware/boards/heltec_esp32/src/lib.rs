#![no_std]

// provide the shared crates via re-export
use common::*;
use soc_esp32::*;

// provice scheduling primitives
use embassy_time::Timer;
