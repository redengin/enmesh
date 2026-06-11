/// https://docs.meshcore.io/packet_format/

/// max size in bytes of path data
/// * v1.12.0 firmware and older only handled legacy 1-byte path hashes and
///   dropped packets whose path bytes exceeded 64 bytes
pub const MAX_PATH_SIZE: usize = 64;

/// max size in bytes of payload
/// * v1.12.0 firmware and older drops packets with payload sizes larger than 184
pub const MAX_PACKET_PAYLOAD: usize = 184;

#[derive(Copy, Clone)]
pub struct MeshCoreLoraPacket<'a> {
    pub header: MeshCoreHeader<'a>,
    // pub version: MeshCoreVersion,
    // pub payload_type: MeshCorePayloadType,
    // pub route_type: MeshCoreRouteType,
    /// optional transport codes (little-endian)
    pub transport_codes: Option<&'a [u16]>,
    pub path: MeshCorePath<'a>,
    pub payload: &'a [u8],
}
impl<'a> MeshCoreLoraPacket<'a> {
    pub fn from_buffer(buffer: &'a [u8]) -> Option<Self> {
        // let Some((version, payload_type, route_type)) = MeshCoreHeader::from(buffer[0])
        // {

        // }

        None
    }

    /// * if Ok, returns the size of the buffer used
    /// * else returns a string describing the error
    pub fn to_buffer(&self, buffer: &mut [u8]) -> Result<usize, &str> {
        let mut used = 0;
        // let header = ((self.version as u8) << 6)
        //     | ((self.payload_type as u8) << 2)
        //     | (self.route_type as u8);
        // buffer[used] = header;
        // used += 1;

        if let Some(transport_codes) = self.transport_codes {
            buffer[used..(used + 2)].copy_from_slice(&transport_codes[0].to_le_bytes());
            used += 2;
            buffer[used..(used + 2)].copy_from_slice(&transport_codes[1].to_le_bytes());
            used += 2;
        }

        let path_length = ((self.path.hash_size as u8) << 6) | self.path.hop_count;
        buffer[used] = path_length;
        used += 1;

        let path_size = (self.path.hop_count as usize) * self.path.hash_size.len();
        buffer[used..(used + path_size)].copy_from_slice(self.path.path_data);
        used += path_size;

        buffer[used..(used + self.payload.len())].copy_from_slice(self.payload);
        used += self.payload.len();
        assert_eq!(100, self.payload.len());

        Ok(used)
    }
}


#[derive(Copy, Clone)]
pub struct MeshCoreHeader<'a> {
    header: &'a u8,
}
impl<'a> MeshCoreHeader<'a> {
    pub fn version(&self) -> Option<MeshCoreVersion>
    {
        MeshCoreVersion::from_header(self.header)
    }
    pub fn packet_type(&self) -> Option<MeshCorePayloadType>
    {
        MeshCorePayloadType::from_header(self.header)
    }
    pub fn route_type(&self) -> Option<MeshCoreRouteType>
    {
        MeshCoreRouteType::from_header(self.header)
    }

}

#[derive(Copy, Clone)]
pub enum MeshCoreVersion {
    _1 = 0x00,
}
impl MeshCoreVersion {
    pub fn from_header(header: &u8) -> Option<Self>
    {
        let version_value= header >> 6;
        if version_value == Self::_1 as u8 {
            return Some(Self::_1)
        }

        None
    }
}

#[derive(Copy, Clone)]
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
    pub fn from_header(header: &u8) -> Option<Self>
    {
        const PAYLOAD_TYPE_MASK: u8 = 0b00111100;
        let payload_type_value = (header & PAYLOAD_TYPE_MASK) >> 2;
        if payload_type_value == Self::REQ as u8 {
            return Some(Self::REQ);
        }
        if payload_type_value == Self::RESPONSE as u8 {
            return Some(Self::RESPONSE);
        }
        if payload_type_value == Self::TEXT_MSG as u8 {
            return Some(Self::TEXT_MSG);
        }
        if payload_type_value == Self::ACK as u8 {
            return Some(Self::ACK);
        }
        if payload_type_value == Self::ADVERT as u8 {
            return Some(Self::ADVERT);
        }
        if payload_type_value == Self::GRP_TXT as u8 {
            return Some(Self::GRP_TXT);
        }
        if payload_type_value == Self::GRP_DATA as u8 {
            return Some(Self::GRP_DATA);
        }
        if payload_type_value == Self::ANON_REQ as u8 {
            return Some(Self::ANON_REQ);
        }
        if payload_type_value == Self::PATH as u8 {
            return Some(Self::PATH);
        }
        if payload_type_value == Self::TRACE as u8 {
            return Some(Self::TRACE);
        }
        if payload_type_value == Self::MULTIPART as u8 {
            return Some(Self::MULTIPART);
        }
        if payload_type_value == Self::CONTROL as u8 {
            return Some(Self::CONTROL);
        }
        if payload_type_value == Self::RAW_CUSTOM as u8 {
            return Some(Self::RAW_CUSTOM);
        }

        None
    }
}

#[derive(Copy, Clone)]
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
    pub fn from_header(header: &u8) -> Option<Self>
    {
        // let payload_type_bits = header >> 6;
        // if payload_type_bits == Self::_1 as u8 {
        //     return Some(Self::_1)
        // }

        None
    }
}


#[derive(Copy, Clone)]
pub struct MeshCorePath<'a> {
    pub hop_count: u8,
    pub hash_size: MeshCoreHashSize,
    path_data: &'a [u8],
}
impl<'a> MeshCorePath<'a> {
    // TODO support access to hops as necessary
}

#[derive(Default, Copy, Clone)]
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
    pub fn len(&self) -> usize {
        return match self {
            Self::LEGACY => 1,
            Self::_2 => 2,
            Self::_3 => 3,
        }
    }
}

// TESTING
//--------------------------------------------------------------------------------
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    #[allow(non_snake_case)]
    fn serde_MeshCorePacket_NoTransportCodes_1_byte_hash_path() {
        // No transport codes, LEGACY (default) 1 byte hash for paths
        // const PACKET: MeshCoreLoraPacket = MeshCoreLoraPacket {
        //     version: MeshCoreVersion::_1,
        //     payload_type: MeshCorePayloadType::ACK,
        //     route_type: MeshCoreRouteType::DIRECT,
        //     transport_codes: None,
        //     path: MeshCorePath {
        //         hop_count: 1,
        //         hash_size: MeshCoreHashSize::LEGACY,
        //         path_data: &[1u8; 1],
        //     },
        //     payload: &[1u8; 100],
        // };

        // // test serialization
        // let mut buffer = [0u8; 1000];
        // let buffer_used = match PACKET.to_buffer(&mut buffer) {
        //     Ok(used) => {
        //         used
        //     }
        //     Err(e) => {
        //         panic!("failed to serialize [{e}]")
        //     }
        // };

        // // test deserialization
        // match MeshCoreLoraPacket::from_buffer(&buffer[..buffer_used])
        // {
        //     Some(_packet) => {

        //     }
        //     None => panic!("failed to deserialize")
        // }
    }
}
