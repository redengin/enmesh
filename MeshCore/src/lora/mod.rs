/// https://docs.meshcore.io/packet_format/

/// max size in bytes of path data
/// * v1.12.0 firmware and older only handled legacy 1-byte path hashes and
///   dropped packets whose path bytes exceeded 64 bytes
pub const MAX_PATH_SIZE: usize = 64;

/// max size in bytes of payload
/// * v1.12.0 firmware and older drops packets with payload sizes larger than 184
pub const MAX_PACKET_PAYLOAD: usize = 184;

pub struct MeshCoreLoraPacket<'a> {
    pub version: MeshCoreVersion,
    pub payload_type: MeshCorePayloadType,
    pub route_type: MeshCoreRouteType,
    /// optional transport codes (little-endian)
    pub transport_codes: Option<&'a [u16]>,
    pub path: MeshCorePath<'a>,
    pub payload: &'a [u8],
}
impl<'a> MeshCoreLoraPacket<'a> {
    pub fn from_buffer(buffer: &'a [u8]) -> Option<Self> {
        todo!()
    }
    pub fn to_buffer(buffer: &mut [u8]) -> Option<Self> {
        todo!()
    }
}



pub enum MeshCoreVersion {
    _1 = 0x00,
}

#[allow(non_camel_case_types)]
pub enum MeshCorePayloadType {
    /// Request (destination/source hashes + MAC)
    REQ = 0x00,
    /// Response to REQ or ANON_REQ
    RESPONSE = 0x01,
    /// Plain text message
    TEXT_MSG = 0x02,
    /// Acknowledgment
    ACK = 0x03,
    /// Node advertisement
    ADVERT = 0x04,
    /// Group text message (unverified)
    GRP_TXT = 0x05,
    /// Group datagram (unverified)
    GRP_DATA = 0x06,
    /// Anonymous request
    ANON_REQ = 0x07,
    /// Returned path
    PATH = 0x08,
    /// Trace a path, collecting SNR for each hop
    TRACE = 0x09,
    /// Packet is part of a sequence of packets
    MULTIPART = 0x0A,
    /// Control packet data (unencrypted)
    CONTROL = 0x0B,
    /// Custom packet (raw bytes, custom encryption)
    RAW_CUSTOM = 0x0F,
}

#[allow(non_camel_case_types)]
pub enum MeshCoreRouteType {
    /// Flood Routing + Transport Codes
    TRANSPORT_FLOOD = 0x00,
    /// Flood Routing
    FLOOD = 0x01,
    /// Direct Routing
    DIRECT = 0x02,
    /// Direct Routing + Transport Codes
    TRANSPORT_DIRECT = 0x03,
}

pub struct MeshCorePath<'a> {
    pub hop_count: u8,
    pub hash_size: MeshCoreHashSize,
    path_data: &'a [u8],
}
impl<'a> MeshCorePath<'a> {
    // TODO support access to hops as necessary
}

#[derive(Default)]
pub enum MeshCoreHashSize {
    /// 1 byte hashes
    #[default]
    LEGACY = 0x00,
    /// 2 byte hashes
    _2 = 0x01,
    /// 3 byte hashes
    _3 = 0x10,
}


// TESTING
//--------------------------------------------------------------------------------
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    #[allow(non_snake_case)]
    fn serde_MeshCorePacket() {
        {
            // No transport codes, LEGACY (default) 1 byte hash for paths
            const PACKET: MeshCoreLoraPacket = MeshCoreLoraPacket{
                version: MeshCoreVersion::_1,
                payload_type: MeshCorePayloadType::ACK,
                route_type: MeshCoreRouteType::DIRECT,
                transport_codes: None,
                path: MeshCorePath {
                    hop_count: 1,
                    hash_size: MeshCoreHashSize::LEGACY,
                    path_data: &[1u8; 100],
                },
                payload: &[1u8, 100],
            };


        }
    }
}
