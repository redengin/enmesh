/// https://docs.meshcore.io/packet_format/

/// max size in bytes of path data
/// * v1.12.0 firmware and older only handled legacy 1-byte path hashes and
///   dropped packets whose path bytes exceeded 64 bytes
pub const MAX_PATH_SIZE: usize = 64;

/// max size in bytes of payload
/// * v1.12.0 firmware and older drops packets with payload sizes larger than 184
pub const MAX_PACKET_PAYLOAD: usize = 184;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct MeshCoreLoraPacket<'a> {
    pub header: MeshCoreHeader,
    /// optional transport codes (little-endian)
    pub transport_codes: Option<[u16; 2]>,
    pub path: MeshCorePath<'a>,
    pub payload: &'a [u8],
}
impl<'a> MeshCoreLoraPacket<'a> {
    /// * if Ok, returns the deserialized packet
    /// * else returns None
    pub fn from_buffer(buffer: &'a [u8]) -> Option<Self> {
        let mut used: usize = 0;

        // deserialize the header
        if let Some(header) = MeshCoreHeader::from_header(&buffer[0]) {
            used += 1;

            // deserialize transport codes (if they exist)
            let transport_codes = if header.route_type.has_transport_codes() {
                let transport_codes_0 = u16::from_le_bytes(
                    buffer[used..(used + 2)]
                        .try_into()
                        .expect("invalid dimensions"),
                );
                used += 2;
                let transport_codes_1 = u16::from_le_bytes(
                    buffer[used..(used + 2)]
                        .try_into()
                        .expect("invalid dimensions"),
                );
                used += 2;

                Some([transport_codes_0, transport_codes_1])
            } else {
                None
            };

            if let Some((path, path_used)) = MeshCorePath::from_buffer(&buffer[used..]) {
                used += path_used;

                // deserialized all the data, the rest is payload
                return Some(Self {
                    header,
                    transport_codes,
                    path,
                    payload: &buffer[used..],
                });
            } else {
                // invalid path
                return None;
            }
        } else {
            // invalid header
            return None;
        }
    }

    /// * if Ok, returns the size of the buffer used
    /// * else returns a string describing the error
    pub fn to_buffer(&self, buffer: &mut [u8]) -> Result<usize, &str> {
        let mut used = 0;

        // deserialize the header
        self.header.to_header(&mut buffer[0]);
        used += 1;

        // serialize transport codes (if they exist)
        if self.header.route_type.has_transport_codes() {
            match self.transport_codes {
                Some(transport_codes) => {
                    buffer[used..(used + 2)].copy_from_slice(&transport_codes[0].to_le_bytes());
                    used += 2;
                    buffer[used..(used + 2)].copy_from_slice(&transport_codes[1].to_le_bytes());
                    used += 2;
                }
                None => return Err("required transport codes not provided"),
            }
        }

        // serialize path
        let path_used = self.path.to_buffer(&mut buffer[used..]);
        used += path_used;

        // serialize the payload
        buffer[used..(used + self.payload.len())].copy_from_slice(self.payload);
        used += self.payload.len();

        Ok(used)
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct MeshCoreHeader {
    pub version: MeshCoreVersion,
    pub payload_type: MeshCorePayloadType,
    pub route_type: MeshCoreRouteType,
}
impl MeshCoreHeader {
    /// * if Ok, returns valid header object
    /// * else None
    fn from_header(header: &u8) -> Option<Self> {
        if let Some(version) = MeshCoreVersion::from_header(header) {
            if let Some(payload_type) = MeshCorePayloadType::from_header(header) {
                if let Some(route_type) = MeshCoreRouteType::from_header(header) {
                    return Some(Self {
                        version,
                        payload_type,
                        route_type,
                    });
                };
            };
        };

        None
    }

    fn to_header(&self, buffer: &mut u8) {
        *buffer = ((self.version as u8) << 6)
            | ((self.payload_type as u8) << 2)
            | (self.route_type as u8);
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum MeshCoreVersion {
    _1 = 0x00,
}
impl MeshCoreVersion {
    fn from_header(header: &u8) -> Option<Self> {
        let version_value = header >> 6;

        return match version_value {
            id if id == Self::_1 as u8 => Some(Self::_1),
            _ => None,
        };
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
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
impl MeshCorePayloadType {
    /// * if Ok, returns valid payload type object
    /// * else None
    fn from_header(header: &u8) -> Option<Self> {
        const PAYLOAD_TYPE_MASK: u8 = 0b0011_1100;
        let payload_type_value = (header & PAYLOAD_TYPE_MASK) >> 2;
        return match payload_type_value {
            id if id == Self::REQ as u8 => Some(Self::REQ),
            id if id == Self::RESPONSE as u8 => Some(Self::RESPONSE),
            id if id == Self::TEXT_MSG as u8 => Some(Self::TEXT_MSG),
            id if id == Self::ACK as u8 => Some(Self::ACK),
            id if id == Self::ADVERT as u8 => Some(Self::ADVERT),
            id if id == Self::GRP_TXT as u8 => Some(Self::GRP_TXT),
            id if id == Self::GRP_DATA as u8 => Some(Self::GRP_DATA),
            id if id == Self::ANON_REQ as u8 => Some(Self::ANON_REQ),
            id if id == Self::PATH as u8 => Some(Self::PATH),
            id if id == Self::TRACE as u8 => Some(Self::TRACE),
            id if id == Self::MULTIPART as u8 => Some(Self::MULTIPART),
            id if id == Self::CONTROL as u8 => Some(Self::CONTROL),
            id if id == Self::RAW_CUSTOM as u8 => Some(Self::RAW_CUSTOM),
            // no patterns matched
            _ => None,
        };
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
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
impl MeshCoreRouteType {
    /// * if Ok, returns valid payload type object
    /// * else None
    fn from_header(header: &u8) -> Option<Self> {
        const ROUTE_TYPE_MASK: u8 = 0b0000_0011;
        let route_type_value = header & ROUTE_TYPE_MASK;
        return match route_type_value {
            id if id == Self::TRANSPORT_FLOOD as u8 => Some(Self::TRANSPORT_FLOOD),
            id if id == Self::FLOOD as u8 => Some(Self::FLOOD),
            id if id == Self::DIRECT as u8 => Some(Self::DIRECT),
            id if id == Self::TRANSPORT_DIRECT as u8 => Some(Self::TRANSPORT_DIRECT),
            // no patterns matched
            _ => None,
        };
    }

    fn has_transport_codes(&self) -> bool {
        return match self {
            Self::TRANSPORT_FLOOD | Self::TRANSPORT_DIRECT => true,
            Self::FLOOD | Self::DIRECT => false,
        };
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct MeshCorePath<'a> {
    pub hop_count: u8,
    pub hash_size: MeshCoreHashSize,
    path_data: &'a [u8],
}
impl<'a> MeshCorePath<'a> {
    fn to_buffer(&self, buffer: &mut [u8]) -> usize {
        let mut used: usize = 0;

        // serialize the header (aka path_length)
        let path_length = ((self.hash_size as u8) << 6) | self.hop_count;
        buffer[used] = path_length;
        used += 1;

        // serialize the data
        let path_size = self.path_data.len();
        buffer[used..(used + path_size)].copy_from_slice(self.path_data);
        used += path_size;

        used
    }

    fn from_buffer(buffer: &'a [u8]) -> Option<(Self, usize /* bytes used */)> {
        let mut used: usize = 0;

        // parse the header (aka path_length)
        let path_length = buffer[0];
        let hop_count = path_length & 0b00111111;
        if let Some(hash_size) = MeshCoreHashSize::from_path_length(&buffer[0]) {
            used += 1;

            // take the sized slice
            let path_size = (hop_count as usize) * hash_size.len();
            let path_data = &buffer[used..(used + path_size)];
            used += path_size;

            return Some((
                Self {
                    hop_count,
                    hash_size,
                    path_data,
                },
                used,
            ));
        }

        // invalid header (aka path_length)
        None
    }
}

#[derive(Default, PartialEq, Debug, Copy, Clone)]
pub enum MeshCoreHashSize {
    /// 1 byte hashes
    #[default]
    LEGACY = 0x00,
    /// 2 byte hashes
    _2 = 0x01,
    /// 3 byte hashes
    _3 = 0x10,
}
impl MeshCoreHashSize {
    /// * if Ok, returns path hash size object
    /// * else None
    pub fn from_byte(value: &u8) -> Option<Self> {
        return match value {
            0 => Some(Self::LEGACY),
            1 => Some(Self::_2),
            2 => Some(Self::_3),
            _ => None
        }
    }

    /// * if Ok, returns path hash size object
    /// * else None
    fn from_path_length(path_length: &u8) -> Option<Self> {
        let hash_size = path_length >> 6;
        return match hash_size {
            id if id == Self::LEGACY as u8 => Some(Self::LEGACY),
            id if id == Self::_2 as u8 => Some(Self::_2),
            id if id == Self::_3 as u8 => Some(Self::_3),
            // no patterns matched
            _ => None
        };
    }

    /// returns the size (in bytes) of the hash size
    const fn len(&self) -> usize {
        return match self {
            Self::LEGACY => 1,
            Self::_2 => 2,
            Self::_3 => 3,
        };
    }
}

// TESTING
//--------------------------------------------------------------------------------
#[cfg(test)]
mod tests {

    use super::*;

    /// evaluates the ability to serialize and deserialize by
    /// * serializing the original packet
    /// * deserializing from the serialized original packet
    /// * asserting that the deserialized packet matches the original packet
    fn serde_harness(packet: &MeshCoreLoraPacket) {

        // serialize
        let mut buffer = [0u8; 1000];
        let buffer_used = match packet.to_buffer(&mut buffer) {
            Ok(used) => used,
            Err(e) => {
                panic!("failed to serialize [{e}]")
            }
        };

        // deserialize
        match MeshCoreLoraPacket::from_buffer(&buffer[..buffer_used]) {
            Some(expected_packet) => {
                // the deserialized result should match the original packet
                assert_eq!(*packet, expected_packet);

            }
            None => panic!("failed to deserialize"),
        }
    }

    #[test]
    #[allow(non_snake_case)]
    fn serde_MeshCorePacket_NoTransportCodes() {
        const PACKET: MeshCoreLoraPacket = MeshCoreLoraPacket {
            header: MeshCoreHeader {
                version: MeshCoreVersion::_1,
                payload_type: MeshCorePayloadType::ACK,
                route_type: MeshCoreRouteType::DIRECT,
            },
            transport_codes: None,
            path: MeshCorePath {
                hop_count: 0,
                hash_size: MeshCoreHashSize::LEGACY,
                path_data: &[u8::MAX; 0],
            },
            payload: &[1u8; 100],
        };

        serde_harness(&PACKET);
    }

    #[test]
    #[allow(non_snake_case)]
    fn serde_MeshCorePacket_TransportCodes() {
        const PACKET: MeshCoreLoraPacket = MeshCoreLoraPacket {
            header: MeshCoreHeader {
                version: MeshCoreVersion::_1,
                payload_type: MeshCorePayloadType::ACK,
                route_type: MeshCoreRouteType::TRANSPORT_DIRECT,
            },
            transport_codes: Some([1u16, 1u16]),
            path: MeshCorePath {
                hop_count: 0,
                hash_size: MeshCoreHashSize::LEGACY,
                path_data: &[u8::MAX; 0],
            },
            payload: &[1u8; 100],
        };

        serde_harness(&PACKET);
    }

    #[test]
    #[allow(non_snake_case)]
    fn serde_MeshCorePacket_Path() {
        const PACKET: MeshCoreLoraPacket = MeshCoreLoraPacket {
            header: MeshCoreHeader {
                version: MeshCoreVersion::_1,
                payload_type: MeshCorePayloadType::ACK,
                route_type: MeshCoreRouteType::DIRECT,
            },
            transport_codes: None,
            path: MeshCorePath {
                hop_count: 1,
                hash_size: MeshCoreHashSize::LEGACY,
                path_data: &[u8::MAX; MeshCoreHashSize::LEGACY.len()],
            },
            payload: &[1u8; 100],
        };

        serde_harness(&PACKET);
    }
}
