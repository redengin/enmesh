//! Identity types for Reticulum networking.
//!
//! This module provides identity types for cryptographic operations in Reticulum,
//! including public identities, private identities, and key derivation.
//!
//! # Overview
//!
//! Reticulum uses a combination of X25519 (for key exchange) and Ed25519 (for
//! signatures) to provide end-to-end encryption. The identity system allows
//! parties to:
//!
//! - Create unique cryptographic identities
//! - Sign and verify messages
//! - Derive shared encryption keys
//! - Encrypt and decrypt data
//!
//! # Identity Types
//!
//! - [`Identity`]: Public identity with encryption and verification keys
//! - [`PrivateIdentity`]: Full identity with private keys for signing/decryption
//! - [`EmptyIdentity`]: No-op identity for plaintext communication
//! - [`GroupIdentity`]: Group-based identity for multi-party communication
//!
//! # Usage
//!
//! ```ignore
//! use reticulum::identity::{Identity, PrivateIdentity};
//! use rand_core::OsRng;
//!
//! // Create a new private identity (can sign and decrypt)
//! let private_id = PrivateIdentity::new_from_rand(OsRng);
//!
//! // Get the public identity for sharing with others
//! let public_id = private_id.as_identity();
//!
//! // Sign data
//! let data = b"Hello, Reticulum!";
//! let signature = private_id.sign(data);
//!
//! // Verify signature
//! public_id.verify(data, &signature).expect("valid signature");
//!
//! // Derive a shared key for encryption
//! let derived_key = private_id.derive_key(&public_id.public_key, None);
//! ```

#[cfg(feature = "std")]
use std::fmt::{self, Write};

// use alloc::fmt::Write;
// use alloc::string::String;
use hkdf::Hkdf;
use rand_core::CryptoRng;

use ed25519_dalek::{ed25519::signature::Signer, Signature, SigningKey, VerifyingKey};
use sha2::{Digest, Sha256};
use x25519_dalek::{EphemeralSecret, PublicKey, SharedSecret, StaticSecret};

use crate::{
    crypt::fernet::{Fernet, PlainText, Token},
    error::RnsError,
    hash::{AddressHash, Hash},
};

/// Length of a public key in bytes (32 bytes for Ed25519).
pub const PUBLIC_KEY_LENGTH: usize = ed25519_dalek::PUBLIC_KEY_LENGTH;

/// Derived key length in bytes when using AES-128 (32 bytes = 256 bits).
#[cfg(feature = "fernet-aes128")]
pub const DERIVED_KEY_LENGTH: usize = 256 / 8;

/// Derived key length in bytes when using AES-256 (64 bytes = 512 bits).
#[cfg(not(feature = "fernet-aes128"))]
pub const DERIVED_KEY_LENGTH: usize = 512 / 8;

/// Trait for identities that can encrypt data.
///
/// Implementors can encrypt plaintext using a derived key and ephemeral
/// key exchange. This is used for forward secrecy in Reticulum communications.
pub trait EncryptIdentity {
    /// Encrypts plaintext using the identity's public key and a derived key.
    ///
    /// # Arguments
    ///
    /// * `rng` - Random number generator for ephemeral keys
    /// * `text` - The plaintext data to encrypt
    /// * `derived_key` - The derived key for encryption
    /// * `out_buf` - Output buffer for the encrypted token
    ///
    /// # Returns
    ///
    /// * `Ok(&[u8])` - The encrypted token
    /// * `Err(RnsError)` - On encryption failure
    fn encrypt<'a, R: CryptoRng + Copy>(
        &self,
        rng: R,
        text: &[u8],
        derived_key: &DerivedKey,
        out_buf: &'a mut [u8],
    ) -> Result<&'a [u8], RnsError>;
}

/// Trait for identities that can decrypt data.
///
/// Implementors can decrypt tokens encrypted with their corresponding
/// public key using a derived key.
pub trait DecryptIdentity {
    /// Decrypts a token using the identity's private key and derived key.
    ///
    /// # Arguments
    ///
    /// * `rng` - Random number generator (if needed)
    /// * `data` - The encrypted token data
    /// * `derived_key` - The derived key for decryption
    /// * `out_buf` - Output buffer for the plaintext
    ///
    /// # Returns
    ///
    /// * `Ok(&[u8])` - The decrypted plaintext
    /// * `Err(RnsError)` - On decryption failure
    fn decrypt<'a, R: CryptoRng + Copy>(
        &self,
        rng: R,
        data: &[u8],
        derived_key: &DerivedKey,
        out_buf: &'a mut [u8],
    ) -> Result<&'a [u8], RnsError>;
}

/// Trait for identities that can provide an address hash.
///
/// This is used for network addressing - identities can be referenced
/// by their truncated hash in Reticulum packets.
pub trait HashIdentity {
    /// Returns the address hash slice for this identity.
    ///
    /// This is a truncated hash used for packet addressing.
    fn as_address_hash_slice(&self) -> &[u8];
}

/// A public identity in Reticulum.
///
/// Identity holds the public keys needed for:
/// - Verifying signatures from the holder
/// - Deriving shared encryption keys
///
/// An Identity can be shared publicly, while the corresponding
/// PrivateIdentity should be kept secret.
///
/// # Usage
///
/// ```
/// use reticulum::identity::Identity;
/// use reticulum::hash::AddressHash;
///
/// // Identity is typically obtained from a PrivateIdentity
/// // let private_id = PrivateIdentity::new_from_rand(OsRng);
/// // let identity = private_id.as_identity();
/// ```
#[derive(Copy, Clone)]
pub struct Identity {
    /// The X25519 public key for key exchange.
    pub public_key: PublicKey,
    /// The Ed25519 public key for signatures.
    pub verifying_key: VerifyingKey,
    /// The truncated address hash used for network addressing.
    pub address_hash: AddressHash,
}

impl Identity {
    /// Creates a new Identity from public keys.
    ///
    /// The address hash is automatically computed from both public keys.
    ///
    /// # Arguments
    ///
    /// * `public_key` - X25519 public key for key exchange
    /// * `verifying_key` - Ed25519 public key for verification
    ///
    /// # Example
    ///
    /// ```
    /// use reticulum::identity::Identity;
    /// use x25519_dalek::PublicKey;
    /// use ed25519_dalek::VerifyingKey;
    /// // Note: In practice, keys are generated cryptographically
    /// ```
    pub fn new(public_key: PublicKey, verifying_key: VerifyingKey) -> Self {
        let hash = Hash::new(
            Hash::generator()
                .chain_update(public_key.as_bytes())
                .chain_update(verifying_key.as_bytes())
                .finalize()
                .into(),
        );

        let address_hash = AddressHash::new_from_hash(&hash);

        Self {
            public_key,
            verifying_key,
            address_hash,
        }
    }

    /// Creates an Identity from raw key bytes.
    ///
    /// Each key must be exactly 32 bytes.
    ///
    /// # Arguments
    ///
    /// * `public_key` - 32 bytes for X25519 public key
    /// * `verifying_key` - 32 bytes for Ed25519 public key
    pub fn new_from_slices(public_key: &[u8], verifying_key: &[u8]) -> Self {
        let public_key = {
            let mut key_data = [0u8; PUBLIC_KEY_LENGTH];
            key_data.copy_from_slice(public_key);
            PublicKey::from(key_data)
        };

        let verifying_key = {
            let mut key_data = [0u8; PUBLIC_KEY_LENGTH];
            key_data.copy_from_slice(verifying_key);
            VerifyingKey::from_bytes(&key_data).unwrap_or_default()
        };

        Self::new(public_key, verifying_key)
    }

    /// Creates an Identity from a hex string.
    ///
    /// The hex string should contain both keys concatenated:
    /// 64 hex chars for public key + 64 hex chars for verifying key = 128 chars total.
    ///
    /// # Arguments
    ///
    /// * `hex_string` - Hex-encoded keys
    ///
    /// # Returns
    ///
    /// * `Ok(Identity)` - On success
    /// * `Err(RnsError::IncorrectHash)` - If string is too short
    ///
    /// # Example
    ///
    /// ```
    /// use reticulum::identity::Identity;
    ///
    /// let hex = "a1b2c3d4e5f60718293a4b5c6d7e8f90a1b2c3d4e5f60718293a4b5c6d7e8f90\
    ///            b2c3d4e5f60718293a4b5c6d7e8f90a1b2c3d4e5f60718293a4b5c6d7e8f90\
    ///            b2c3d4e5f60718293a4b5c6d7e8f90a1b2c3d4e5f60718293a4b5c6d7e8f90";
    /// let identity = Identity::new_from_hex_string(hex).unwrap();
    /// ```
    pub fn new_from_hex_string(hex_string: &str) -> Result<Self, RnsError> {
        if hex_string.len() < PUBLIC_KEY_LENGTH * 2 * 2 {
            return Err(RnsError::IncorrectHash);
        }

        let mut public_key_bytes = [0u8; PUBLIC_KEY_LENGTH];
        let mut verifying_key_bytes = [0u8; PUBLIC_KEY_LENGTH];

        for i in 0..PUBLIC_KEY_LENGTH {
            public_key_bytes[i] = u8::from_str_radix(&hex_string[i * 2..(i * 2) + 2], 16).unwrap();
            verifying_key_bytes[i] = u8::from_str_radix(
                &hex_string[PUBLIC_KEY_LENGTH * 2 + (i * 2)..PUBLIC_KEY_LENGTH * 2 + (i * 2) + 2],
                16,
            )
            .unwrap();
        }

        Ok(Self::new_from_slices(
            &public_key_bytes[..],
            &verifying_key_bytes[..],
        ))
    }

    /// Converts the identity to a hex string.
    ///
    /// Returns both public keys concatenated as hex characters.
    ///
    /// # Example
    ///
    /// ```
    /// use reticulum::identity::Identity;
    /// // let identity = Identity::new_from_hex_string(...).unwrap();
    /// // let hex = identity.to_hex_string();
    /// ```
    #[cfg(feature = "std")]
    pub fn to_hex_string(&self) -> String {
        let mut hex_string = String::with_capacity((PUBLIC_KEY_LENGTH * 2) * 2);

        for byte in self.public_key.as_bytes() {
            write!(&mut hex_string, "{:02x}", byte).unwrap();
        }

        for byte in self.verifying_key.as_bytes() {
            write!(&mut hex_string, "{:02x}", byte).unwrap();
        }

        hex_string
    }

    /// Returns the raw public key bytes.
    ///
    /// Returns 32 bytes representing the X25519 public key.
    pub fn public_key_bytes(&self) -> &[u8; PUBLIC_KEY_LENGTH] {
        self.public_key.as_bytes()
    }

    /// Returns the raw verifying key bytes.
    ///
    /// Returns 32 bytes representing the Ed25519 public key.
    pub fn verifying_key_bytes(&self) -> &[u8; PUBLIC_KEY_LENGTH] {
        self.verifying_key.as_bytes()
    }

    /// Verifies a signature over the given data.
    ///
    /// # Arguments
    ///
    /// * `data` - The data that was signed
    /// * `signature` - The signature to verify
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If signature is valid
    /// * `Err(RnsError::IncorrectSignature)` - If signature is invalid
    pub fn verify(&self, data: &[u8], signature: &Signature) -> Result<(), RnsError> {
        self.verifying_key
            .verify_strict(data, signature)
            .map_err(|_| RnsError::IncorrectSignature)
    }

    /// Derives a shared key for encryption.
    ///
    /// Uses ephemeral key exchange to create a shared secret
    /// that can be used for encryption.
    ///
    /// # Arguments
    ///
    /// * `rng` - Random number generator
    /// * `salt` - Optional salt for key derivation
    ///
    /// # Returns
    ///
    /// A DerivedKey for encryption/decryption
    pub fn derive_key<R: CryptoRng + Copy>(&self, rng: R, salt: Option<&[u8]>) -> DerivedKey {
        DerivedKey::new_from_ephemeral_key(rng, &self.public_key, salt)
    }
}

impl Default for Identity {
    fn default() -> Self {
        let empty_key = [0u8; PUBLIC_KEY_LENGTH];
        Self::new(PublicKey::from(empty_key), VerifyingKey::default())
    }
}

impl HashIdentity for Identity {
    fn as_address_hash_slice(&self) -> &[u8] {
        self.address_hash.as_slice()
    }
}

impl EncryptIdentity for Identity {
    fn encrypt<'a, R: CryptoRng + Copy>(
        &self,
        mut rng: R,
        text: &[u8],
        derived_key: &DerivedKey,
        out_buf: &'a mut [u8],
    ) -> Result<&'a [u8], RnsError> {
        let mut out_offset = 0;
        let ephemeral_key = EphemeralSecret::random_from_rng(&mut rng);
        {
            let ephemeral_public = PublicKey::from(&ephemeral_key);
            let ephemeral_public_bytes = ephemeral_public.as_bytes();

            if out_buf.len() >= ephemeral_public_bytes.len() {
                out_buf[..ephemeral_public_bytes.len()].copy_from_slice(ephemeral_public_bytes);
                out_offset += ephemeral_public_bytes.len();
            } else {
                return Err(RnsError::InvalidArgument);
            }
        }

        let token = Fernet::new_from_slices(
            &derived_key.as_bytes()[..16],
            &derived_key.as_bytes()[16..],
            rng,
        )
        .encrypt(PlainText::from(text), &mut out_buf[out_offset..])?;

        out_offset += token.as_bytes().len();

        Ok(&out_buf[..out_offset])
    }
}

/// An identity with no cryptographic capabilities.
///
/// EmptyIdentity is used for plain (unencrypted) destinations.
/// It passes data through unchanged and provides no authentication.
pub struct EmptyIdentity;

impl HashIdentity for EmptyIdentity {
    fn as_address_hash_slice(&self) -> &[u8] {
        &[]
    }
}

impl EncryptIdentity for EmptyIdentity {
    fn encrypt<'a, R: CryptoRng + Copy>(
        &self,
        _rng: R,
        text: &[u8],
        _derived_key: &DerivedKey,
        out_buf: &'a mut [u8],
    ) -> Result<&'a [u8], RnsError> {
        if text.len() > out_buf.len() {
            return Err(RnsError::OutOfMemory);
        }

        let result = &mut out_buf[..text.len()];
        result.copy_from_slice(text);
        Ok(result)
    }
}

impl DecryptIdentity for EmptyIdentity {
    fn decrypt<'a, R: CryptoRng + Copy>(
        &self,
        _rng: R,
        data: &[u8],
        _derived_key: &DerivedKey,
        out_buf: &'a mut [u8],
    ) -> Result<&'a [u8], RnsError> {
        if data.len() > out_buf.len() {
            return Err(RnsError::OutOfMemory);
        }

        let result = &mut out_buf[..data.len()];
        result.copy_from_slice(data);
        Ok(result)
    }
}

/// A full private identity with signing and decryption capabilities.
///
/// PrivateIdentity contains all the secret keys needed to:
/// - Sign messages (Ed25519)
/// - Decrypt received messages (X25519)
///
/// The public components can be extracted and shared with peers.
#[derive(Clone)]
pub struct PrivateIdentity {
    identity: Identity,
    private_key: StaticSecret,
    sign_key: SigningKey,
}

impl PrivateIdentity {
    /// Creates a new PrivateIdentity from existing keys.
    ///
    /// # Arguments
    ///
    /// * `private_key` - X25519 static secret for key exchange
    /// * `sign_key` - Ed25519 signing key for signatures
    pub fn new(private_key: StaticSecret, sign_key: SigningKey) -> Self {
        Self {
            identity: Identity::new((&private_key).into(), sign_key.verifying_key()),
            private_key,
            sign_key,
        }
    }

    /// Creates a new random PrivateIdentity.
    ///
    /// Generates fresh cryptographic keys for a new identity.
    ///
    /// # Arguments
    ///
    /// * `rng` - Random number generator
    ///
    /// # Example
    ///
    /// ```ignore
    /// use reticulum::identity::PrivateIdentity;
    /// use rand_core::OsRng;
    ///
    /// let identity = PrivateIdentity::new_from_rand(OsRng);
    /// ```
    pub fn new_from_rand<R: CryptoRng>(mut rng: R) -> Self {
        let sign_key = SigningKey::generate(&mut rng);
        let private_key = StaticSecret::random_from_rng(&mut rng);

        Self::new(private_key, sign_key)
    }

    /// Creates a new PrivateIdentity from a name (deterministic).
    ///
    /// The name is hashed to derive the private keys. This allows
    /// deterministic identity creation from a passphrase or name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name to derive keys from
    ///
    /// # Warning
    ///
    /// This method is not suitable for high-security identities as
    /// the keys can be recovered if the name is known.
    pub fn new_from_name(name: &str) -> Self {
        let hash = Hash::new_from_slice(name.as_bytes());
        let private_key = StaticSecret::from(hash.to_bytes());

        let hash = Hash::new_from_slice(hash.as_bytes());
        let sign_key = SigningKey::from_bytes(hash.as_bytes());

        Self::new(private_key, sign_key)
    }

    /// Creates a PrivateIdentity from a hex string.
    ///
    /// The hex string should contain both private keys concatenated:
    /// 64 hex chars for private key + 64 hex chars for signing key = 128 chars.
    ///
    /// # Arguments
    ///
    /// * `hex_string` - Hex-encoded private keys
    ///
    /// # Returns
    ///
    /// * `Ok(PrivateIdentity)` - On success
    /// * `Err(RnsError::IncorrectHash)` - If string is too short
    pub fn new_from_hex_string(hex_string: &str) -> Result<Self, RnsError> {
        if hex_string.len() < PUBLIC_KEY_LENGTH * 2 * 2 {
            return Err(RnsError::IncorrectHash);
        }

        let mut private_key_bytes = [0u8; PUBLIC_KEY_LENGTH];
        let mut sign_key_bytes = [0u8; PUBLIC_KEY_LENGTH];

        for i in 0..PUBLIC_KEY_LENGTH {
            private_key_bytes[i] = u8::from_str_radix(&hex_string[i * 2..(i * 2) + 2], 16).unwrap();
            sign_key_bytes[i] = u8::from_str_radix(
                &hex_string[PUBLIC_KEY_LENGTH * 2 + (i * 2)..PUBLIC_KEY_LENGTH * 2 + (i * 2) + 2],
                16,
            )
            .unwrap();
        }

        Ok(Self::new(
            StaticSecret::from(private_key_bytes),
            SigningKey::from_bytes(&sign_key_bytes),
        ))
    }

    /// Returns a reference to the signing key.
    ///
    /// Used for signing outbound messages.
    pub fn sign_key(&self) -> &SigningKey {
        &self.sign_key
    }

    /// Converts to the public Identity.
    ///
    /// This extracts just the public parts of the identity
    /// that can be shared with peers.
    pub fn into(&self) -> &Identity {
        &self.identity
    }

    /// Returns a reference to the public Identity.
    ///
    /// Same as `into()` but more explicit.
    pub fn as_identity(&self) -> &Identity {
        &self.identity
    }

    /// Returns the address hash of this identity.
    pub fn address_hash(&self) -> &AddressHash {
        &self.identity.address_hash
    }

    // FIXME
    // /// Converts the private identity to a hex string.
    // ///
    // /// Returns both private keys concatenated as hex.
    // ///
    // /// # Warning
    // ///
    // /// This exposes secret key material! Handle with extreme care.
    // pub fn to_hex_string(&self) -> String {
    //     let mut hex_string = String::with_capacity((PUBLIC_KEY_LENGTH * 2) * 2);

    //     for byte in self.private_key.as_bytes() {
    //         write!(&mut hex_string, "{:02x}", byte).unwrap();
    //     }

    //     for byte in self.sign_key.as_bytes() {
    //         write!(&mut hex_string, "{:02x}", byte).unwrap();
    //     }

    //     hex_string
    // }

    /// Verifies a signature over data.
    ///
    /// Delegates to the underlying Identity.
    pub fn verify(&self, data: &[u8], signature: &Signature) -> Result<(), RnsError> {
        self.identity.verify(data, signature)
    }

    /// Signs data with the Ed25519 signing key.
    ///
    /// # Arguments
    ///
    /// * `data` - The data to sign
    ///
    /// # Returns
    ///
    /// An Ed25519 signature (64 bytes)
    ///
    /// # Example
    ///
    /// ```ignore
    /// use reticulum::identity::PrivateIdentity;
    /// use rand_core::OsRng;
    ///
    /// let identity = PrivateIdentity::new_from_rand(OsRng);
    /// let signature = identity.sign(b"Hello, Reticulum!");
    /// ```
    pub fn sign(&self, data: &[u8]) -> Signature {
        self.sign_key.try_sign(data).expect("signature")
    }

    /// Performs X25519 key exchange with a peer's public key.
    ///
    /// # Arguments
    ///
    /// * `public_key` - The peer's X25519 public key
    ///
    /// # Returns
    ///
    /// A shared secret that can be used for key derivation
    pub fn exchange(&self, public_key: &PublicKey) -> SharedSecret {
        self.private_key.diffie_hellman(public_key)
    }

    /// Derives a shared encryption key from a peer's public key.
    ///
    /// Performs key exchange and HKDF key derivation in one step.
    ///
    /// # Arguments
    ///
    /// * `public_key` - The peer's public key
    /// * `salt` - Optional salt for HKDF
    ///
    /// # Returns
    ///
    /// A DerivedKey for encryption/decryption
    pub fn derive_key(&self, public_key: &PublicKey, salt: Option<&[u8]>) -> DerivedKey {
        DerivedKey::new_from_private_key(&self.private_key, public_key, salt)
    }
}

impl HashIdentity for PrivateIdentity {
    fn as_address_hash_slice(&self) -> &[u8] {
        self.identity.address_hash.as_slice()
    }
}

impl EncryptIdentity for PrivateIdentity {
    fn encrypt<'a, R: CryptoRng + Copy>(
        &self,
        rng: R,
        text: &[u8],
        derived_key: &DerivedKey,
        out_buf: &'a mut [u8],
    ) -> Result<&'a [u8], RnsError> {
        let mut out_offset = 0;

        let token = Fernet::new_from_slices(
            &derived_key.as_bytes()[..DERIVED_KEY_LENGTH / 2],
            &derived_key.as_bytes()[DERIVED_KEY_LENGTH / 2..],
            rng,
        )
        .encrypt(PlainText::from(text), &mut out_buf[out_offset..])?;

        out_offset += token.len();

        Ok(&out_buf[..out_offset])
    }
}

impl DecryptIdentity for PrivateIdentity {
    fn decrypt<'a, R: CryptoRng + Copy>(
        &self,
        _rng: R,
        data: &[u8],
        derived_key: &DerivedKey,
        out_buf: &'a mut [u8],
    ) -> Result<&'a [u8], RnsError> {
        if data.len() <= PUBLIC_KEY_LENGTH {
            return Err(RnsError::InvalidArgument);
        }

        let fernet = Fernet::new_from_slices(
            &derived_key.as_bytes()[..DERIVED_KEY_LENGTH / 2],
            &derived_key.as_bytes()[DERIVED_KEY_LENGTH / 2..],
            _rng,
        );

        let token = Token::from(data);

        let token = fernet.verify(token)?;

        let plain_text = fernet.decrypt(token, out_buf)?;

        Ok(plain_text.as_slice())
    }
}

/// A placeholder for group-based identities.
///
/// This is currently an empty struct for future group encryption support.
pub struct GroupIdentity {}

/// A derived key for symmetric encryption.
///
/// Derived keys are created from shared secrets using HKDF (HMAC-based
/// Key Derivation Function). They are used for Fernet encryption/decryption.
pub struct DerivedKey {
    key: [u8; DERIVED_KEY_LENGTH],
}

impl DerivedKey {
    /// Creates a new DerivedKey from a shared secret.
    ///
    /// # Arguments
    ///
    /// * `shared_key` - The X25519 shared secret
    /// * `salt` - Optional salt for HKDF
    pub fn new(shared_key: &SharedSecret, salt: Option<&[u8]>) -> Self {
        let mut key = [0u8; DERIVED_KEY_LENGTH];

        let _ = Hkdf::<Sha256>::new(salt, shared_key.as_bytes()).expand(&[], &mut key[..]);

        Self { key }
    }

    /// Creates an empty DerivedKey (all zeros).
    ///
    /// # Example
    ///
    /// ```
    /// use reticulum::identity::DerivedKey;
    ///
    /// let key = DerivedKey::new_empty();
    /// ```
    pub fn new_empty() -> Self {
        Self {
            key: [0u8; DERIVED_KEY_LENGTH],
        }
    }

    /// Creates a DerivedKey from a private key and peer's public key.
    ///
    /// Performs X25519 key exchange and HKDF in one step.
    ///
    /// # Arguments
    ///
    /// * `priv_key` - The local X25519 private key
    /// * `pub_key` - The peer's X25519 public key
    /// * `salt` - Optional salt for HKDF
    pub fn new_from_private_key(
        priv_key: &StaticSecret,
        pub_key: &PublicKey,
        salt: Option<&[u8]>,
    ) -> Self {
        Self::new(&priv_key.diffie_hellman(pub_key), salt)
    }

    /// Creates a DerivedKey using an ephemeral key pair.
    ///
    /// Generates a random ephemeral key pair, performs key exchange
    /// with the peer's public key, and derives a key.
    ///
    /// # Arguments
    ///
    /// * `rng` - Random number generator
    /// * `pub_key` - The peer's X25519 public key
    /// * `salt` - Optional salt for HKDF
    pub fn new_from_ephemeral_key<R: CryptoRng + Copy>(
        mut rng: R,
        pub_key: &PublicKey,
        salt: Option<&[u8]>,
    ) -> Self {
        let secret = EphemeralSecret::random_from_rng(&mut rng);
        let shared_key = secret.diffie_hellman(pub_key);
        Self::new(&shared_key, salt)
    }

    /// Returns the key as a byte array reference.
    pub fn as_bytes(&self) -> &[u8; DERIVED_KEY_LENGTH] {
        &self.key
    }

    /// Returns the key as a byte slice.
    pub fn as_slice(&self) -> &[u8] {
        &self.key[..]
    }
}



