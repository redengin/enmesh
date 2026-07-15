//! Error types for Reticulum operations.
//!
//! This module defines the error types used throughout Reticulum for
//! representing various failure conditions in cryptographic operations,
//! packet handling, networking, and other operations.
//!
//! # Usage
//!
//! Most Reticulum functions return `Result<T, RnsError>` to indicate
//! success or failure. Match on the error variant to handle different
//! failure scenarios:
//!
//! ```
//! use reticulum::error::RnsError;
//!
//! fn handle_error(err: RnsError) {
//!     match err {
//!         RnsError::OutOfMemory => {
//!             println!("Buffer too small or allocation failed");
//!         }
//!         RnsError::InvalidArgument => {
//!             println!("Invalid parameter provided");
//!         }
//!         RnsError::IncorrectSignature => {
//!             println!("Signature verification failed");
//!         }
//!         RnsError::IncorrectHash => {
//!             println!("Hash mismatch");
//!         }
//!         RnsError::CryptoError => {
//!             println!("Cryptographic operation failed");
//!         }
//!         RnsError::PacketError => {
//!             println!("Invalid or malformed packet");
//!         }
//!         RnsError::ConnectionError => {
//!             println!("Network connection failed");
//!         }
//!         RnsError::LinkClosed => {
//!             println!("Link is closed");
//!         }
//!     }
//! }
//! ```

/// Error types returned by Reticulum operations.
///
/// This enum represents the various error conditions that can occur
/// during Reticulum operations, including cryptographic operations,
/// packet handling, and network communication.
#[derive(Debug)]
pub enum RnsError {
    /// Indicates insufficient memory or a buffer that is too small.
    ///
    /// This error is returned when:
    /// - A provided buffer is too small to hold the output data
    /// - Memory allocation fails
    /// - A data structure's capacity is exceeded
    ///
    /// # Example
    ///
    /// ```
    /// use reticulum::buffer::StaticBuffer;
    ///
    /// let mut buffer: StaticBuffer<4> = StaticBuffer::new();
    /// let result = buffer.write(b"Hello World");
    /// assert!(result.is_err());
    /// // Returns Err(RnsError::OutOfMemory) - data doesn't fit
    /// ```
    OutOfMemory,

    /// An invalid argument was provided to a function.
    ///
    /// This error is returned when:
    /// - A required parameter is missing or null
    /// - A parameter value is out of valid range
    /// - A parameter format is invalid
    ///
    /// # Example
    ///
    /// ```
    /// use reticulum::crypt::fernet::{Fernet, PlainText};
    ///
    /// let fernet = Fernet::new_rand(rand_core::OsRng);
    /// let mut small_buf = [0u8; 10];  // Too small!
    /// let result = fernet.encrypt("test".into(), &mut small_buf);
    /// // Returns Err(RnsError::InvalidArgument) - buffer too small for overhead
    /// ```
    InvalidArgument,

    /// A signature verification failed.
    ///
    /// This error indicates that:
    /// - The cryptographic signature is invalid
    /// - The data has been tampered with
    /// - The wrong signing key was used
    ///
    /// # Example
    ///
    /// ```
    /// use reticulum::identity::PrivateIdentity;
    /// use reticulum::destination::DestinationAnnounce;
    /// use rand_core::OsRng;
    ///
    /// // Create a destination and announce
    /// let identity = PrivateIdentity::new_from_rand(OsRng);
    /// // ... create and verify announce packet
    /// // If signature doesn't match, returns RnsError::IncorrectSignature
    /// ```
    IncorrectSignature,

    /// A hash comparison failed.
    ///
    /// This error indicates that:
    /// - Computed hash doesn't match expected hash
    /// - Data has been modified
    /// - Wrong key or identifier was used
    IncorrectHash,

    /// A cryptographic operation failed.
    ///
    /// This is a general cryptographic error for failures such as:
    /// - Encryption/decryption failures
    /// - Key derivation failures
    /// - Invalid key formats
    /// - Random number generation failures
    ///
    /// # Example
    ///
    /// ```
    /// use reticulum::identity::Identity;
    /// use x25519_dalek::PublicKey;
    ///
    /// // Creating an identity with invalid key bytes
    /// let result = Identity::new_from_slices(&[0u8; 32], &[0u8; 32]);
    /// // May return Err(RnsError::CryptoError) for invalid key
    /// ```
    CryptoError,

    /// A packet is invalid, malformed, or inappropriate.
    ///
    /// This error is returned when:
    /// - Packet header is invalid or corrupted
    /// - Packet type is unexpected
    /// - Packet destination doesn't match
    /// - Packet is too small or truncated
    ///
    /// # Example
    ///
    /// ```
    /// use reticulum::packet::Packet;
    /// use reticulum::destination::DestinationAnnounce;
    ///
    /// // Validate an announce packet
    /// let result = DestinationAnnounce::validate(&packet);
    /// if result.is_err() {
    ///     // Could be RnsError::PacketError if packet isn't an announce
    /// }
    /// ```
    PacketError,

    /// A network connection error occurred.
    ///
    /// This error indicates failures such as:
    /// - Connection refused
    /// - Connection timeout
    /// - Network unreachable
    /// - Interface not available
    ///
    /// # Example
    ///
    /// ```
    /// // When connecting to a peer fails
    /// // Could return RnsError::ConnectionError
    /// ```
    ConnectionError,

    /// Attempted operation on a closed link.
    ///
    /// This error is returned when:
    /// - Trying to send on a closed link
    /// - Trying to receive on a closed link
    /// - Link was closed due to timeout or error
    ///
    /// # Example
    ///
    /// ```
    /// use reticulum::destination::link::Link;
    ///
    /// // If link is closed, operations return LinkClosed
    /// // let result = link.send(b"data");
    /// // Could return Err(RnsError::LinkClosed)
    /// ```
    LinkClosed,
}
