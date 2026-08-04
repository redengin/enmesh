use heapless::Vec;


/// https://resource.heltec.cn/download/Wireless_Paper/E-Ink%20Datasheet/E-INK%20V1.0(DEPG0213BNS800F41-2.0)%20.pdf#page=15
pub enum Commands {
    DriverOutputControl,
    GateVoltageControl,
    SourceVoltageControl,
    OtpStart,
    OtpWrite(OtpData),
}
struct OtpData {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
}

impl Commands {
    pub fn command(&self) -> u8 {
        match self {
            Self::DriverOutputControl => 0x01,
            Self::GateVoltageControl => 0x03,
            Self::SourceVoltageControl => 0x04,
            Self::OtpStart => 0x08,
            Self::OtpWrite(_) => 0x09,
        }
    }

    pub fn data(&self) -> Vec<u8, 10> {
        let ret = Vec::<u8, 10>::new();
        // match self {
        //     Self::DriverOutputControl => { ret.push(0xF9); ret.Some(&[0xF9, 0, 0]),
        //     Self::GateVoltageControl => Some(&[0x17]), // [POR], VGH 20V[POR]
        //     Self::SourceVoltageControl => Some(&[0x41, 0xAC, 0x32]), // VSH1 15V, VSH2 5.4V, VSL -15V
        //     Self::OtpStart => None,
        //     Self::OtpWrite(data) => Some(&[
        //         data.a,
        //     ]),
        // }

        ret
    }
}
