//! Cryptographic primitives for Reticulum.
//!
//! This module provides the cryptographic functionality used throughout
//! Reticulum for encryption, decryption, and authentication.
//!
//! # Modules
//!
//! - [`fernet`]: Fernet symmetric encryption implementation
//!
//! # Overview
//!
//! Reticulum uses a modified Fernet encryption scheme for symmetric
//! encryption of packets. The implementation uses AES-CBC with PKCS7
//! padding and HMAC-SHA256 for message authentication.
//!
//! For more details on the Fernet implementation, see the [`fernet`] module.
//!
//! # Usage
//!
//! ```ignore
//! use reticulum::crypt::fernet::{Fernet, PlainText};
//! use rand_core::OsRng;
//!
//! // Create a new Fernet instance with random keys
//! let fernet = Fernet::new_rand(OsRng);
//!
//! // Encrypt data
//! let mut buf = [0u8; 256];
//! let token = fernet.encrypt("Hello, Reticulum!".into(), &mut buf).unwrap();
//!
//! // Verify and decrypt
//! let verified = fernet.verify(token).unwrap();
//! let mut out = [0u8; 256];
//! let plaintext = fernet.decrypt(verified, &mut out).unwrap();
//! assert_eq!(plaintext.as_slice(), b"Hello, Reticulum!");
//! ```

pub mod fernet;