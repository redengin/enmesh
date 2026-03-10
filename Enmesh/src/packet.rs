pub struct RawPacket<'a> {
    /// identify what geographical area the packet came from
    pub from: Option<Location>,
    pub protocol: PacketProtocol,
    pub data: &'a [u8],
}

/// supported protocols for enmesh endpoints
pub enum PacketProtocol {
    MeshTastic = 1,
    MeshCore = 2,
    EnMesh = 3,
}

/// geographical source of the packet
/// bridge nodes can determine how to use the packet depending on it's source area
pub struct Location {
    pub city: CityLocation,
}

/// rough geographical location (11.1km area)
pub struct CityLocation {
    /// latitude * 10 for city level ~11.1km area
    pub latitude: i16,

    /// longitude * 10 for city level ~11.1km area
    pub longitude: i16,
}
impl CityLocation {
    // pub fn new(latitude: f32, longitude: f32) -> Self {
    pub fn new(latitude: f32, longitude: f32) -> Self {

        Self {
            latitude: (latitude * 10.0) as i16,
            longitude: (longitude * 10.0) as i16,
        }
    }
}
