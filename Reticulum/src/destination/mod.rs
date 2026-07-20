//! Destination types for Reticulum networking.
//!
//! This module provides types for representing endpoints in a Reticulum Network.
//! Destinations are used as targets for sending packets, establishing links,
//! and receiving data in Reticulum.
//!
//! # Overview
//!
//! A [`Destination`] represents an endpoint in the Reticulum network. It combines:
//! - An [`Identity`][crate::identity::Identity] for cryptographic operations
//! - A [`DestinationName`] for human-readable addressing
//! - Direction (input/output) and Type (single/group/plain) for configuration
//!
//! # Destination Types
//!
//! - **Single**: Point-to-point encrypted destination
//! - **Group**: Group-based encrypted destination
//! - **Plain**: Unencrypted destination
//!
//! # Directions
//!
//! - **Input**: Receiving end (can decrypt incoming data)
//! - **Output**: Sending end (can encrypt data for destination)
//!
//! # Usage
//!
//! Destinations are typically created via the [`Transport`][crate::transport::Transport]
//! and announced to the network using [`announce()`][SingleInputDestination::announce].

// pub mod link;
// pub mod link_map;

use core::{fmt, marker::PhantomData};

use sha2::Digest;
use rand_core::CryptoRng;
use ed25519_dalek::{Signature, SigningKey, VerifyingKey, SIGNATURE_LENGTH};
use x25519_dalek::PublicKey;


use crate::{
    error::RnsError,
    hash::{AddressHash, Hash},
    identity::{EmptyIdentity, HashIdentity, Identity, PrivateIdentity, PUBLIC_KEY_LENGTH},
    packet::{
        self, DestinationType, Header, HeaderType, IfacFlag, Packet, PacketContext,
        PacketDataBuffer, PacketType, PropagationType,
    },
};

/// A destination endpoint in the Reticulum network.
///
/// Destinations are used as targets for sending packets, establishing links,
/// and receiving data. The type parameters encode:
///
/// - `I`: The identity type ( [`PrivateIdentity`] for input, [`Identity`] for output)
/// - `D`: The direction ([`Input`] or [`Output`])
/// - `T`: The destination type ([`Single`], [`Group`], or [`Plain`])
///
/// # Type Aliases
///
/// For common combinations, use the type aliases:
/// - [`SingleInputDestination`] - Single destination for receiving
/// - [`SingleOutputDestination`] - Single destination for sending
/// - [`PlainInputDestination`] - Plain destination for receiving
/// - [`PlainOutputDestination`] - Plain destination for sending
///
/// # Usage
///
/// ```ignore
/// use reticulum::destination::{DestinationName, SingleInputDestination};
/// use reticulum::identity::PrivateIdentity;
/// use rand_core::OsRng;
///
/// let identity = PrivateIdentity::new_from_rand(OsRng);
/// let destination = SingleInputDestination::new(identity, DestinationName::new("app", "service"));
/// ```
pub struct Destination<I: HashIdentity, D: Direction, T: Type> {
    /// The direction (input or output).
    pub direction: PhantomData<D>,
    /// The destination type (single, group, or plain).
    pub r#type: PhantomData<T>,
    /// The identity used for cryptographic operations.
    pub identity: I,
    /// The destination descriptor containing addressing info.
    pub desc: DestinationDesc,
}
impl Destination<PrivateIdentity, Input, Single> {
    /// Creates a new single-input destination.
    ///
    /// A single-input destination can receive encrypted messages and must
    /// be announced to the network for peers to discover it.
    ///
    /// # Arguments
    ///
    /// * `identity` - The private identity for this destination
    /// * `name` - The destination name
    ///
    /// # Example
    ///
    /// ```ignore
    /// use reticulum::destination::{DestinationName, SingleInputDestination};
    /// use reticulum::identity::PrivateIdentity;
    /// use rand_core::OsRng;
    ///
    /// let identity = PrivateIdentity::new_from_rand(OsRng);
    /// let destination = SingleInputDestination::new(
    ///     identity,
    ///     DestinationName::new("myapp", "chat")
    /// );
    /// ```
    pub fn new(identity: PrivateIdentity, name: DestinationName) -> Self {
        let address_hash = create_address_hash(&identity, &name);
        let pub_identity = *identity.as_identity();

        Self {
            direction: PhantomData,
            r#type: PhantomData,
            identity,
            desc: DestinationDesc {
                identity: pub_identity,
                name,
                address_hash,
            },
        }
    }

    /// Creates and returns an announce packet for this destination.
    ///
    /// The announce packet broadcasts the destination's presence to the
    /// network and includes all information peers need to establish
    /// encrypted communication.
    ///
    /// # Arguments
    ///
    /// * `rng` - Random number generator
    /// * `app_data` - Optional application-specific data to include
    ///
    /// # Returns
    ///
    /// * `Ok(Packet)` - The announce packet ready to send
    /// * `Err(RnsError)` - On error
    ///
    /// # Example
    ///
    /// ```ignore
    /// use reticulum::destination::{DestinationName, SingleInputDestination};
    /// use reticulum::identity::PrivateIdentity;
    /// use rand_core::OsRng;
    ///
    /// let identity = PrivateIdentity::new_from_rand(OsRng);
    /// let destination = SingleInputDestination::new(
    ///     identity,
    ///     DestinationName::new("app", "service")
    /// );
    ///
    /// let packet = destination.announce(OsRng, None).unwrap();
    /// // Send packet to network...
    /// ```
    pub fn announce<R: CryptoRng + Copy>(
        &self,
        rng: R,
        app_data: Option<&[u8]>,
    ) -> Result<Packet, RnsError> {
        let mut packet_data = PacketDataBuffer::new();

        // FIXME only write the data once - rather than reset as below

        let random_hash = Hash::new_from_rand(rng);
        let timestamp = announce_timestamp_bytes();
        let rand_hash = [
            &random_hash.as_slice()[..RAND_HASH_LENGTH / 2],
            &timestamp[3..],
        ]
        .concat();

        let pub_key = self.identity.as_identity().public_key_bytes();
        let verifying_key = self.identity.as_identity().verifying_key_bytes();

        packet_data
            .chain_safe_write(self.desc.address_hash.as_slice())
            .chain_safe_write(pub_key)
            .chain_safe_write(verifying_key)
            .chain_safe_write(self.desc.name.as_name_hash_slice())
            .chain_safe_write(&rand_hash);

        if let Some(data) = app_data {
            packet_data.write(data)?;
        }

        let signature = self.identity.sign(packet_data.as_slice());

        packet_data.reset();

        packet_data
            .chain_safe_write(pub_key)
            .chain_safe_write(verifying_key)
            .chain_safe_write(self.desc.name.as_name_hash_slice())
            .chain_safe_write(&rand_hash)
            .chain_safe_write(&signature.to_bytes());

        if let Some(data) = app_data {
            packet_data.write(data)?;
        }

        Ok(Packet {
            header: Header {
                ifac_flag: IfacFlag::Open,
                header_type: HeaderType::Type1,
                propagation_type: PropagationType::Broadcast,
                destination_type: DestinationType::Single,
                packet_type: PacketType::Announce,
                hops: 0,
            },
            ifac: None,
            destination: self.desc.address_hash,
            transport: None,
            context: PacketContext::None,
            data: packet_data,
        })
    }

    /// Creates a path response packet.
    ///
    /// Path responses are used in path discovery and allow the destination
    /// to respond to path requests from peers.
    ///
    /// # Arguments
    ///
    /// * `rng` - Random number generator
    /// * `app_data` - Optional application data
    pub fn path_response<R: CryptoRng + Copy>(
        &self,
        rng: R,
        app_data: Option<&[u8]>,
    ) -> Result<Packet, RnsError> {
        let mut announce = self.announce(rng, app_data)?;
        announce.context = PacketContext::PathResponse;

        Ok(announce)
    }

    /// Handles an incoming packet for this destination.
    ///
    /// Examines the packet to determine what action should be taken.
    ///
    /// # Arguments
    ///
    /// * `packet` - The incoming packet
    ///
    /// # Returns
    ///
    /// The [`DestinationHandleStatus`] indicating what action to take.
    pub fn handle_packet(&mut self, packet: &Packet) -> DestinationHandleStatus {
        if self.desc.address_hash != packet.destination {
            return DestinationHandleStatus::None;
        }

        if packet.header.packet_type == PacketType::LinkRequest {
            // TODO: check prove strategy
            return DestinationHandleStatus::LinkProof;
        }

        DestinationHandleStatus::None
    }

    /// Returns a reference to the signing key for this destination.
    ///
    /// Used for signing outbound messages and announcements.
    pub fn sign_key(&self) -> &SigningKey {
        self.identity.sign_key()
    }
}
impl Destination<Identity, Output, Single> {
    /// Creates a new single-output destination.
    ///
    /// An output destination is used to address packets to a peer.
    /// It holds only the public identity.
    ///
    /// # Arguments
    ///
    /// * `identity` - The public identity of the destination
    /// * `name` - The destination name
    pub fn new(identity: Identity, name: DestinationName) -> Self {
        let address_hash = create_address_hash(&identity, &name);
        Self {
            direction: PhantomData,
            r#type: PhantomData,
            identity,
            desc: DestinationDesc {
                identity,
                name,
                address_hash,
            },
        }
    }
}

impl<D: Direction> Destination<EmptyIdentity, D, Plain> {
    /// Creates a new plain destination (input or output).
    ///
    /// Plain destinations transmit data without encryption. The
    /// direction determines whether it can send or receive.
    ///
    /// # Arguments
    ///
    /// * `identity` - Empty identity for plain destinations
    /// * `name` - The destination name
    pub fn new(identity: EmptyIdentity, name: DestinationName) -> Self {
        let address_hash = create_address_hash(&identity, &name);
        Self {
            direction: PhantomData,
            r#type: PhantomData,
            identity,
            desc: DestinationDesc {
                identity: Default::default(),
                name,
                address_hash,
            },
        }
    }
}


/// Creates an address hash from an identity and destination name.
///
/// The address hash is used for routing packets in Reticulum.
/// It is derived by hashing the name hash together with the
/// identity's address hash.
fn create_address_hash<I: HashIdentity>(identity: &I, name: &DestinationName) -> AddressHash {
    AddressHash::new_from_hash(&Hash::new(
        Hash::generator()
            .chain_update(name.as_name_hash_slice())
            .chain_update(identity.as_address_hash_slice())
            .finalize()
            .into(),
    ))
}

fn announce_timestamp_bytes() -> [u8; 8] {
    #[cfg(feature = "std")]
    {
        std::time::UNIX_EPOCH
            .elapsed()
            .unwrap()
            .as_secs()
            .to_be_bytes()
    }
    #[cfg(not(feature = "std"))]
    {
        [0u8; 8]
    }
}


/// Status returned when handling a packet for a destination.
///
/// Indicates what action should be taken after processing an incoming packet.
pub enum DestinationHandleStatus {
    /// No action needed for this packet.
    None,
    /// A link proof is required.
    LinkProof,
}







impl<I: HashIdentity, D: Direction, T: Type> Destination<I, D, T> {
    /// Returns the packet destination type.
    ///
    /// This corresponds to the [`packet::DestinationType`] used in
    /// Reticulum protocol headers.
    pub fn destination_type(&self) -> packet::DestinationType {
        <T as Type>::destination_type()
    }
}
// FIXME
// impl<I: DecryptIdentity + HashIdentity, T: Type> Destination<I, Input, T> {
//     pub fn decrypt<'b, R: CryptoRng + Copy>(
//         &self,
//         rng: R,
//         data: &[u8],
//         out_buf: &'b mut [u8],
//     ) -> Result<&'b [u8], RnsError> {
//         self.identity.decrypt(rng, data, out_buf)
//     }
// }
// FIXME
// impl<I: EncryptIdentity + HashIdentity, D: Direction, T: Type> Destination<I, D, T> {
//     pub fn encrypt<'b, R: CryptoRngCore + Copy>(
//         &self,
//         rng: R,
//         text: &[u8],
//         out_buf: &'b mut [u8],
//     ) -> Result<&'b [u8], RnsError> {
//         // self.identity.encrypt(
//         //     rng,
//         //     text,
//         //     Some(self.identity.as_address_hash_slice()),
//         //     out_buf,
//         // )
//     }
// }

/// A destination descriptor containing identity and addressing information.
///
/// This is the core data structure that identifies a specific destination
/// in the Reticulum network, combining cryptographic identity with
/// human-readable naming.
#[derive(Copy, Clone)]
pub struct DestinationDesc {
    /// The cryptographic identity of this destination.
    pub identity: Identity,
    /// The address hash used in packet routing.
    pub address_hash: AddressHash,
    /// The destination name (app_name.aspects).
    pub name: DestinationName,
}
#[cfg(feature = "std")]
impl fmt::Display for DestinationDesc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.address_hash)?;

        Ok(())
    }
}

/// Length of the destination name hash in bytes (10 bytes).
///
/// The name hash is derived from the app_name and aspects, truncated
/// to this length for inclusion in announce packets.
pub const NAME_HASH_LENGTH: usize = 10;

/// A destination name derived from app_name and aspects.
///
/// Destination names are hashed to create a compact identifier used
/// in packet addressing. The full name is formed as `{app_name}.{aspects}`.
#[derive(Copy, Clone)]
pub struct DestinationName {
    /// The hash of the destination name.
    pub hash: Hash,
}
impl DestinationName {
    /// Creates a new DestinationName from an app name and aspects.
    ///
    /// The full name is formed by concatenating `app_name`, a dot, and `aspects`,
    /// then hashing the result.
    ///
    /// # Arguments
    ///
    /// * `app_name` - The application name (e.g., "myapp")
    /// * `aspects` - The aspects/path (e.g., "chat", "status")
    ///
    /// # Example
    ///
    /// ```
    /// use reticulum::destination::DestinationName;
    ///
    /// let name = DestinationName::new("messenger", "chat");
    /// let hash = name.as_name_hash_slice();
    /// assert_eq!(hash.len(), 10);
    /// ```
    pub fn new(app_name: &str, aspects: &str) -> Self {
        let hash = Hash::new(
            Hash::generator()
                .chain_update(app_name.as_bytes())
                .chain_update(".".as_bytes())
                .chain_update(aspects.as_bytes())
                .finalize()
                .into(),
        );

        Self { hash }
    }

    /// Creates a DestinationName from an existing name hash slice.
    ///
    /// This is useful when reconstructing a destination name from
    /// announce packet data.
    ///
    /// # Arguments
    ///
    /// * `hash_slice` - A slice containing the name hash bytes
    pub fn new_from_hash_slice(hash_slice: &[u8]) -> Self {
        let mut hash = [0u8; 32];
        hash[..hash_slice.len()].copy_from_slice(hash_slice);

        Self {
            hash: Hash::new(hash),
        }
    }

    /// Returns the name hash slice (first NAME_HASH_LENGTH bytes).
    ///
    /// This is the portion of the hash used in packet addressing.
    ///
    /// # Example
    ///
    /// ```
    /// use reticulum::destination::DestinationName;
    ///
    /// let name = DestinationName::new("app", "service");
    /// let hash_slice = name.as_name_hash_slice();
    /// assert_eq!(hash_slice.len(), 10);
    /// ```
    pub fn as_name_hash_slice(&self) -> &[u8] {
        &self.hash.as_slice()[..NAME_HASH_LENGTH]
    }
}

/// A single-input destination (receiving end, encrypted).
///
/// This is the most common type for servers or applications that
/// need to receive encrypted messages from clients.
pub type SingleInputDestination = Destination<PrivateIdentity, Input, Single>;

/// A single-output destination (sending end, encrypted).
///
/// Used when you want to send encrypted messages to a specific
/// destination that you know the identity of.
pub type SingleOutputDestination = Destination<Identity, Output, Single>;

/// A plain-input destination (receiving end, unencrypted).
pub type PlainInputDestination = Destination<EmptyIdentity, Input, Plain>;

/// A plain-output destination (sending end, unencrypted).
pub type PlainOutputDestination = Destination<EmptyIdentity, Output, Plain>;

/// Length of the random hash in announce packets (10 bytes).
///
/// A random value is included in announce packets to ensure uniqueness
/// and prevent certain cryptanalytic attacks.
pub const RAND_HASH_LENGTH: usize = 10;

/// Minimum data length for a valid announce packet.
///
/// This is calculated as: public key + verifying key + name hash +
/// random hash + signature = 32*2 + 10 + 10 + 64 = 148 bytes
pub const MIN_ANNOUNCE_DATA_LENGTH: usize =
    PUBLIC_KEY_LENGTH * 2 + NAME_HASH_LENGTH + RAND_HASH_LENGTH + SIGNATURE_LENGTH;

/// A packet containing a destination announcement.
///
/// Announce packets are broadcast to the network to notify peers of
/// a destination's presence and provide necessary keys for encrypted
/// communication.
pub type DestinationAnnounce = Packet;
impl DestinationAnnounce {
    /// Validates an announce packet and extracts the destination information.
    ///
    /// This method verifies the signature on the announce packet and
    /// extracts the announced destination's identity and application data.
    ///
    /// # Arguments
    ///
    /// * `packet` - The packet to validate
    ///
    /// # Returns
    ///
    /// * `Ok((SingleOutputDestination, app_data))` - On success, returns the
    ///   destination and any application-specific data
    /// * `Err(RnsError::PacketError)` - If the packet is not an announce
    /// * `Err(RnsError::OutOfMemory)` - If packet data is too short
    /// * `Err(RnsError::CryptoError)` - If signature verification fails
    ///
    /// # Example
    ///
    /// ```
    /// use reticulum::packet::{Packet, PacketType};
    /// use reticulum::destination::DestinationAnnounce;
    /// fn example(packet: &Packet) {
    /// if packet.header.packet_type == PacketType::Announce {
    ///     let result = DestinationAnnounce::validate(packet);
    ///     if let Ok((destination, app_data)) = result {
    ///         // Use the announced destination
    ///     }
    /// }
    /// # }
    /// ```
    pub fn validate(packet: &Packet) -> Result<(SingleOutputDestination, &[u8]), RnsError> {
        if packet.header.packet_type != PacketType::Announce {
            return Err(RnsError::PacketError);
        }

        let announce_data = packet.data.as_slice();

        if announce_data.len() < MIN_ANNOUNCE_DATA_LENGTH {
            return Err(RnsError::OutOfMemory);
        }

        let mut offset = 0usize;

        let public_key = {
            let mut key_data = [0u8; PUBLIC_KEY_LENGTH];
            key_data.copy_from_slice(&announce_data[offset..(offset + PUBLIC_KEY_LENGTH)]);
            offset += PUBLIC_KEY_LENGTH;
            PublicKey::from(key_data)
        };

        let verifying_key = {
            let mut key_data = [0u8; PUBLIC_KEY_LENGTH];
            key_data.copy_from_slice(&announce_data[offset..(offset + PUBLIC_KEY_LENGTH)]);
            offset += PUBLIC_KEY_LENGTH;

            VerifyingKey::from_bytes(&key_data).map_err(|_| RnsError::CryptoError)?
        };

        let identity = Identity::new(public_key, verifying_key);

        let name_hash = &announce_data[offset..(offset + NAME_HASH_LENGTH)];
        offset += NAME_HASH_LENGTH;
        let rand_hash = &announce_data[offset..(offset + RAND_HASH_LENGTH)];
        offset += RAND_HASH_LENGTH;
        let signature = &announce_data[offset..(offset + SIGNATURE_LENGTH)];
        offset += SIGNATURE_LENGTH;

        let app_data = &announce_data[offset..];

        let destination = &packet.destination;

        // Keeping signed data on stack is only option for now.
        // Verification function doesn't support prehashed message.
        let signed_data = PacketDataBuffer::new()
            .chain_write(destination.as_slice())?
            .chain_write(public_key.as_bytes())?
            .chain_write(verifying_key.as_bytes())?
            .chain_write(name_hash)?
            .chain_write(rand_hash)?
            .chain_write(app_data)?
            .finalize();

        let signature = Signature::from_slice(signature).map_err(|_| RnsError::CryptoError)?;

        identity.verify(signed_data.as_slice(), &signature)?;

        Ok((
            SingleOutputDestination::new(identity, DestinationName::new_from_hash_slice(name_hash)),
            app_data,
        ))
    }
}




// Traits
//***************************************************************************

/// Trait for distinguishing input and output directions.
///
/// This is a marker trait used as a type-level indicator of whether
/// a destination is for receiving ([`Input`]) or sending ([`Output`]).
pub trait Direction {}

/// Input direction - receiving end of communication.
///
/// An Input destination can receive and decrypt incoming packets.
/// It holds a [`PrivateIdentity`][crate::identity::PrivateIdentity] with
/// both encryption and signing keys.
pub struct Input;
impl Direction for Input {}

/// Output direction - sending end of communication.
///
/// An Output destination is used to send encrypted packets to a peer.
/// It holds only a public [`Identity`][crate::identity::Identity].
pub struct Output;
impl Direction for Output {}

/// Trait for destination types.
///
/// Defines the packet [`DestinationType`][packet::DestinationType] used
/// by a destination in the Reticulum protocol.
pub trait Type {
    /// Returns the packet destination type for this destination type.
    fn destination_type() -> DestinationType;
}

/// Single destination type.
///
/// A Single destination provides point-to-point encrypted communication.
/// Each single destination has a unique identity and only the holder of
/// the corresponding private key can decrypt messages sent to it.
///
/// # Usage
///
/// Single destinations are used for direct, encrypted communication
/// between two parties.
pub struct Single;
impl Type for Single {
    fn destination_type() -> DestinationType {
        DestinationType::Single
    }
}

/// Plain (unencrypted) destination type.
///
/// A Plain destination does not provide encryption. Messages sent to
/// or from a plain destination are transmitted in plaintext.
///
/// # Usage
///
/// Plain destinations are useful for broadcast applications, testing,
/// or cases where encryption is handled at a different layer.
pub struct Plain;
impl Type for Plain {
    fn destination_type() -> DestinationType {
        DestinationType::Plain
    }
}

/// Group destination type.
///
/// A Group destination allows multiple recipients to decrypt messages
/// using a shared group key. Any member of the group can decrypt
/// messages sent to the group destination.
///
/// # Usage
///
/// Group destinations are useful for group communication where multiple
/// parties need to receive the same encrypted messages.
pub struct Group;
impl Type for Group {
    fn destination_type() -> DestinationType {
        DestinationType::Group
    }
}

//==============================================================================
#[cfg(test)]
mod tests {
    // use rand_core::OsRng;
    use serde::Serialize;

    use super::*;

    use crate::buffer::OutputBuffer;
    use crate::hash::Hash;
    use crate::identity::PrivateIdentity;

    // FIXME
    // #[test]
    // fn create_announce() {
    //     let identity = PrivateIdentity::new_from_rand(OsRng);

    //     let single_in_destination =
    //         SingleInputDestination::new(identity, DestinationName::new("test", "in"));

    //     let announce_packet = single_in_destination
    //         .announce(OsRng, None)
    //         .expect("valid announce packet");

    //     println!("Announce packet {}", announce_packet);
    // }

    #[test]
    fn create_path_request_hash() {
        let name = DestinationName::new("rnstransport", "path.request");

        println!("PathRequest Name Hash {}", name.hash);
        println!(
            "PathRequest Destination Hash {}",
            Hash::new_from_slice(name.as_name_hash_slice())
        );
    }

    // FIXME
    // #[test]
    // fn compare_announce() {
    //     let priv_key: [u8; 32] = [
    //         0xf0, 0xec, 0xbb, 0xa4, 0x9e, 0x78, 0x3d, 0xee, 0x14, 0xff, 0xc6, 0xc9, 0xf1, 0xe1,
    //         0x25, 0x1e, 0xfa, 0x7d, 0x76, 0x29, 0xe0, 0xfa, 0x32, 0x41, 0x3c, 0x5c, 0x59, 0xec,
    //         0x2e, 0x0f, 0x6d, 0x6c,
    //     ];

    //     let sign_priv_key: [u8; 32] = [
    //         0xf0, 0xec, 0xbb, 0xa4, 0x9e, 0x78, 0x3d, 0xee, 0x14, 0xff, 0xc6, 0xc9, 0xf1, 0xe1,
    //         0x25, 0x1e, 0xfa, 0x7d, 0x76, 0x29, 0xe0, 0xfa, 0x32, 0x41, 0x3c, 0x5c, 0x59, 0xec,
    //         0x2e, 0x0f, 0x6d, 0x6c,
    //     ];

    //     let priv_identity = PrivateIdentity::new(priv_key.into(), sign_priv_key.into());

    //     println!("identity hash {}", priv_identity.as_identity().address_hash);

    //     let destination = SingleInputDestination::new(
    //         priv_identity,
    //         DestinationName::new("example_utilities", "announcesample.fruits"),
    //     );

    //     println!("destination name hash {}", destination.desc.name.hash);
    //     println!("destination hash {}", destination.desc.address_hash);

    //     let announce = destination
    //         .announce(OsRng, None)
    //         .expect("valid announce packet");

    //     let mut output_data = [0u8; 4096];
    //     let mut buffer = OutputBuffer::new(&mut output_data);

    //     let _ = announce.serialize(&mut buffer).expect("correct data");

    //     println!("ANNOUNCE {}", buffer);
    // }

    // FIXME
    // #[test]
    // fn check_announce() {
    //     let priv_identity = PrivateIdentity::new_from_rand(OsRng);

    //     let destination = SingleInputDestination::new(
    //         priv_identity,
    //         DestinationName::new("example_utilities", "announcesample.fruits"),
    //     );

    //     let announce = destination
    //         .announce(OsRng, None)
    //         .expect("valid announce packet");

    //     DestinationAnnounce::validate(&announce).expect("valid announce");
    // }
}