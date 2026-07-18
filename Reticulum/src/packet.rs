//! Packet types for Reticulum networking.
//!
//! This module provides the packet types used in Reticulum for network communication,
//! including headers, packet types, and the main Packet struct.
//!
//! # Overview
//!
//! Packets are the fundamental unit of communication in Reticulum. Each packet
//! contains a header with metadata and a data payload. The header encodes:
//!
//! - Interface access flag (authenticated or open)
//! - Header type (1 or 2)
//! - Propagation type (broadcast or transport)
//! - Destination type (single, group, plain, or link)
//! - Packet type (data, announce, link request, or proof)
//! - Hop count
//!
//! # Packet Format
//!
//! A Reticulum packet consists of:
//! 1. Header (1 byte + destination address)
//! 2. Optional IFAC (Interface Access Code)
//! 3. Destination address (16 bytes)
//! 4. Optional transport address (16 bytes)
//! 5. Data payload
//!
//! # Usage
//!
//! ```
//! use reticulum::packet::{Header, Packet, PacketType, DestinationType, PropagationType};
//! use reticulum::buffer::StaticBuffer;
//!
//! // Create a simple packet
//! let header = Header {
//!     ifac_flag: reticulum::packet::IfacFlag::Open,
//!     header_type: reticulum::packet::HeaderType::Type1,
//!     propagation_type: PropagationType::Broadcast,
//!     destination_type: DestinationType::Single,
//!     packet_type: PacketType::Data,
//!     hops: 0,
//! };
//!
//! let mut data = StaticBuffer::<2048>::new();
//! data.write(b"Hello, Reticulum!").unwrap();
//!
//! let packet = Packet {
//!     header,
//!     ifac: None,
//!     destination: reticulum::hash::AddressHash::new_empty(),
//!     transport: None,
//!     context: reticulum::packet::PacketContext::None,
//!     data,
//! };
//! ```

use sha2::Digest;

use crate::buffer::StaticBuffer;
use crate::hash::AddressHash;
use crate::hash::Hash;

#[cfg(feature = "std")]
use std::fmt;

/// Maximum size of a packet data buffer (2048 bytes).
///
/// This is the maximum payload size for a single Reticulum packet.
pub const PACKET_MDU: usize = 2048;

/// Type alias for a packet data buffer (StaticBuffer of PACKET_MDU bytes).
pub type PacketDataBuffer = StaticBuffer<PACKET_MDU>;

/// A Reticulum packet.
///
/// The core data structure for all communication in Reticulum. Contains
/// a header, optional interface access code, destination, optional transport,
/// context, and data payload.
// #[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[derive(Debug)]
pub struct Packet {
    /// The packet header containing metadata.
    pub header: Header,
    /// Optional interface access code for authenticated destinations.
    pub ifac: Option<PacketIfac>,
    /// The destination address hash.
    pub destination: AddressHash,
    /// Optional transport address for multi-hop routing.
    pub transport: Option<AddressHash>,
    /// Packet context indicating special semantics.
    pub context: PacketContext,
    /// The packet data payload.
    pub data: PacketDataBuffer,
}
impl Default for Packet {
    /// Creates a default empty packet.
    fn default() -> Self {
        Self {
            header: Default::default(),
            destination: AddressHash::new_empty(),
            data: Default::default(),
            ifac: None,
            transport: None,
            context: crate::packet::PacketContext::None,
        }
    }
}
impl Packet {
    /// Computes a hash of the packet for identification.
    ///
    /// The hash is derived from the header, destination, context, and data.
    /// This is used for packet identification and deduplication.
    ///
    /// # Returns
    ///
    /// A Hash of the packet contents.
    ///
    /// # Example
    ///
    /// ```
    /// use reticulum::packet::Packet;
    ///
    /// // let packet = Packet { ... };
    /// // let hash = packet.hash();
    /// ```
    pub fn hash(&self) -> Hash {
        Hash::new(
            Hash::generator()
                .chain_update([self.header.to_meta() & 0b00001111])
                .chain_update(self.destination.as_slice())
                .chain_update([self.context as u8])
                .chain_update(self.data.as_slice())
                .finalize()
                .into(),
        )
    }
}
#[cfg(feature = "std")]
impl fmt::Display for Packet {
    /// Formats the packet as a human-readable string.
    ///
    /// Example: `[01.00 0x0001... 0x[100]] /a1b2c3d4e5f6.../ 0x[50]]`
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}", self.header)?;

        if let Some(transport) = self.transport {
            write!(f, " {}", transport)?;
        }

        write!(f, " {}", self.destination)?;

        write!(f, " 0x[{}]]", self.data.len())?;

        Ok(())
    }
}

/// The header of a Reticulum packet.
///
/// Contains all metadata about the packet including type, destination,
/// propagation, and interface access information.
// #[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[derive(Debug)]
pub struct Header {
    /// Interface access flag (open or authenticated).
    pub ifac_flag: IfacFlag,
    /// Header type (Type1 or Type2).
    pub header_type: HeaderType,
    /// Propagation type (broadcast or transport).
    pub propagation_type: PropagationType,
    /// Destination type (single, group, plain, or link).
    pub destination_type: DestinationType,
    /// The packet type (data, announce, link request, or proof).
    pub packet_type: PacketType,
    /// Number of hops the packet has traveled.
    pub hops: u8,
}
impl Default for Header {
    fn default() -> Self {
        Self {
            ifac_flag: IfacFlag::Open,
            header_type: HeaderType::Type1,
            propagation_type: PropagationType::Broadcast,
            destination_type: DestinationType::Single,
            packet_type: PacketType::Data,
            hops: 0,
        }
    }
}
impl Header {
    /// Converts the header to a metadata byte.
    /// # Returns
    ///
    /// A u8 containing the encoded header information.
    ///
    /// # Format
    ///
    /// ```ignore
    /// [IFAC (1 bit)] [Type (1 bit)] [Prop (2 bits)] [Dest (2 bits)] [Packet (2 bits)]
    /// ```
    pub fn to_meta(&self) -> u8 {
        (self.ifac_flag as u8) << 7
            | (self.header_type as u8) << 6
            | (self.propagation_type as u8) << 4
            | (self.destination_type as u8) << 2
            | (self.packet_type as u8)
    }

    /// Creates a header from a metadata byte.
    ///
    /// # Arguments
    ///
    /// * `meta` - The encoded metadata byte
    ///
    /// # Returns
    ///
    /// A new Header with the decoded fields.
    pub fn from_meta(meta: u8) -> Self {
        Self {
            ifac_flag: IfacFlag::from(meta >> 7),
            header_type: HeaderType::from(meta >> 6),
            propagation_type: PropagationType::from(meta >> 4),
            destination_type: DestinationType::from(meta >> 2),
            packet_type: PacketType::from(meta),
            hops: 0,
        }
    }
}
#[cfg(feature = "std")]
impl fmt::Display for Header {
    /// Formats the header as a human-readable string.
    ///
    /// Example output: `0110.0` (Type1, Broadcast, Single, Data, 0 hops)
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:b}{:b}{:0>2b}{:0>2b}{:0>2b}.{}",
            self.ifac_flag as u8,
            self.header_type as u8,
            self.propagation_type as u8,
            self.destination_type as u8,
            self.packet_type as u8,
            self.hops,
        )
    }
}

/// Interface access flag indicating whether a destination requires authentication.
///
/// This flag determines if the packet requires an IFAC (Interface Access Code)
/// for the destination to accept it.
// #[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[derive(Debug, Copy, Clone)]
pub enum IfacFlag {
    /// Open destination - accepts packets without authentication.
    Open = 0b0,
    /// Authenticated destination - requires valid IFAC.
    Authenticated = 0b1,
}

impl From<u8> for IfacFlag {
    fn from(value: u8) -> Self {
        match value {
            0 => IfacFlag::Open,
            1 => IfacFlag::Authenticated,
            _ => IfacFlag::Open,
        }
    }
}

/// Header type determining the packet header format.
///
/// Type 1 is the standard header, Type 2 may have additional fields.
// #[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[derive(Debug, Copy, Clone)]
pub enum HeaderType {
    /// Standard header format.
    Type1 = 0b0,
    /// Extended header format.
    Type2 = 0b1,
}

impl From<u8> for HeaderType {
    fn from(value: u8) -> Self {
        match value & 0b1 {
            0 => HeaderType::Type1,
            1 => HeaderType::Type2,
            _ => HeaderType::Type1,
        }
    }
}

/// Propagation type determining how a packet is routed through the network.
///
/// This affects whether a packet is broadcast locally or forwarded to other networks.
//#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[derive(Debug, Copy, Clone)]
pub enum PropagationType {
    /// Broadcast locally - not forwarded to other networks.
    Broadcast = 0b00,
    /// Transport - can be forwarded to other networks.
    Transport = 0b01,
    /// Reserved for future use.
    Reserved1 = 0b10,
    /// Reserved for future use.
    Reserved2 = 0b11,
}
impl From<u8> for PropagationType {
    fn from(value: u8) -> Self {
        match value & 0b11 {
            0b00 => PropagationType::Broadcast,
            0b01 => PropagationType::Transport,
            0b10 => PropagationType::Reserved1,
            0b11 => PropagationType::Reserved2,
            _ => PropagationType::Broadcast,
        }
    }
}

/// Destination type specifying the addressing mode.
///
/// This determines how the destination address is interpreted and what
/// encryption/signing rules apply.
//#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[derive(Debug, Copy, Clone)]
pub enum DestinationType {
    /// Single destination - point-to-point encrypted communication.
    Single = 0b00,
    /// Group destination - multiple recipients with shared key.
    Group = 0b01,
    /// Plain destination - no encryption.
    Plain = 0b10,
    /// Link destination - for link establishment.
    Link = 0b11,
}
impl From<u8> for DestinationType {
    fn from(value: u8) -> Self {
        match value & 0b11 {
            0b00 => DestinationType::Single,
            0b01 => DestinationType::Group,
            0b10 => DestinationType::Plain,
            0b11 => DestinationType::Link,
            _ => DestinationType::Single,
        }
    }
}

/// The type of packet, determining its purpose in the protocol.
///
/// Different packet types serve different roles in Reticulum communication.
// #[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[derive(Debug, Copy, Clone)]
pub enum PacketType {
    /// Data packet - contains application data.
    Data = 0b00,
    /// Announce packet - advertises a destination's presence.
    Announce = 0b01,
    /// Link request - initiates link establishment.
    LinkRequest = 0b10,
    /// Proof packet - provides cryptographic proof.
    Proof = 0b11,
}
impl From<u8> for PacketType {
    fn from(value: u8) -> Self {
        match value & 0b11 {
            0b00 => PacketType::Data,
            0b01 => PacketType::Announce,
            0b10 => PacketType::LinkRequest,
            0b11 => PacketType::Proof,
            _ => PacketType::Data,
        }
    }
}

/// Maximum length of the Interface Access Code (IFAC) in bytes.
///
/// The IFAC is used for authenticated access to destinations.
pub const PACKET_IFAC_MAX_LENGTH: usize = 64;

/// Interface Access Code (IFAC) for authenticated destinations.
///
/// The IFAC is an optional access code that can be required by destinations
/// to accept incoming packets, providing an additional layer of access control.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct PacketIfac {
    /// The access code bytes.
    pub access_code: [u8; PACKET_IFAC_MAX_LENGTH],
    /// The actual length of the access code in use.
    pub length: usize,
}

impl PacketIfac {
    /// Creates a new PacketIfac from a byte slice.
    ///
    /// # Arguments
    ///
    /// * `slice` - The access code bytes (max 64 bytes)
    ///
    /// # Example
    ///
    /// ```
    /// use reticulum::packet::PacketIfac;
    ///
    /// let ifac = PacketIfac::new_from_slice(b"my_access_code");
    /// ```
    pub fn new_from_slice(slice: &[u8]) -> Self {
        let mut access_code = [0u8; PACKET_IFAC_MAX_LENGTH];
        access_code[..slice.len()].copy_from_slice(slice);
        Self {
            access_code,
            length: slice.len(),
        }
    }

    /// Returns the IFAC data as a slice.
    ///
    /// # Example
    ///
    /// ```
    /// use reticulum::packet::PacketIfac;
    ///
    /// let ifac = PacketIfac::new_from_slice(b"secret");
    /// let data = ifac.as_slice();
    /// assert_eq!(data, b"secret");
    /// ```
    pub fn as_slice(&self) -> &[u8] {
        &self.access_code[..self.length]
    }
}

/// Context field providing additional packet semantics.
///
/// The context indicates what kind of data the packet contains or its role
/// in higher-level protocols like resource transfer, requests/responses, etc.
// #[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[derive(Debug, Copy, Clone)]
pub enum PacketContext {
    /// Generic data packet with no special context.
    None = 0x00,
    /// Packet is part of a resource transfer.
    Resource = 0x01,
    /// Packet is a resource advertisement.
    ResourceAdvertisement = 0x02,
    /// Packet is a request for resource parts.
    ResourceRequest = 0x03,
    /// Packet is a resource hash map update.
    ResourceHashUpdate = 0x04,
    /// Packet is a resource proof.
    ResourceProof = 0x05,
    /// Packet is a resource initiator cancel message.
    ResourceInitiatorCancel = 0x06,
    /// Packet is a resource receiver cancel message.
    ResourceReceiverCancel = 0x07,
    /// Packet is a cache request.
    CacheRequest = 0x08,
    /// Packet is a request (request-response pattern).
    Request = 0x09,
    /// Packet is a response to a request.
    Response = 0x0A,
    /// Packet is a response to a path request.
    PathResponse = 0x0B,
    /// Packet is a command.
    Command = 0x0C,
    /// Packet is a command status response.
    CommandStatus = 0x0D,
    /// Packet contains link channel data.
    Channel = 0x0E,
    /// Packet is a keepalive.
    KeepAlive = 0xFA,
    /// Packet is a link peer identification proof.
    LinkIdentify = 0xFB,
    /// Packet is a link close message.
    LinkClose = 0xFC,
    /// Packet is a link packet proof.
    LinkProof = 0xFD,
    /// Packet is a link RTT measurement.
    LinkRTT = 0xFE,
    /// Packet is a link request proof.
    LinkRequestProof = 0xFF,
}
impl From<u8> for PacketContext {
    fn from(value: u8) -> Self {
        match value {
            0x01 => PacketContext::Resource,
            0x02 => PacketContext::ResourceAdvertisement,
            0x03 => PacketContext::ResourceRequest,
            0x04 => PacketContext::ResourceHashUpdate,
            0x05 => PacketContext::ResourceProof,
            0x06 => PacketContext::ResourceInitiatorCancel,
            0x07 => PacketContext::ResourceReceiverCancel,
            0x08 => PacketContext::CacheRequest,
            0x09 => PacketContext::Request,
            0x0A => PacketContext::Response,
            0x0B => PacketContext::PathResponse,
            0x0C => PacketContext::Command,
            0x0D => PacketContext::CommandStatus,
            0x0E => PacketContext::Channel,
            0xFA => PacketContext::KeepAlive,
            0xFB => PacketContext::LinkIdentify,
            0xFC => PacketContext::LinkClose,
            0xFD => PacketContext::LinkProof,
            0xFE => PacketContext::LinkRTT,
            0xFF => PacketContext::LinkRequestProof,
            _ => PacketContext::None,
        }
    }
}
