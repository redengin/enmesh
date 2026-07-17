//! Fernet encryption implementation for Reticulum.
//!
//! This module provides a modified implementation of the Fernet symmetric
//! encryption specification. Fernet is a symmetric encryption format that
//! uses AES-128-CBC or AES-256-CBC with PKCS7 padding and HMAC-SHA256 for
//! authentication.
//!
//! # Modified Implementation
//!
//! This is a slightly modified implementation of the [Fernet spec](https://github.com/fernet/spec/blob/master/Spec.md).
//! The original Fernet specification includes a one-byte VERSION field and
//! eight-byte TIMESTAMP field at the start of each token. These fields are
//! not relevant to Reticulum and are therefore stripped from this implementation
//! to reduce overhead and prevent leakage of initiator metadata.
//!
//! # Token Format
//!
//! The modified token format is:
//! ```text
//! [IV (16 bytes)] [Ciphertext (variable)] [HMAC-SHA256 (32 bytes)]
//! ```
//!
//! Total overhead: 48 bytes (16 byte IV + 32 byte HMAC)
//!
//! # Usage
//!
//! ```
//! use reticulum::crypt::fernet::{Fernet, PlainText};
//! use rand_core::OsRng;
//!
//! // Create a new Fernet instance with random keys
//! let fernet = Fernet::new_rand(OsRng);
//!
//! // Encrypt a message
//! let mut out_buf = [0u8; 4096];
//! let token = fernet.encrypt("Hello, Reticulum!".into(), &mut out_buf).unwrap();
//!
//! // Verify the token (authenticates and checks integrity)
//! let verified = fernet.verify(token).unwrap();
//!
//! // Decrypt the verified token
//! let mut in_buf = [0u8; 4096];
//! let plaintext = fernet.decrypt(verified, &mut in_buf).unwrap();
//! assert_eq!(plaintext.as_slice(), b"Hello, Reticulum!");
//! ```

use core::cmp;
use core::convert::From;

use aes::cipher::block_padding::Pkcs7;
use aes::cipher::{BlockModeDecrypt, BlockModeEncrypt};
use aes::cipher::Key;
// use aes::cipher::Unsigned;
// use cbc::cipher::BlockEncryptMut;
use cbc::cipher::KeyIvInit;
// use crypto_common::{IvSizeUser, KeySizeUser, OutputSizeUser};
use hmac::{Hmac, Mac, KeyInit};
use rand_core::CryptoRng;
use sha2::Sha256;

use crate::error::RnsError;

/// A plaintext message to be encrypted.
///
/// This is a newtype wrapper around a byte slice that clearly indicates
/// the data is unencrypted and ready for encryption.
pub struct PlainText<'a>(&'a [u8]);
impl<'a> PlainText<'a> {
    /// Returns the plaintext data as a byte slice.
    ///
    /// # Example
    ///
    /// ```
    /// use reticulum::crypt::fernet::PlainText;
    ///
    /// let pt = PlainText::from(b"hello");
    /// assert_eq!(pt.as_slice(), b"hello");
    /// ```
    pub fn as_slice(&self) -> &'a [u8] {
        self.0
    }
}
impl<'a> From<&'a str> for PlainText<'a> {
    /// Creates a PlainText from a string slice.
    ///
    /// # Example
    ///
    /// ```
    /// use reticulum::crypt::fernet::PlainText;
    ///
    /// let pt: PlainText = "hello world".into();
    /// assert_eq!(pt.as_slice(), b"hello world");
    /// ```
    fn from(item: &'a str) -> Self {
        Self(item.as_bytes())
    }
}
impl<'a> From<&'a [u8]> for PlainText<'a> {
    /// Creates a PlainText from a byte slice.
    ///
    /// # Example
    ///
    /// ```
    /// use reticulum::crypt::fernet::PlainText;
    ///
    /// let data: &[u8] = b"binary data";
    /// let pt: PlainText = data.into();
    /// assert_eq!(pt.as_slice(), b"binary data");
    /// ```
    fn from(item: &'a [u8]) -> Self {
        Self(item)
    }
}

/// A verified token that has passed HMAC authentication.
///
/// This represents a token that has been verified to be authentic and
/// not tampered with. It can be safely decrypted. The lifetime `'a`
/// ties the verified token to the underlying data.
pub struct VerifiedToken<'a>(&'a [u8]);

/// An encrypted token containing ciphertext and authentication tag.
///
/// This is the output of encryption - a self-contained token that can
/// be transmitted safely and verified/decrypted by the recipient who
/// has the same Fernet key.
pub struct Token<'a>(&'a [u8]);
impl<'a> Token<'a> {
    /// Returns the token data as a byte slice.
    ///
    /// # Example
    ///
    /// ```
    /// use reticulum::crypt::fernet::Token;
    ///
    /// let token = Token::from(b"encrypted data here");
    /// assert_eq!(token.as_bytes(), b"encrypted data here");
    /// ```
    pub fn as_bytes(&self) -> &'a [u8] {
        self.0
    }

    /// Returns the length of the token in bytes.
    ///
    /// # Example
    ///
    /// ```
    /// use reticulum::crypt::fernet::Token;
    ///
    /// let token = Token::from(b"abc");
    /// assert_eq!(token.len(), 3);
    /// ```
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
impl<'a> From<&'a [u8]> for Token<'a> {
    /// Creates a Token from a byte slice.
    ///
    /// # Example
    ///
    /// ```
    /// use reticulum::crypt::fernet::Token;
    ///
    /// let data: &[u8] = b"token bytes";
    /// let token: Token = data.into();
    /// ```
    fn from(item: &'a [u8]) -> Self {
        Self(item)
    }
}



/// AES-128-CBC when the "fernet-aes128" feature is enabled, otherwise AES-256-CBC.
#[cfg(feature = "fernet-aes128")]
type AesAlgo = aes::Aes128;
#[cfg(not(feature = "fernet-aes128"))]
type AesAlgo = aes::Aes256;
/// Size of the AES key in bytes (16 for AES-128, 32 for AES-256).
#[cfg(feature = "fernet-aes128")]
const AES_KEY_SIZE: usize = 16;
#[cfg(not(feature = "fernet-aes128"))]
const AES_KEY_SIZE: usize = 32;

/// AES-CBC encryptor type alias using the configured algorithm.
type AesCbcEnc = cbc::Encryptor<AesAlgo>;
/// AES-CBC decryptor type alias using the configured algorithm.
type AesCbcDec = cbc::Decryptor<AesAlgo>;
/// AES key type alias using the configured algorithm.
type AesKey = Key<AesAlgo>;

/// HMAC-SHA256 type alias for message authentication.
type HmacSha256 = Hmac<Sha256>;

/// Size of the HMAC-SHA256 output in bytes (32 bytes).
const HMAC_OUT_SIZE: usize = 32;

/// Size of the IV for AES-CBC (16 bytes).
const IV_KEY_SIZE: usize = 16;
/// Total overhead: IV + HMAC (48 bytes).
const FERNET_OVERHEAD_SIZE: usize = IV_KEY_SIZE + HMAC_OUT_SIZE;





// This class provides a slightly modified implementation of the Fernet spec
// found at: https://github.com/fernet/spec/blob/master/Spec.md
//
// According to the spec, a Fernet token includes a one byte VERSION and
// eight byte TIMESTAMP field at the start of each token. These fields are
// not relevant to Reticulum. They are therefore stripped from this
// implementation, since they incur overhead and leak initiator metadata.

/// Fernet symmetric encryption instance.
///
/// Fernet provides authenticated encryption using AES-CBC with HMAC-SHA256
/// for message authentication. It guarantees both confidentiality (through
/// AES encryption) and integrity (through HMAC verification).
///
/// # Key Configuration
///
/// A Fernet instance requires two keys:
/// - **Sign key**: Used for HMAC-SHA256 authentication (must be kept secret)
/// - **Enc key**: Used for AES-CBC encryption (must be kept secret)
///
/// Both keys can be the same (64 bytes for AES-256) or different (32 bytes each).
///
/// # Random Generation
///
/// Use [`Fernet::new_rand`] to create a new instance with randomly generated keys.
/// This is recommended for new code.
///
/// # Example
///
/// ```
/// use reticulum::crypt::fernet::{Fernet, PlainText};
/// use rand_core::OsRng;
///
/// let fernet = Fernet::new_rand(OsRng);
/// let mut buf = [0u8; 256];
/// let token = fernet.encrypt("secret message".into(), &mut buf).unwrap();
/// let verified = fernet.verify(token).unwrap();
/// let mut out = [0u8; 256];
/// let plaintext = fernet.decrypt(verified, &mut out).unwrap();
/// assert_eq!(plaintext.as_slice(), b"secret message");
/// ```
pub struct Fernet<R: CryptoRng> {
    rng: R,
    /// Key used for HMAC-SHA256 authentication.
    sign_key: [u8; AES_KEY_SIZE],
    /// Key used for AES-CBC encryption.
    enc_key: AesKey,
}
impl<R: CryptoRng> Fernet<R> {
    /// Creates a new Fernet instance with explicit keys.
    ///
    /// # Arguments
    ///
    /// * `sign_key` - The HMAC-SHA256 signing key (16 bytes for AES-128, 32 for AES-256)
    /// * `enc_key` - The AES encryption key
    /// * `rng` - A random number generator
    ///
    /// # Example
    ///
    /// ```
    /// use reticulum::crypt::fernet::Fernet;
    /// use rand_core::OsRng;
    ///
    /// let mut sign_key = [0u8; 32];
    /// let mut enc_key = [0u8; 32];
    /// OsRng.fill_bytes(&mut sign_key);
    /// OsRng.fill_bytes(&mut enc_key);
    ///
    /// let fernet = Fernet::new(sign_key, enc_key.into(), OsRng);
    /// ```
    pub fn new(sign_key: [u8; AES_KEY_SIZE], enc_key: AesKey, rng: R) -> Self {
        Self {
            rng,
            sign_key,
            enc_key,
        }
    }

    /// Creates a new Fernet instance from raw key slices.
    ///
    /// If the provided slices are shorter than required, they are zero-padded.
    /// If longer, they are truncated.
    ///
    /// # Arguments
    ///
    /// * `sign_key` - Raw bytes for the HMAC signing key
    /// * `enc_key` - Raw bytes for the AES encryption key
    /// * `rng` - A random number generator
    ///
    /// # Example
    ///
    /// ```
    /// use reticulum::crypt::fernet::Fernet;
    /// use rand_core::OsRng;
    ///
    /// let fernet = Fernet::new_from_slices(b"my_sign_key_12345678901234", b"my_enc_key_12345678901234", OsRng);
    /// ```
    pub fn new_from_slices(sign_key: &[u8], enc_key: &[u8], rng: R) -> Self {
        let mut sign_key_bytes = [0u8; AES_KEY_SIZE];
        sign_key_bytes[..cmp::min(AES_KEY_SIZE, sign_key.len())].copy_from_slice(sign_key);

        let mut enc_key_bytes = [0u8; AES_KEY_SIZE];
        enc_key_bytes[..cmp::min(AES_KEY_SIZE, enc_key.len())].copy_from_slice(enc_key);

        Self {
            rng,
            sign_key: sign_key_bytes,
            enc_key: enc_key_bytes.into(),
        }
    }

    /// Creates a new Fernet instance with randomly generated keys.
    ///
    /// This is the recommended way to create a new Fernet instance for
    /// fresh encryption. Both the signing key and encryption key are
    /// generated randomly using the provided RNG.
    ///
    /// # Arguments
    ///
    /// * `rng` - A random number generator (e.g., `rand_core::OsRng`)
    ///
    /// # Example
    ///
    /// ```
    /// use reticulum::crypt::fernet::Fernet;
    /// use rand_core::OsRng;
    ///
    /// let fernet = Fernet::new_rand(OsRng);
    /// ```
    pub fn new_rand(mut rng: R) -> Self {
        let mut sign_key = [0u8; AES_KEY_SIZE];
        rng.fill_bytes(&mut sign_key);
        let enc_key = AesCbcEnc::generate_key(&mut rng);

        Self {
            rng,
            sign_key,
            enc_key,
        }
    }

    /// Encrypts plaintext data and produces an authenticated token.
    ///
    /// This method encrypts the plaintext using AES-CBC with a randomly
    /// generated IV, then computes an HMAC-SHA256 tag over the IV and
    /// ciphertext. The output token is the concatenation:
    /// `[IV (16 bytes)] [Ciphertext (variable)] [HMAC (32 bytes)]`
    ///
    /// # Arguments
    ///
    /// * `text` - The plaintext to encrypt (wrapped in [`PlainText`])
    /// * `out_buf` - Output buffer to write the token to (must be larger than plaintext + 48 bytes)
    ///
    /// # Returns
    ///
    /// * `Ok(Token)` - The encrypted token
    /// * `Err(RnsError::InvalidArgument)` - If the output buffer is too small
    ///
    /// # Example
    ///
    /// ```
    /// use reticulum::crypt::fernet::{Fernet, PlainText};
    /// use rand_core::OsRng;
    ///
    /// let fernet = Fernet::new_rand(OsRng);
    /// let mut buf = [0u8; 256];
    ///
    /// let token = fernet.encrypt("sensitive data".into(), &mut buf).unwrap();
    /// assert!(token.len() > 48);  // overhead is 48 bytes
    /// ```
    pub fn encrypt<'a>(
        &mut self,
        text: PlainText,
        out_buf: &'a mut [u8],
    ) -> Result<Token<'a>, RnsError> {
        if out_buf.len() <= FERNET_OVERHEAD_SIZE {
            return Err(RnsError::InvalidArgument);
        }

        let mut out_len = 0;

        // Generate random IV
        let iv = AesCbcEnc::generate_iv(&mut self.rng);
        out_buf[..iv.len()].copy_from_slice(iv.as_slice());

        out_len += iv.len();

        let chiper_len = AesCbcEnc::new(&self.enc_key, &iv)
            .encrypt_padded_b2b::<Pkcs7>(text.0, &mut out_buf[out_len..])
            .unwrap()
            .len();

        out_len += chiper_len;

        let mut hmac = HmacSha256::new_from_slice(&self.sign_key)
            .map_err(|_| RnsError::InvalidArgument)?;

        hmac.update(&out_buf[..out_len]);

        let tag = hmac.finalize().into_bytes();

        out_buf[out_len..out_len + tag.len()].copy_from_slice(tag.as_slice());
        out_len += tag.len();

        Ok(Token(&out_buf[..out_len]))
    }

    /// Verifies the authenticity and integrity of a token.
    ///
    /// This method computes the HMAC of the IV and ciphertext portions of
    /// the token and compares it to the appended HMAC tag. If they match,
    /// the token is authentic and has not been tampered with.
    ///
    /// # Arguments
    ///
    /// * `token` - The token to verify (created by [`Fernet::encrypt`])
    ///
    /// # Returns
    ///
    /// * `Ok(VerifiedToken)` - If the HMAC is valid
    /// * `Err(RnsError::IncorrectSignature)` - If the HMAC is invalid (token tampered or wrong key)
    /// * `Err(RnsError::InvalidArgument)` - If the token is too short
    ///
    /// # Example
    ///
    /// ```
    /// use reticulum::crypt::fernet::{Fernet, PlainText};
    /// use rand_core::OsRng;
    ///
    /// let fernet = Fernet::new_rand(OsRng);
    /// let mut buf = [0u8; 256];
    /// let token = fernet.encrypt("data".into(), &mut buf).unwrap();
    ///
    /// // Verify the token
    /// let verified = fernet.verify(token).unwrap();
    /// ```
    pub fn verify<'a>(&self, token: Token<'a>) -> Result<VerifiedToken<'a>, RnsError> {
        let token_data = token.0;

        if token_data.len() <= FERNET_OVERHEAD_SIZE {
            return Err(RnsError::InvalidArgument);
        }

        let expected_tag = &token_data[token_data.len() - HMAC_OUT_SIZE..];

        let mut hmac = HmacSha256::new_from_slice(&self.sign_key)
            .map_err(|_| RnsError::InvalidArgument)?;

        hmac.update(&token_data[..token_data.len() - HMAC_OUT_SIZE]);

        let actual_tag = hmac.finalize().into_bytes();

        let valid = expected_tag
            .iter()
            .zip(actual_tag.as_slice())
            .map(|(x, y)| x.cmp(y))
            .find(|&ord| ord != cmp::Ordering::Equal)
            .unwrap_or(actual_tag.len().cmp(&expected_tag.len()))
            == cmp::Ordering::Equal;

        if valid {
            Ok(VerifiedToken(token_data))
        } else {
            Err(RnsError::IncorrectSignature)
        }
    }

    /// Decrypts a verified token to recover the plaintext.
    ///
    /// The token must have been verified first using [`Fernet::verify`].
    /// This method extracts the IV, decrypts the ciphertext using AES-CBC,
    /// and removes PKCS7 padding.
    ///
    /// # Arguments
    ///
    /// * `token` - A verified token (from [`Fernet::verify`])
    /// * `out_buf` - Output buffer for the decrypted plaintext (must be large enough)
    ///
    /// # Returns
    ///
    /// * `Ok(PlainText)` - The decrypted plaintext
    /// * `Err(RnsError::InvalidArgument)` - If the token is too short
    /// * `Err(RnsError::CryptoError)` - If decryption fails (corrupted data or wrong key)
    ///
    /// # Example
    ///
    /// ```
    /// use reticulum::crypt::fernet::{Fernet, PlainText};
    /// use rand_core::OsRng;
    ///
    /// let fernet = Fernet::new_rand(OsRng);
    /// let mut buf = [0u8; 256];
    /// let token = fernet.encrypt("secret message".into(), &mut buf).unwrap();
    /// let verified = fernet.verify(token).unwrap();
    ///
    /// let mut out = [0u8; 256];
    /// let plaintext = fernet.decrypt(verified, &mut out).unwrap();
    /// assert_eq!(plaintext.as_slice(), b"secret message");
    /// ```
    pub fn decrypt<'a, 'b>(
        &self,
        token: VerifiedToken<'a>,
        out_buf: &'b mut [u8],
    ) -> Result<PlainText<'b>, RnsError> {
        let token_data = token.0;

        if token_data.len() <= FERNET_OVERHEAD_SIZE {
            return Err(RnsError::InvalidArgument);
        }

        let tag_start_index = token_data.len() - HMAC_OUT_SIZE;

        let iv: [u8; IV_KEY_SIZE] = token_data[..IV_KEY_SIZE].try_into().unwrap();

        let ciphertext = &token_data[IV_KEY_SIZE..tag_start_index];

        let msg = AesCbcDec::new(&self.enc_key, &iv.into())
            .decrypt_padded_b2b::<Pkcs7>(ciphertext, out_buf)
            .map_err(|_| RnsError::CryptoError)?;

        Ok(PlainText(msg))
    }
}


// #[cfg(test)]
// mod tests {
//     use crate::crypt::fernet::Fernet;
//     use core::str;
//     use rand_core::CryptoRng;

//     #[test]
//     fn encrypt_then_decrypt() {
//         const BUF_SIZE: usize = 4096;

//         let fernet = Fernet::new_rand(CryptoRng::rng);

//         let out_msg: &str = "#FERNET_TEST_MESSAGE#";

//         let mut out_buf = [0u8; BUF_SIZE];

//         let token = fernet
//             .encrypt(out_msg.into(), &mut out_buf[..])
//             .expect("cipher token");

//         let token = fernet.verify(token).expect("verified token");

//         let mut in_buf = [0u8; BUF_SIZE];
//         let in_msg = str::from_utf8(fernet.decrypt(token, &mut in_buf).expect("decoded token").0)
//             .expect("valid string");

//         assert_eq!(in_msg, out_msg);
//     }

    // #[test]
    // fn small_buffer() {
    //     let fernet = Fernet::new_rand(OsRng);

    //     let test_msg: &str = "#FERNET_TEST_MESSAGE#";

    //     let mut out_buf = [0u8; 12];
    //     assert!(fernet.encrypt(test_msg.into(), &mut out_buf[..]).is_err());
    // }
// }