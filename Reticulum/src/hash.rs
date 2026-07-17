//! Hash types for Reticulum networking.
//!
//! This module provides hash types used throughout Reticulum for addressing,
//! cryptographic operations, and data integrity. It uses SHA-256 as the
//! underlying hash algorithm.
//!
//! # Overview
//!
//! Reticulum uses two main hash types:
//! - [`Hash`]: A full SHA-256 hash (32 bytes) for general cryptographic use
//! - [`AddressHash`]: A truncated hash (16 bytes) for network addressing
//!
//! The shorter AddressHash is used for efficiency in packet headers while
//! still providing sufficient collision resistance for network addressing.
//!
//! # Usage
//!
//! ```ignore
//! use reticulum::hash::{Hash, AddressHash};
//! use rand_core::OsRng;
//!
//! // Create a full hash from data
//! let hash = Hash::new_from_slice(b"hello world");
//!
//! // Create an address hash from a full hash
//! let addr_hash = AddressHash::new_from_hash(&hash);
//!
//! // Create random hashes
//! let random_hash = Hash::new_from_rand(OsRng);
//! let random_addr = AddressHash::new_from_rand(OsRng);
//!
//! // Convert to hex string for display
//! println!("Address: {}", random_addr.to_hex_string());
//! ```

use rand_core::CryptoRng;
use sha2::{Digest, Sha256};

use crate::error::RnsError;

#[cfg(feature = "std")]
use std::fmt::{self, Write};

/// Size of an address hash in bytes (16 bytes).
///
/// Address hashes are truncated SHA-256 hashes used for network addressing.
/// The 16-byte size provides a good balance between compactness and
/// collision resistance for Reticulum's addressing scheme.
pub const ADDRESS_HASH_SIZE: usize = 16;

/// A truncated address hash (16 bytes).
///
/// Address hashes are used for network addressing in Reticulum. They are
/// derived from the full SHA-256 hash but truncated to 16 bytes for
/// efficiency in packet headers.
///
/// # Usage
///
/// Address hashes are used in packet destinations to identify endpoints
/// in the Reticulum network. They provide sufficient uniqueness for
/// network addressing while keeping packet headers compact.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash)]
pub struct AddressHash([u8; ADDRESS_HASH_SIZE]);
impl AddressHash {
    /// Creates an AddressHash from a raw 16-byte array.
    ///
    /// # Arguments
    ///
    /// * `hash` - A 16-byte array containing the address hash
    ///
    /// # Example
    ///
    /// ```ignore
    /// use reticulum::hash::AddressHash;
    ///
    /// let data = [0u8; 16];
    /// let addr = AddressHash::new(data);
    /// ```
    pub const fn new(hash: [u8; ADDRESS_HASH_SIZE]) -> Self {
        Self(hash)
    }

    /// Creates an AddressHash by hashing the input data.
    ///
    /// Computes SHA-256 of the input and truncates to 16 bytes.
    ///
    /// # Arguments
    ///
    /// * `data` - The data to hash
    ///
    /// # Example
    ///
    /// ```ignore
    /// use reticulum::hash::AddressHash;
    ///
    /// let addr = AddressHash::new_from_slice(b"destination name");
    /// assert_eq!(addr.as_slice().len(), 16);
    /// ```
    pub fn new_from_slice(data: &[u8]) -> Self {
        let mut hash = [0u8; ADDRESS_HASH_SIZE];
        create_hash(data, &mut hash);
        Self(hash)
    }

    /// Creates an AddressHash from a full Hash.
    ///
    /// Takes a 32-byte Hash and truncates it to 16 bytes.
    /// This is the standard way to create address hashes from
    /// identity or destination hashes.
    ///
    /// # Arguments
    ///
    /// * `hash` - The full Hash to truncate
    ///
    /// # Example
    ///
    /// ```ignore
    /// use reticulum::hash::{Hash, AddressHash};
    ///
    /// let full_hash = Hash::new_from_slice(b"some data");
    /// let addr_hash = AddressHash::new_from_hash(&full_hash);
    /// ```
    pub fn new_from_hash(hash: &Hash) -> Self {
        let mut address_hash = [0u8; ADDRESS_HASH_SIZE];
        address_hash.copy_from_slice(&hash.0[0..ADDRESS_HASH_SIZE]);
        Self(address_hash)
    }

    /// Creates a random AddressHash.
    ///
    /// Generates random bytes, hashes them, and truncates to 16 bytes.
    ///
    /// # Arguments
    ///
    /// * `rng` - A random number generator
    ///
    /// # Example
    ///
    /// ```ignore
    /// use reticulum::hash::AddressHash;
    /// use rand_core::OsRng;
    ///
    /// let addr = AddressHash::new_from_rand(OsRng);
    /// ```
    pub fn new_from_rand<R: CryptoRng>(rng: R) -> Self {
        Self::new_from_hash(&Hash::new_from_rand(rng))
    }

    /// Creates an AddressHash from a hex string.
    ///
    /// Parses a hexadecimal string representation of the address hash.
    /// The string must be at least 32 hex characters (16 bytes).
    ///
    /// # Arguments
    ///
    /// * `hex_string` - A hex string (e.g., "a1b2c3d4e5f60718293a4b5c6d7e8f90")
    ///
    /// # Returns
    ///
    /// * `Ok(AddressHash)` - On successful parsing
    /// * `Err(RnsError::IncorrectHash)` - If the hex string is too short
    ///
    /// # Example
    ///
    /// ```ignore
    /// use reticulum::hash::AddressHash;
    ///
    /// let addr = AddressHash::new_from_hex_string("a1b2c3d4e5f60718293a4b5c6d7e8f90").unwrap();
    /// ```
    pub fn new_from_hex_string(hex_string: &str) -> Result<Self, RnsError> {
        if hex_string.len() < ADDRESS_HASH_SIZE * 2 {
            return Err(RnsError::IncorrectHash);
        }

        let mut bytes = [0u8; ADDRESS_HASH_SIZE];

        for i in 0..ADDRESS_HASH_SIZE {
            bytes[i] = u8::from_str_radix(&hex_string[i * 2..(i * 2) + 2], 16).unwrap();
        }

        Ok(Self(bytes))
    }

    /// Creates an empty AddressHash (all zeros).
    ///
    /// # Example
    ///
    /// ```ignore
    /// use reticulum::hash::AddressHash;
    ///
    /// let addr = AddressHash::new_empty();
    /// assert_eq!(addr.as_slice(), &[0u8; 16]);
    /// ```
    pub const fn new_empty() -> Self {
        Self([0u8; ADDRESS_HASH_SIZE])
    }

    /// Returns a slice view of the address hash data.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use reticulum::hash::AddressHash;
    ///
    /// let addr = AddressHash::new_from_slice(b"test");
    /// let slice = addr.as_slice();
    /// assert_eq!(slice.len(), 16);
    /// ```
    pub fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }

    /// Returns a mutable slice view of the address hash data.
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        &mut self.0[..]
    }

    /// Returns the length of the address hash (16 bytes).
    ///
    /// # Example
    ///
    /// ```ignore
    /// use reticulum::hash::AddressHash;
    ///
    /// let addr = AddressHash::new_from_slice(b"test");
    /// assert_eq!(addr.len(), 16);
    /// ```
    pub const fn len(&self) -> usize {
        self.0.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        *self == Self::new_empty()
    }

    /// Converts the address hash to a hexadecimal string.
    ///
    /// Returns a 32-character lowercase hex string representation
    /// of the 16-byte address hash.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use reticulum::hash::AddressHash;
    ///
    /// let addr = AddressHash::new_from_slice(b"test");
    /// let hex = addr.to_hex_string();
    /// assert_eq!(hex.len(), 32);
    /// ```
    #[cfg(feature = "std")]
    pub fn to_hex_string(&self) -> String {
        let mut hex_string = String::with_capacity(ADDRESS_HASH_SIZE * 2);

        for byte in self.0 {
            write!(&mut hex_string, "{:02x}", byte).unwrap();
        }

        hex_string
    }
}
#[cfg(feature = "std")]
impl fmt::Display for AddressHash {
    /// Formats the address hash as a Reticulum-style address string.
    ///
    /// The format is `/` followed by 32 hex characters (lowercase),
    /// followed by another `/`.
    ///
    /// Example: `/a1b2c3d4e5f60718293a4b5c6d7e8f90/`
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "/")?;
        for data in self.0.iter() {
            write!(f, "{:0>2x}", data)?;
        }
        write!(f, "/")?;

        Ok(())
    }
}

/// Size of a SHA-256 hash in bytes (32 bytes).
pub const HASH_SIZE: usize = 32;

/// A SHA-256 hash (32 bytes).
///
/// This is the standard hash type used throughout Reticulum for cryptographic
/// operations. It represents a full SHA-256 digest which is 32 bytes long.
///
/// # Memory Layout
///
/// The hash is stored as a fixed-size array of 32 bytes on the stack.
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct Hash([u8; HASH_SIZE]);
impl Hash {
    /// Creates a new SHA-256 hasher for incremental hashing.
    ///
    /// This returns a `Sha256` instance that can be used to compute
    /// a hash incrementally by calling `.chain_update()` with data chunks.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use reticulum::hash::Hash;
    ///
    /// let hash = Hash::generator()
    ///     .chain_update(b"part1")
    ///     .chain_update(b"part2")
    ///     .finalize();
    /// ```
    pub fn generator() -> Sha256 {
        Sha256::new()
    }

    /// Creates a Hash from a raw 32-byte array.
    ///
    /// # Arguments
    ///
    /// * `hash` - A 32-byte array containing the hash value
    ///
    /// # Example
    ///
    /// ```ignore
    /// use reticulum::hash::Hash;
    ///
    /// let data = [0u8; 32];
    /// let hash = Hash::new(data);
    /// ```
    pub const fn new(hash: [u8; HASH_SIZE]) -> Self {
        Self(hash)
    }

    /// Creates a new Hash with all zeros.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use reticulum::hash::Hash;
    ///
    /// let hash = Hash::new_empty();
    /// assert_eq!(hash.as_slice(), &[0u8; 32]);
    /// ```
    pub const fn new_empty() -> Self {
        Self([0u8; HASH_SIZE])
    }

    /// Creates a Hash by hashing the input data.
    ///
    /// Computes SHA-256 of the input slice and returns a new Hash.
    ///
    /// # Arguments
    ///
    /// * `data` - The data to hash
    ///
    /// # Example
    ///
    /// ```ignore
    /// use reticulum::hash::Hash;
    ///
    /// let hash = Hash::new_from_slice(b"hello world");
    /// assert_eq!(hash.as_slice().len(), 32);
    /// ```
    pub fn new_from_slice(data: &[u8]) -> Self {
        let mut hash = [0u8; HASH_SIZE];
        create_hash(data, &mut hash);
        Self(hash)
    }

    /// Creates a Hash from random data.
    ///
    /// Generates random bytes, hashes them, and returns the hash.
    /// This is useful for creating unique identifiers or nonces.
    ///
    /// # Arguments
    ///
    /// * `rng` - A random number generator
    ///
    /// # Example
    ///
    /// ```ignore
    /// use reticulum::hash::Hash;
    /// use rand_core::OsRng;
    ///
    /// let hash = Hash::new_from_rand(OsRng);
    /// ```
    pub fn new_from_rand<R: CryptoRng>(mut rng: R) -> Self {
        let mut hash = [0u8; HASH_SIZE];
        let mut data = [0u8; HASH_SIZE];

        rng.fill_bytes(&mut data[..]);

        create_hash(&data, &mut hash);
        Self(hash)
    }

    /// Returns a slice view of the hash data.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use reticulum::hash::Hash;
    ///
    /// let hash = Hash::new_from_slice(b"test");
    /// let slice: &[u8] = hash.as_slice();
    /// assert_eq!(slice.len(), 32);
    /// ```
    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }

    /// Returns a reference to the underlying 32-byte array.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use reticulum::hash::Hash;
    ///
    /// let hash = Hash::new_from_slice(b"test");
    /// let bytes: &[u8; 32] = hash.as_bytes();
    /// ```
    pub fn as_bytes(&self) -> &[u8; HASH_SIZE] {
        &self.0
    }

    /// Consumes the Hash and returns the underlying 32-byte array.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use reticulum::hash::Hash;
    ///
    /// let hash = Hash::new_from_slice(b"test");
    /// let bytes: [u8; 32] = hash.to_bytes();
    /// ```
    pub fn to_bytes(&self) -> [u8; HASH_SIZE] {
        self.0
    }

    /// Returns a mutable slice view of the hash data.
    ///
    /// Allows direct modification of the hash bytes.
    pub fn as_slice_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}
#[cfg(feature = "std")]
impl fmt::Display for Hash {
    /// Formats the hash as a hexadecimal string (lowercase).
    ///
    /// Example: `a1b2c3d4e5f60718293a4b5c6d7e8f90a1b2c3d4e5f60718293a4b5c6d7e8f90`
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for data in self.0.iter() {
            write!(f, "{:0>2x}", data)?;
        }

        Ok(())
    }
}

/// Creates a SHA-256 hash of the input data.
///
/// This is a utility function that computes SHA-256 and writes the result
/// to the provided output buffer, truncating if necessary.
///
/// # Arguments
///
/// * `data` - The data to hash
/// * `out` - Output buffer to write hash to (will be truncated to fit)
///
/// # Example
///
/// ```ignore
/// use reticulum::hash::{create_hash, HASH_SIZE};
///
/// let mut hash_out = [0u8; HASH_SIZE];
/// create_hash(b"test data", &mut hash_out);
/// ```
pub fn create_hash(data: &[u8], out: &mut [u8]) {
    out.copy_from_slice(
        &Sha256::new().chain_update(data).finalize().as_slice()
            [..core::cmp::min(out.len(), HASH_SIZE)],
    );
}
