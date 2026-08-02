//! Error types utilized by the display driver.
// use core::fmt::Debug;

// use embedded_graphics_core::geometry::Point;
// #[cfg(test)]
// use embedded_hal::spi;
// #[cfg(test)]
// use embedded_hal_mock::eh1::MockError;
// use thiserror::Error;

// #[cfg(test)]
// use crate::interface::SpiError;

/// Error that the display driver may encounter, with variants that describe which sequence of
/// commands it was executing or which graphics task it was performing when the error occurred.
///
/// Variants that have a single field of type `T` are errors that originate from the SPI interface.
// #[derive(Error, Debug)]
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
// pub enum Error<T> {
pub enum Error {
    // /// Error that occurred while resetting the controller.
    // #[error("failed to wake up display device")]
    // WakeUp(T),
    // /// Error that occurred while resetting the controller to its initial state.
    // #[error("failed to perform a full reset")]
    // FullReset(T),
    // /// Error that occurred while initializing the display device for full refresh mode.
    // #[error("failed to set up display device for full refresh mode")]
    // SetFullRefresh(T),
    // /// Error that occurred while transferring graphics data to the display device.
    // #[error("failed to write to display device random-access memory")]
    // WriteRam(T),
    // /// Error that occurred while executing the commands to update the display.
    // #[error("failed to execute display update sequence")]
    // UpdateDisplay(T),
    // /// Error that occurred while sending a request to enter deep sleep mode.
    // #[error("failed to enter deep sleep mode")]
    // DeepSleep(T),
    // /// Error that occurred while processing the graphics data to be drawn.
    // #[error(transparent)]
    // Draw(#[from] DrawError),
}

/// Error that occurs while processing the graphics data before being written to the display device
/// random-access memory.
// #[derive(Error, Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DrawError {
    // /// The point where a pixel was to be drawn falls outside of the display area.
    // #[error("invalid point")]
    // InvalidPoint(Point),
}

// #[cfg(test)]
// impl From<SpiError<spi::ErrorKind, MockError, MockError, MockError>>
//     for Error<SpiError<spi::ErrorKind, MockError, MockError, MockError>>
// {
//     fn from(_: SpiError<spi::ErrorKind, MockError, MockError, MockError>) -> Self {
//         unimplemented!();
//     }
// }