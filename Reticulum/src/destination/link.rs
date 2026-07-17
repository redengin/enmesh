/// Link management for Reticulum connections.
///
/// This module provides the Link implementation for managing persistent
/// bidirectional connections between Reticulum peers. Links provide:
///
/// - Reliable encrypted communication
/// - Keep-alive and latency tracking
/// - Message proofs for verification
/// - Automatic reconnection handling
///
/// # Overview
///
/// A Link is a persistent connection between two Reticulum destinations.
/// Unlike simple packet exchange, links maintain state including:
/// - Shared encryption keys derived via DH
/// - Round-trip time (RTT) tracking
/// - Keep-alive for connection monitoring
/// - Message proofs for data integrity
///
/// # Link Lifecycle
///
/// 1. **Pending**: Link request sent, waiting for proof
/// 2. **Handshake**: Proof received, key exchange in progress
/// 3. **Active**: Link established, can send/receive data
/// 4. **Stale**: Link hasn't received data recently
/// 5. **Closed**: Link terminated
///
/// # Usage
///
/// Links are typically created through the Transport:
/// ```ignore
/// let link = transport.link(destination).await;
/// ```
use std::{
    cmp::min,
    time::{Duration, Instant},
};

use ed25519_dalek::{Signature, SigningKey, Verifier, PUBLIC_KEY_LENGTH, SIGNATURE_LENGTH};
use rand_core::OsRng;
use sha2::Digest;
use x25519_dalek::StaticSecret;

use crate::{
    buffer::OutputBuffer,
    error::RnsError,
    hash::{AddressHash, Hash, ADDRESS_HASH_SIZE, HASH_SIZE},
    identity::{DecryptIdentity, DerivedKey, EncryptIdentity, Identity, PrivateIdentity},
    packet::{
        DestinationType, Header, Packet, PacketContext, PacketDataBuffer, PacketType, PACKET_MDU,
    },
    transport::engine::{classify_link_data, LinkDataAction},
};

use super::DestinationDesc;

/// Size of the link MTU extension in bytes.
const LINK_MTU_SIZE: usize = 3;

/// The status of a link connection.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum LinkStatus {
    /// Link request has been sent, waiting for proof.
    Pending = 0x00,
    /// Proof received, performing key exchange.
    Handshake = 0x01,
    /// Link is active and can exchange data.
    Active = 0x02,
    /// Link is stale, no recent activity.
    Stale = 0x03,
    /// Link has been closed.
    Closed = 0x04,
}

impl LinkStatus {
    /// Checks if the link is not yet active.
    ///
    /// Returns true if the link is in Pending or Handshake state.
    pub fn not_yet_active(&self) -> bool {
        *self == LinkStatus::Pending || *self == LinkStatus::Handshake
    }
}

/// A type alias for link identifiers (uses AddressHash).
pub type LinkId = AddressHash;

/// The payload data for a link message.
#[derive(Clone)]
pub struct LinkPayload {
    /// Internal buffer storing the payload data.
    buffer: [u8; PACKET_MDU],
    /// Actual length of data in the buffer.
    len: usize,
}

impl LinkPayload {
    /// Creates a new empty link payload.
    pub fn new() -> Self {
        Self {
            buffer: [0u8; PACKET_MDU],
            len: 0,
        }
    }

    /// Creates a link payload from a byte slice.
    ///
    /// # Arguments
    ///
    /// * `data` - The data to store in the payload
    pub fn new_from_slice(data: &[u8]) -> Self {
        let mut buffer = [0u8; PACKET_MDU];

        let len = min(data.len(), buffer.len());

        buffer[..len].copy_from_slice(&data[..len]);

        Self { buffer, len }
    }

    /// Creates a link payload from bytes (alias for [`Self::new_from_slice`]).
    ///
    /// # Arguments
    ///
    /// * `data` - Source bytes
    pub fn new_from_vec(data: &[u8]) -> Self {
        Self::new_from_slice(data)
    }

    /// Returns the length of the payload data.
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns the payload data as a slice.
    pub fn as_slice(&self) -> &[u8] {
        &self.buffer[..self.len]
    }
}

impl Default for LinkPayload {
    fn default() -> Self {
        Self::new()
    }
}

impl From<&Packet> for LinkId {
    /// Creates a LinkId from a link request packet.
    ///
    /// The link ID is derived from the packet's header, destination,
    /// context, and public keys.
    fn from(packet: &Packet) -> Self {
        let data = packet.data.as_slice();
        let data_diff = if data.len() > PUBLIC_KEY_LENGTH * 2 {
            data.len() - PUBLIC_KEY_LENGTH * 2
        } else {
            0
        };

        let hashable_data = &data[..data.len() - data_diff];

        AddressHash::new_from_hash(&Hash::new(
            Hash::generator()
                .chain_update([packet.header.to_meta() & 0b00001111])
                .chain_update(packet.destination.as_slice())
                .chain_update([packet.context as u8])
                .chain_update(hashable_data)
                .finalize()
                .into(),
        ))
    }
}

/// Result of handling a packet on a link.
#[allow(clippy::large_enum_variant)]
pub enum LinkHandleResult {
    /// No action needed.
    None,
    /// Link has been activated.
    Activated,
    /// Keep-alive packet received.
    KeepAlive,
    /// Data packet received (with optional proof).
    MessageReceived(Option<Packet>),
}

/// Events that can occur on a link.
#[derive(Clone)]
#[allow(clippy::large_enum_variant)]
pub enum LinkEvent {
    /// Link has been activated.
    Activated,
    /// Data received on the link.
    Data(Box<LinkPayload>),
    /// Proof received for a message.
    Proof(Hash),
    /// Link has been closed.
    Closed,
}

/// Data associated with a link event.
#[derive(Clone)]
pub struct LinkEventData {
    /// The link ID this event is for.
    pub id: LinkId,
    /// The destination address hash.
    pub address_hash: AddressHash,
    /// The event that occurred.
    pub event: LinkEvent,
}

/// A link connection to a remote destination.
///
/// Links provide reliable, encrypted bidirectional communication
/// between Reticulum peers.
pub struct Link {
    /// Unique identifier for this link.
    id: LinkId,
    /// The destination this link connects to.
    destination: DestinationDesc,
    /// Our private identity for this link.
    priv_identity: PrivateIdentity,
    /// The peer's public identity.
    peer_identity: Identity,
    /// Derived shared key for encryption.
    derived_key: DerivedKey,
    /// Current status of the link.
    status: LinkStatus,
    /// When the link request was sent.
    request_time: Instant,
    /// Round-trip time measurement.
    rtt: Duration,
    /// Channel for sending link events.
    event_tx: tokio::sync::broadcast::Sender<LinkEventData>,
    /// Whether to prove outgoing messages.
    proves_messages: bool,
}

impl Link {
    /// Creates a new outbound link to a destination.
    ///
    /// # Arguments
    ///
    /// * `destination` - The destination to connect to
    /// * `event_tx` - Channel for sending link events
    pub fn new(
        destination: DestinationDesc,
        event_tx: tokio::sync::broadcast::Sender<LinkEventData>,
    ) -> Self {
        Self {
            id: AddressHash::new_empty(),
            destination,
            priv_identity: PrivateIdentity::new_from_rand(OsRng),
            peer_identity: Identity::default(),
            derived_key: DerivedKey::new_empty(),
            status: LinkStatus::Pending,
            request_time: Instant::now(),
            rtt: Duration::from_secs(0),
            event_tx,
            proves_messages: false,
        }
    }

    /// Sets whether to prove outgoing messages.
    ///
    /// # Arguments
    ///
    /// * `setting` - Whether to prove messages
    pub fn prove_messages(&mut self, setting: bool) {
        self.proves_messages = setting;
    }

    /// Creates a link from an incoming link request.
    ///
    /// # Arguments
    ///
    /// * `packet` - The link request packet
    /// * `signing_key` - Our signing key for the link
    /// * `destination` - The destination descriptor
    /// * `event_tx` - Channel for sending link events
    ///
    /// # Returns
    ///
    /// * `Ok(Link)` - The created link
    /// * `Err(RnsError)` - If the request is invalid
    pub fn new_from_request(
        packet: &Packet,
        signing_key: SigningKey,
        destination: DestinationDesc,
        event_tx: tokio::sync::broadcast::Sender<LinkEventData>,
    ) -> Result<Self, RnsError> {
        if packet.data.len() < PUBLIC_KEY_LENGTH * 2 {
            return Err(RnsError::InvalidArgument);
        }

        let peer_identity = Identity::new_from_slices(
            &packet.data.as_slice()[..PUBLIC_KEY_LENGTH],
            &packet.data.as_slice()[PUBLIC_KEY_LENGTH..PUBLIC_KEY_LENGTH * 2],
        );

        let link_id = LinkId::from(packet);
        log::debug!("link: create from request {}", link_id);

        let mut link = Self {
            id: link_id,
            destination,
            priv_identity: PrivateIdentity::new(StaticSecret::random_from_rng(OsRng), signing_key),
            peer_identity,
            derived_key: DerivedKey::new_empty(),
            status: LinkStatus::Pending,
            request_time: Instant::now(),
            rtt: Duration::from_secs(0),
            event_tx,
            proves_messages: false,
        };

        link.handshake(peer_identity);

        Ok(link)
    }

    /// Creates a link request packet.
    ///
    /// Returns a packet that can be sent to initiate the link.
    pub fn request(&mut self) -> Packet {
        let mut packet_data = PacketDataBuffer::new();

        packet_data.safe_write(self.priv_identity.as_identity().public_key.as_bytes());
        packet_data.safe_write(self.priv_identity.as_identity().verifying_key.as_bytes());

        let packet = Packet {
            header: Header {
                packet_type: PacketType::LinkRequest,
                ..Default::default()
            },
            ifac: None,
            destination: self.destination.address_hash,
            transport: None,
            context: PacketContext::None,
            data: packet_data,
        };

        self.status = LinkStatus::Pending;
        self.id = LinkId::from(&packet);
        self.touch();

        packet
    }

    pub fn touch(&mut self) {
        self.request_time = Instant::now();
    }

    /// Creates a proof packet for this link.
    ///
    /// Proof packets verify our identity to the peer.
    pub fn prove(&mut self) -> Packet {
        log::debug!("link({}): prove", self.id);

        if self.status != LinkStatus::Active {
            self.status = LinkStatus::Active;
            self.post_event(LinkEvent::Activated);
        }

        let mut packet_data = PacketDataBuffer::new();

        packet_data.safe_write(self.id.as_slice());
        packet_data.safe_write(self.priv_identity.as_identity().public_key.as_bytes());
        packet_data.safe_write(self.priv_identity.as_identity().verifying_key.as_bytes());

        let signature = self.priv_identity.sign(packet_data.as_slice());

        packet_data.reset();
        packet_data.safe_write(&signature.to_bytes()[..]);
        packet_data.safe_write(self.priv_identity.as_identity().public_key.as_bytes());

        Packet {
            header: Header {
                packet_type: PacketType::Proof,
                ..Default::default()
            },
            ifac: None,
            destination: self.id,
            transport: None,
            context: PacketContext::LinkRequestProof,
            data: packet_data,
        }
    }

    /// Handles an incoming data packet on this link.
    fn handle_data_packet(&mut self, packet: &Packet, out_link: bool) -> LinkHandleResult {
        if self.status != LinkStatus::Active {
            log::warn!("link({}): handling data packet in inactive state", self.id);
        }

        let first_byte = packet.data.as_slice().first().copied();
        match classify_link_data(packet.context, first_byte) {
            LinkDataAction::Message => {
                let mut buffer = [0u8; PACKET_MDU];
                if let Ok(plain_text) = self.decrypt(packet.data.as_slice(), &mut buffer[..]) {
                    log::trace!("link({}): data {}B", self.id, plain_text.len());
                    self.touch();
                    self.post_event(LinkEvent::Data(Box::new(LinkPayload::new_from_slice(
                        plain_text,
                    ))));

                    let proof = if self.proves_messages {
                        Some(self.message_proof(packet.hash()))
                    } else {
                        None
                    };

                    return LinkHandleResult::MessageReceived(proof);
                } else {
                    log::error!("link({}): can't decrypt packet", self.id);
                }
            }
            LinkDataAction::KeepAliveRequest => {
                self.touch();
                log::trace!("link({}): keep-alive request", self.id);
                return LinkHandleResult::KeepAlive;
            }
            LinkDataAction::KeepAliveResponse => {
                log::trace!("link({}): keep-alive response", self.id);
                self.touch();
                return LinkHandleResult::None;
            }
            LinkDataAction::Rtt => {
                if !out_link {
                    let mut buffer = [0u8; PACKET_MDU];
                    if let Ok(plain_text) = self.decrypt(packet.data.as_slice(), &mut buffer[..]) {
                        if let Ok(rtt) = rmp::decode::read_f64(&mut &plain_text[..]) {
                            self.rtt = Duration::from_secs_f64(rtt);
                        } else {
                            log::error!("link({}): failed to decode rtt", self.id);
                        }
                    } else {
                        log::error!("link({}): can't decrypt rtt packet", self.id);
                    }
                }
            }
            LinkDataAction::Close => {
                let mut buffer = [0u8; PACKET_MDU];
                if let Ok(plain_text) = self.decrypt(packet.data.as_slice(), &mut buffer[..]) {
                    match plain_text[..].try_into() {
                        Err(err) => {
                            log::error!(
                                "link({}): invalid decode link close payload: {}",
                                self.id,
                                err
                            )
                        }
                        Ok(dest_bytes) => {
                            let link_id = LinkId::new(dest_bytes);
                            if self.id == link_id {
                                self.close();
                            }
                        }
                    }
                } else {
                    log::error!("link({}): can't decrypt link close packet", self.id);
                }
            }
            LinkDataAction::Other => {}
        }

        LinkHandleResult::None
    }

    /// Handles an incoming packet on this link.
    ///
    /// # Arguments
    ///
    /// * `packet` - The packet to handle
    /// * `out_link` - Whether this is an outbound link
    ///
    /// # Returns
    ///
    /// The result of handling the packet
    pub fn handle_packet(&mut self, packet: &Packet, out_link: bool) -> LinkHandleResult {
        if packet.destination != self.id {
            return LinkHandleResult::None;
        }

        match packet.header.packet_type {
            PacketType::Data => self.handle_data_packet(packet, out_link),
            PacketType::Proof => self.handle_proof_packet(packet),
            _ => LinkHandleResult::None,
        }
    }

    /// Handles a proof packet on this link.
    fn handle_proof_packet(&mut self, packet: &Packet) -> LinkHandleResult {
        if self.status == LinkStatus::Pending && packet.context == PacketContext::LinkRequestProof {
            if let Ok(identity) = validate_proof_packet(&self.destination, &self.id, packet) {
                log::debug!("link({}): has been proved", self.id);

                self.handshake(identity);

                self.status = LinkStatus::Active;
                self.rtt = self.request_time.elapsed();

                log::debug!("link({}): activated", self.id);

                self.post_event(LinkEvent::Activated);

                return LinkHandleResult::Activated;
            } else {
                log::warn!("link({}): proof is not valid", self.id);
            }
        }

        if self.status == LinkStatus::Active && packet.context == PacketContext::None {
            if let Ok(hash) = validate_message_proof(&self.destination, packet.data.as_slice()) {
                self.post_event(LinkEvent::Proof(hash));
            }
        }

        LinkHandleResult::None
    }

    /// Creates a data packet to send over this link.
    ///
    /// # Arguments
    ///
    /// * `data` - The data to send
    ///
    /// # Returns
    ///
    /// * `Ok(Packet)` - The encrypted data packet
    /// * `Err(RnsError)` - If the link is closed or encryption fails
    pub fn data_packet(&self, data: &[u8]) -> Result<Packet, RnsError> {
        if self.status != LinkStatus::Active && self.status != LinkStatus::Stale {
            log::warn!("link: can't create data packet for closed link");
            return Err(RnsError::LinkClosed);
        }

        let mut packet_data = PacketDataBuffer::new();

        let cipher_text_len = {
            let cipher_text = self.encrypt(data, packet_data.accuire_buf_max())?;
            cipher_text.len()
        };

        packet_data.resize(cipher_text_len);

        Ok(Packet {
            header: Header {
                destination_type: DestinationType::Link,
                packet_type: PacketType::Data,
                ..Default::default()
            },
            ifac: None,
            destination: self.id,
            transport: None,
            context: PacketContext::None,
            data: packet_data,
        })
    }

    /// Creates a keep-alive packet.
    ///
    /// # Arguments
    ///
    /// * `data` - The keep-alive type (0xFF request, 0xFE response)
    pub fn keep_alive_packet(&self, data: u8) -> Packet {
        log::trace!("link({}): create keep alive {}", self.id, data);

        let mut packet_data = PacketDataBuffer::new();
        packet_data.safe_write(&[data]);

        Packet {
            header: Header {
                destination_type: DestinationType::Link,
                packet_type: PacketType::Data,
                ..Default::default()
            },
            ifac: None,
            destination: self.id,
            transport: None,
            context: PacketContext::KeepAlive,
            data: packet_data,
        }
    }

    /// Creates a proof for a message hash.
    ///
    /// # Arguments
    ///
    /// * `hash` - The hash to prove
    pub fn message_proof(&self, hash: Hash) -> Packet {
        log::trace!(
            "link({}): creating proof for message hash {}",
            self.id,
            hash
        );

        let signature = self.priv_identity.sign(hash.as_slice());

        let mut packet_data = PacketDataBuffer::new();
        packet_data.safe_write(hash.as_slice());
        packet_data.safe_write(&signature.to_bytes()[..]);

        Packet {
            header: Header {
                destination_type: DestinationType::Link,
                packet_type: PacketType::Proof,
                ..Default::default()
            },
            ifac: None,
            destination: self.id,
            transport: None,
            context: PacketContext::None,
            data: packet_data,
        }
    }

    /// Encrypts data using the link's derived key.
    ///
    /// # Arguments
    ///
    /// * `text` - The plaintext to encrypt
    /// * `out_buf` - Output buffer for ciphertext
    pub fn encrypt<'a>(&self, text: &[u8], out_buf: &'a mut [u8]) -> Result<&'a [u8], RnsError> {
        self.priv_identity
            .encrypt(OsRng, text, &self.derived_key, out_buf)
    }

    /// Decrypts data using the link's derived key.
    ///
    /// # Arguments
    ///
    /// * `text` - The ciphertext to decrypt
    /// * `out_buf` - Output buffer for plaintext
    pub fn decrypt<'a>(&self, text: &[u8], out_buf: &'a mut [u8]) -> Result<&'a [u8], RnsError> {
        self.priv_identity
            .decrypt(OsRng, text, &self.derived_key, out_buf)
    }

    /// Returns the destination for this link.
    pub fn destination(&self) -> &DestinationDesc {
        &self.destination
    }

    /// Creates an RTT measurement packet.
    pub fn create_rtt(&self) -> Packet {
        let rtt = self.rtt.as_secs_f64();
        let mut buf = Vec::new();
        {
            buf.reserve(8);
            rmp::encode::write_f64(&mut buf, rtt).unwrap();
        }

        let mut packet_data = PacketDataBuffer::new();

        let token_len = {
            let token = self
                .encrypt(buf.as_slice(), packet_data.accuire_buf_max())
                .expect("encrypted data");
            token.len()
        };

        packet_data.resize(token_len);

        log::trace!("link: {} create rtt packet = {} sec", self.id, rtt);

        Packet {
            header: Header {
                destination_type: DestinationType::Link,
                ..Default::default()
            },
            ifac: None,
            destination: self.id,
            transport: None,
            context: PacketContext::LinkRTT,
            data: packet_data,
        }
    }

    /// Performs the link handshake to establish encryption.
    fn handshake(&mut self, peer_identity: Identity) {
        log::debug!("link({}): handshake", self.id);

        self.status = LinkStatus::Handshake;
        self.peer_identity = peer_identity;

        self.derived_key = self
            .priv_identity
            .derive_key(&self.peer_identity.public_key, Some(self.id.as_slice()));
    }

    /// Posts an event to the event channel.
    fn post_event(&self, event: LinkEvent) {
        let _ = self.event_tx.send(LinkEventData {
            id: self.id,
            address_hash: self.destination.address_hash,
            event,
        });
    }

    /// Tears down the link, optionally sending a close packet.
    pub(crate) fn teardown(&mut self) -> Result<Option<Packet>, RnsError> {
        let packet = if self.status != LinkStatus::Pending && self.status != LinkStatus::Closed {
            let mut packet = self.data_packet(self.id.as_slice())?;
            packet.context = PacketContext::LinkClose;
            Some(packet)
        } else {
            None
        };
        self.close();
        Ok(packet)
    }

    /// Closes the link.
    pub(crate) fn close(&mut self) {
        self.status = LinkStatus::Closed;
        self.post_event(LinkEvent::Closed);
        log::warn!("link: close {}", self.id);
    }

    /// Marks the link as stale.
    pub fn stale(&mut self) {
        self.status = LinkStatus::Stale;

        log::warn!("link: stale {}", self.id);
    }

    /// Restarts the link (re-requests connection).
    pub fn restart(&mut self) {
        log::warn!(
            "link({}): restart after {}s",
            self.id,
            self.request_time.elapsed().as_secs()
        );

        self.status = LinkStatus::Pending;
    }

    /// Returns how long since the last activity.
    pub fn elapsed(&self) -> Duration {
        self.request_time.elapsed()
    }

    /// Returns the current status of the link.
    pub fn status(&self) -> LinkStatus {
        self.status
    }

    /// Returns the link ID.
    pub fn id(&self) -> &LinkId {
        &self.id
    }

    /// Returns the round-trip time.
    pub fn rtt(&self) -> &Duration {
        &self.rtt
    }
}

/// Validates a proof packet from a peer.
///
/// # Arguments
///
/// * `destination` - The destination descriptor
/// * `id` - The link ID
/// * `packet` - The proof packet
fn validate_proof_packet(
    destination: &DestinationDesc,
    id: &LinkId,
    packet: &Packet,
) -> Result<Identity, RnsError> {
    const MIN_PROOF_LEN: usize = SIGNATURE_LENGTH + PUBLIC_KEY_LENGTH;
    const MTU_PROOF_LEN: usize = SIGNATURE_LENGTH + PUBLIC_KEY_LENGTH + LINK_MTU_SIZE;
    const SIGN_DATA_LEN: usize = ADDRESS_HASH_SIZE + PUBLIC_KEY_LENGTH * 2 + LINK_MTU_SIZE;

    if packet.data.len() < MIN_PROOF_LEN {
        return Err(RnsError::PacketError);
    }

    let mut proof_data = [0u8; SIGN_DATA_LEN];

    let verifying_key = destination.identity.verifying_key.as_bytes();
    let sign_data_len = {
        let mut output = OutputBuffer::new(&mut proof_data[..]);

        output.write(id.as_slice())?;
        output.write(
            &packet.data.as_slice()[SIGNATURE_LENGTH..SIGNATURE_LENGTH + PUBLIC_KEY_LENGTH],
        )?;
        output.write(verifying_key)?;

        if packet.data.len() >= MTU_PROOF_LEN {
            let mtu_bytes = &packet.data.as_slice()[SIGNATURE_LENGTH + PUBLIC_KEY_LENGTH..];
            output.write(mtu_bytes)?;
        }

        output.offset()
    };

    let identity = Identity::new_from_slices(
        &proof_data[ADDRESS_HASH_SIZE..ADDRESS_HASH_SIZE + PUBLIC_KEY_LENGTH],
        verifying_key,
    );

    let signature = Signature::from_slice(&packet.data.as_slice()[..SIGNATURE_LENGTH])
        .map_err(|_| RnsError::CryptoError)?;

    identity
        .verify(&proof_data[..sign_data_len], &signature)
        .map_err(|_| RnsError::IncorrectSignature)?;

    Ok(identity)
}

/// Validates a message proof.
///
/// # Arguments
///
/// * `destination` - The destination descriptor
/// * `data` - The data including hash and signature
fn validate_message_proof(destination: &DestinationDesc, data: &[u8]) -> Result<Hash, RnsError> {
    if data.len() <= HASH_SIZE {
        return Err(RnsError::PacketError);
    }

    let maybe_signature = Signature::from_slice(&data[HASH_SIZE..]);
    let signature = match maybe_signature {
        Ok(s) => s,
        Err(_) => return Err(RnsError::PacketError),
    };

    let hash_slice = &data[..HASH_SIZE];

    if destination
        .identity
        .verifying_key
        .verify(hash_slice, &signature)
        .is_ok()
    {
        Ok(Hash::new(hash_slice.try_into().unwrap()))
    } else {
        Err(RnsError::IncorrectSignature)
    }
}