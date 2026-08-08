#[cfg(feature = "std")]
use std::Vec;
#[cfg(all(not(feature = "std")))]
use heapless::Vec;

/// https://datasheet4u.com/pdf/1577256/SSD1680.pdf#page=17
pub enum Command {
    // DriverOutputControl,
    // GateVoltageControl,
    // SourceVoltageControl,
    // OtpStart,
    // OtpWrite(OtpData),
    // OtpSetting,
    // DeepSleep(DeepSleepMode),
    // DataEntryMode(DataSequence, bool),
    // BoosterControl,
    // Reset,
    // /// true: use internal sensor, false: use external sensor
    // TempSensorSelect(bool),
    // TempSensorWrite {
    //     a: u8,
    //     b: u8,
    // },
    // MasterUpdate,
    // DisplayUpdateControl1 {
    //     red: RamOption,
    //     bw: RamOption,
    //     source: SourceAddress,
    // },
    // DisplayUpdateControl2(UpdateSequence),
    // StartBwWrite,
    // StartRedWrite,
    // VcomWrite(u8),
    // OtpDisplayOption,
    // Status,
    // ProgramWaveform,
    // WriteLut,
    // OtpProgramMode,
    // BorderWaveform,
    // SetRamAddressX {
    //     start: u8,
    //     stop: u8,
    // },
    // SetRamAddressY {
    //     start: u8,
    //     stop: u8,
    // },
    // SetRamCounterX(u8),
    // SetRamCounterY(u8),
}
// pub struct OtpData {
//     a: u8,
//     b: u8,
//     c: u8,
//     d: u8,
// }

// pub enum DeepSleepMode {
//     Normal = 0b00,
//     DeepSleep1 = 0b01,
//     DeepSleep2 = 0b11,
// }

// pub enum DataSequence {
//     YDecXDec = 0b00,
//     YDecXInc = 0b01,
//     YIncXDec = 0b10,
//     YIncXInc = 0b11,
// }

// pub enum RamOption {
//     Normal = 0b0000,
//     Bypass = 0b0100,
//     Inverse = 0b1000,
// }

// pub enum SourceAddress {
//     S0 = 0b0,
//     S8 = 0b1,
// }

// pub enum UpdateSequence {/* not implemented */}

impl Command {
    pub fn id(&self) -> u8 {
        match self {
            // Self::DriverOutputControl => 0x01,
            // Self::GateVoltageControl => 0x03,
            // Self::SourceVoltageControl => 0x04,
            // Self::OtpStart => 0x08,
            // Self::OtpWrite(_) => 0x09,
            // Self::OtpSetting => 0x0A,
            // Self::DeepSleep(_) => 0x10,
            // Self::DataEntryMode(_, _) => 0x11,
            // Self::BoosterControl => 0x0C,
            // Self::Reset => 0x12,
            // Self::TempSensorSelect(_) => 0x18,
            // Self::TempSensorWrite { .. } => 0x1A,
            // Self::MasterUpdate => 0x20,
            // Self::DisplayUpdateControl1 { .. } => 0x21,
            // Self::DisplayUpdateControl2 { .. } => 0x22,
            // Self::StartBwWrite => 0x24,
            // Self::StartRedWrite => 0x26,
            // Self::VcomWrite(_) => 0x2C,
            // Self::OtpDisplayOption => 0x2D,
            // Self::Status => 0x2F,
            // Self::ProgramWaveform => 0x30,
            // Self::WriteLut => 0x32,
            // Self::OtpProgramMode => 0x39,
            // Self::BorderWaveform => 0x3C,
            // Self::SetRamAddressX { .. } => 0x44,
            // Self::SetRamAddressY { .. } => 0x45,
            // Self::SetRamCounterX(_) => 0x4E,
            // Self::SetRamCounterY(_) => 0x4F,
        }
        0
    }

    pub fn data(&self) -> Vec<u8, 10> {
        let mut ret = Vec::<u8, 10>::new();
        match self {
            // Self::DriverOutputControl => {
            //     ret.push(0xF9).unwrap();
            //     ret.push(0).unwrap();
            //     ret.push(0).unwrap();
            // }
            // Self::GateVoltageControl => {
            //     ret.push(0x17).unwrap();
            // }
            // Self::SourceVoltageControl => {
            //     ret.push(0x41).unwrap(); // VSH1 15V
            //     ret.push(0xAC).unwrap(); // VSH2 5.4V
            //     ret.push(0x32).unwrap(); // VSL  -15V
            // }
            // Self::OtpStart => { /* no data */ }
            // Self::OtpWrite(data) => {
            //     ret.push(data.a).unwrap();
            //     ret.push(data.b).unwrap();
            //     ret.push(data.c).unwrap();
            //     ret.push(data.d).unwrap();
            // }
            // Self::OtpSetting => { /* no data */ }
            // Self::DeepSleep(mode) => {
            //     let _ = match mode {
            //         DeepSleepMode::Normal => ret.push(0b00).unwrap(),
            //         DeepSleepMode::DeepSleep1 => ret.push(0b01).unwrap(),
            //         DeepSleepMode::DeepSleep2 => ret.push(0b11).unwrap(),
            //     };
            // }
            // Self::DataEntryMode(sequence, column_or_row) => {
            //     match sequence {
            //         DataSequence::YDecXDec => ret.push(0b00).unwrap(),
            //         DataSequence::YDecXInc => ret.push(0b01).unwrap(),
            //         DataSequence::YIncXDec => ret.push(0b10).unwrap(),
            //         DataSequence::YIncXInc => ret.push(0b11).unwrap(),
            //     };
            // }
            // Self::BoosterControl => { todo!() }
            // Self::Reset => { /* no data */ }
            // Self::TempSensorSelect(internal) => {
            //     if *internal {
            //        ret.push(0x80).unwrap();
            //     }
            //     else {
            //        ret.push(0x48).unwrap();
            //     }
            // }
            // Self::TempSensorWrite { .. } => { todo!() }
            // Self::MasterUpdate => { /* no data */ }
            // Self::DisplayUpdateControl1 { .. } => { todo!() }
            // Self::DisplayUpdateControl2(_) => { todo!() }
            // Self::StartBwWrite => { /* no data */ }
            // Self::StartRedWrite => { /* no data */ }
            // Self::VcomWrite(value) => ret.push(*value).unwrap(),
            // Self::OtpDisplayOption => { todo!() }
            // Self::Status => { todo!() }
            // Self::ProgramWaveform => { todo!() }
            // Self::WriteLut => { todo!() }
            // Self::OtpProgramMode => { todo!() }
            // Self::BorderWaveform => {
            //     ret.push(0x01).unwrap();
            // }
            // Self::SetRamAddressX { .. } => { todo!() }
            // Self::SetRamAddressY { .. } => { todo!() }
            // Self::SetRamCounterX(_) => { todo!() }
            // Self::SetRamCounterY(_) => { todo!() }
        }

        return ret;
    }
}
