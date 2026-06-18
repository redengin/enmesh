use crate::lora::MeshCoreHashSize;

/// helper macro to provide sscanf functionality
/// NOTE: str.split doesn't dedupe multiple separators (i.e. only one separator char is supported)
macro_rules! scan {
    ( $string:expr, $( $x:ty ),+ ) => {{
        let mut iter = $string.split(is_separator);
        ($(iter.next().and_then(|word| word.parse::<$x>().ok()),)*)
    }}
}
/// use the MeshCore seperators
fn is_separator(c: char) -> bool {
    return if c.is_whitespace() || (c == ',') {
        true
    } else {
        false
    };
}

/// https://docs.meshcore.io/cli_commands/
///
/// commands that can be sent to MeshCore Repeaters, Room Servers and Sensors.
#[derive(Debug, PartialEq)]
pub enum CliCommands<'a> {
    /// "reboot" - reboot the device
    Reboot,
    /// "poweroff" or "shutdown" - poweroff the device
    PowerOff,
    /// "clkreboot" - reset the clock and reboot
    ResetClockAndReboot,
    /// "clock sync" - sync the clock with the remote device
    ClockSync,
    /// "clock" - show the time in UTC
    ShowClock,
    /// "time <epoch seconds>" - set the clock
    SetClock(u32),
    /// "advert" - send a flood advert
    SendFloodAdvert,
    /// "advert.zerohop" - send a zero-hop advert
    SendZeroHopAdvert,
    /// "start ota" - start an over-the-air update
    StartOta,
    /// "erase" - perform a factory reset
    FactoryReset,
    /// "neighbors" - show last 8 received adverts ({pubkey-prefix}:{timestamp}:{snr*4})
    ShowLastAdverts,
    /// "neighbor.remove <pubkey_prefix>"
    RemoveNeighbor(&'a str),
    /// "discover.neighbors" - discover zero-hop neighbors
    DiscoverZeroHopNeighbors,

    /// "clear stats" - clear all statistics
    ClearStatistics,
    /// "stats-core" - show battery level, uptime, tx queue length, and debug flags
    ShowCoreStats,
    /// "stats-radio" - show noise floor, last rssi/snr, airtime, receive errors
    ShowRadioStats,
    /// "stats-packet" - show packet counters (received, sent)
    ShowPacketStats,

    /// "log start" - begin logging rx
    StartRxLog,
    /// "log stop" - stop logging rx
    StopRxLog,
    /// "log erase" - erase the logged data
    EraseRxLog,
    /// "log" - show the logged data
    ShowRxLog,

    /// "ver" - show version
    ShowVersion,
    /// "board" - show hardware name
    ShowHardwareName,

    /// "get radio" - show radio config (<freq>,<bw>,<sf>,<cr>)
    ShowRadioConfig,
    /// "set radio <freq>,<bw>,<sf>,<cr>" - set the radio config
    SetRadioConfig { freq: f32, bw: f32, sf: u8, cr: u8 },
    /// "get tx" - show tx power (integer dBm)
    ShowTxPower,
    /// "set tx <dbm>" - set tx power (integer dBm)
    SetTxPower(i8),
    /// "tempradio <freq>,<bw>,<sf>,<cr>,<timeout_mins>" - change radio parameters for a duration (minutes)
    SetTempRadioConfig {
        freq: f32,
        bw: f32,
        sf: u8,
        cr: u8,
        duration_minutes: u8,
    },
    /// "get freq"- show the radio frequency
    ShowFreq,
    /// "set freq <frequency>" - set the radio frequency (in MHz)
    SetFreq(f32),
    /// "get radio.rxgain" - show if rxgain is enabled
    ShowRxGainStatus,
    /// "set radio.rxgain <state>" - "on": enable rx gain
    ///                              "off": disable rx gain
    SetRxGain(bool),

    /// "get name" - show the name of this device
    ShowName,
    /// "set name <name>" - set the name of this device
    SetName(&'a str),
    /// "get lat" - show the latitude of the device
    ShowLat,
    /// "set lat <degrees>" - set the longitude
    SetLat(f32),
    /// "get lon" - show the longitude of the device
    ShowLon,
    /// "set lon <degrees>" - set the longitude
    SetLon(f32),
    /// "get prv.key" - show the private key
    ShowPrivateKey,
    /// "set prv.key <private key>" - set the private key
    SetPrivateKey(&'a str),
    /// "password <new password>" - change the admin password
    SetAdminPassword(&'a str),
    /// "get guest.password" - show the guest password
    ShowGuestPassword,
    /// "set guest.password <password>" - set the guest password
    SetGuestPassword(&'a str),
    /// "get owner.info" - show the owner information (text where '|' treated as newline)
    ShowOwnerInfo,
    /// "set owner.info <text>" - set the owner information
    SetOwnerInfo(&'a str),
    /// "get adc.multiplier" - show the battery ADC scaling
    GetBatteryGain,
    /// "set adc.multiplier <value>" - set the battery ADC scaling
    SetBatteryGain(f32),
    /// "get public.key" - show the public key
    ShowPublicKey,
    /// "get role" - show this nodes role
    ShowRole,
    /// "powersaving" - show if power saving is enabled
    ShowPowerSaving,
    /// "powersaving on" - enable power saving
    EnablePowerSaving,
    /// "powersaving off" - disable power saving
    DisablePowerSaving,

    /// "get repeat" - show if repeating is enabled
    ShowRepeat,
    /// "set repeat <state>" - "on": enable / "off": disable
    SetRepeat(bool),
    /// "get path.hash.mode" - show advert path hash-size
    ShowHashSize,
    /// "set path.hash.mode <value>" - 0: 1 byte hash, 1: 2 byte hash, 2: 3 byte hash
    SetHashHize(MeshCoreHashSize),
    /// "get loop.detect" - show if loop-detection enabled
    ShowLoopDetection,
    /// "set loop.detect <state>" - "off": disabled,
    ///                             "minimal"  : 4 or more for 1 byte hash,
    ///                                          2 or more for 2 byte hash,
    ///                                          1 or more for 3 byte hash
    ///                             "moderate" : 2 or more for 1 byte hash,
    ///                                          1 or more for 2 byte hash,
    ///                                          1 or more for 3 byte hash
    ///                             "strict"   : 1 or more (for any byte hash)
    SetLoopDetection(LoopDetection),
    /// "get txdelay" - show transmit delay factor
    ShowTransmitDelay,
    /// "set txdelay <value>" - [0.0 .. 2.0] delay factor
    SetTransmitDelay(f32),
    /// "get rxdelay" - show the receive detlay factor
    ShowReceiveDelay,
    /// "set rxdelay <value>" - [0.0 .. 20.0]
    SetReceiveDelay(f32),
    /// "get dutycycle" - show duty cycle
    ShowDutyCycle,
    /// "set dutycycle <value>" - [0 .. 100]
    SetDutyCycle,
    /// "get af" - show airtime factor
    ShowAirtimeFactor,
    /// "set af <value>" - [0.0 .. 9.0]
    SetAirtimeFactor(f32),
    /// "get int.thresh" - show the interference threshold
    ShowInterferenceThreshold,
    /// "set int.thresh <value>"
    SetInterferenceThreshold(f32),
    /// "get agc.reset.interval" - show AGC reset interval
    ShowAgcResetInterval,
    /// "set agc.reset.interval <value>" - multiple of 4 seconds (rounds down)
    SetAgcResetInterval(u8),
    /// "get multi.acks" - show if Multi-Acks enabled
    ShowMultiAcksEnabled,
    /// "set multi.acks <state>" - 0: disable, 1: enable
    SetMultiAcksEnabled(bool),
    /// "get advert.interval" - show the advertisement interval
    ShowAdvertInterval,
    /// "set advert.interval <minutes>" - multiple of 2 seconds (rounds down) [60 .. 240]
    SetAdvertInterval(u8),
    /// "get flood.max.unscoped" - show max hop count for unscoped packets
    ShowUnscopedMaxHopCount,
    /// "set flood.max.unscoped <value>" - [0 .. 64]
    SetUnscopedMaxHopCount(u8),
    /// "get flood.max.advert" - show max hop count for flood advert
    ShowFloodAdvertMaxHopCount,
    /// "set flood.max.advert <value>" - [0 .. 64]
    SetFloodAdvertMaxHopCount(u8),

    /// "setperm <pubkey> <permissions>"
    SetAclPermissions {
        pubkey: &'a [u8],
        permissions: PermissionLevel,
    },
    /// "get acl" - show the ACL
    ShowAcl,
    /// "get allow.read.only" - show if this room is read-only
    ShowRoomMode,
    /// "set allow.read.only <state>" - "on": read-only, "off" - read-write
    SetRoomMode { read_only: bool },

    /// "region load <name> [flood_flag]" - name: "*" represents wildcard region
    ///                                     (optional)flood_flag: "F" to allow flooding
    SetRegion { name: &'a str, allow_flood: bool },
    /// "region save" - save changes to region
    SaveRegion,
    /// "region allowf <name>" - allow forwarding for region, name: "*" represents wildcard region
    AllowRegion { name: &'a str },
    /// "region denyf <name>" - deny forwarding for region, name: "*" represents wildcard region
    DenyRegion { name: &'a str },
    /// "region get <name>" - show information for region
    ShowRegion { name: &'a str },
    /// "region home" - show home region of this node
    ShowHomeRegion,
    /// "region home <name>" - name: <null> to remove the region
    SetHomeRegion { name: Option<&'a str> },
    /// "region default" - show default scope region for this node
    ShowDefaultRegion,
    /// "region default {name|<null>}" - set the default region
    SetDefaultRegion { name: Option<&'a str> },
    /// "region put <name> [parent_name]" - create a new region
    CreateRegion { name: &'a str, parent_name: &'a str },
    /// "region def <token> [<token>...]" - define region hierarchy using a single line
    ///            tokens: <name> - create name as child of current cursor
    ///                    <name>|<jump> - where jump exists in the previous tokens
    DefineRegionHierarchy {
        region: &'a str,
        tokens: &'a [&'a str],
    },
    /// "region remove <name>" - remove a region
    RemoveRegion { name: &'a str },
    /// "region list <filter>" - show regions, filter: "allowed", "denied"
    /// * allowed_denied - true: show the "allowed", false: show the "denied"
    ShowRegionList { allowed_denied: bool },

    /// "gps" - show if GPS is enabled
    ShowGps,
    /// "gps <state>" - <state> "on": enable GPS, "off": disable GPS
    SetGps { enabled: bool },
    /// "gps sync" - sync time with GPS
    SyncGpsTime,
    /// "gps setloc" - use GPS to set the location
    SyncGpsLocation,
    /// "gps advert" - show the GPS advert policy
    ShowGpsAdvertPolicy,
    /// "gps advert <policy>" - set the GPS advert policy
    /// * <policy>
    ///     * "none"
    ///     * "share"
    ///     * "prefs"
    SetGpsAdvertPolicy(u8),

    /// "sensor list [start]" - show sensors, optionally start at [start] index
    ShowSensors { start_index: u8 },
    /// "sensor get <key>" - show the value of a sensor
    ShowSensor { key: u8 },
    /// "sensor set <key> <value>" - set the value of a sensor
    SetSensor { key: u8, value: u8 },

    /// "get bridge.type" - show the bridging mode
    ShowBridgeType,
    /// "get bridge.enabled" - show whether bridging is enabled
    ShowBridgingEnabled,
    /// "set bridge.enabled <state>" - <state> "on": enabled, "off": disabled
    SetBridingEnabled { support_bridge: bool },
    /// "get bridge.delay" - show the bridge delay
    ShowBridgeDelay,
    /// "set bridge.delay <ms>" - set the bridge delay in ms
    SetBridgeDelay { delay_ms: u16 },
    /// "get bridge.source" - show the number of pakcets on the bridge
    ShowBridgeSource,
    /// "set bridge.source <source>" - //TODO wtf is this
    SetBridgeSource(),
    /// "get bridge.baud" - show the baudrate supported by the bridge
    ShowBridgeBaud,
    /// "set bridge.baud <rate>" - rate: [9600, 19200, 38400, 57600, 115200]
    SetBridgeBaud { baud: u16 },
    /// "get bridge.channel" - show the channel for the bridge
    ShowBridgeChannel,
    /// "set bridge.channel <channel>" - channel: [1 .. 14]
    SetBridgeChannel { channel: u8 },
    /// "get bridge.secret" - show the ESP-NOW secret
    ShowBridgeSecret,
    /// "set bridge.secret <secret>" - set the ESP-NOW secret
    SetBridgeSecret { secret: &'a [u8] },
    /// "get bootloader.ver" - show the NRF52 bootloader version
    ShowBootLoaderVersion,
    /// "get pwrmgt.support" - show the power management support
    ShowPowerManagementSupport,
    /// "get pwrmgt.source" - show the power source
    ShowPowerSource,
    /// "get pwrmgmt.bootreason"
    ShowBootReasons,
    /// "get pwrmgt.bootmv" - show teh boot voltage
    ShowBootVoltage,
}
impl<'a> CliCommands<'a> {
    pub fn from_string(s: &'a str) -> Result<Self, &'a str> {
        {
            const COMMAND_STRING: &str = "reboot";
            if s.eq(COMMAND_STRING) {
                return Ok(Self::Reboot);
            }
        }
        {
            const COMMAND_STRING: &str = "poweroff";
            if s.eq(COMMAND_STRING) {
                return Ok(Self::PowerOff);
            }
        }
        {
            const COMMAND_STRING: &str = "shutdown";
            if s.eq(COMMAND_STRING) {
                return Ok(Self::PowerOff);
            }
        }
        {
            const COMMAND_STRING: &str = "clkreboot";
            if s.eq(COMMAND_STRING) {
                return Ok(Self::ResetClockAndReboot);
            }
        }
        {
            const COMMAND_STRING: &str = "clock sync";
            if s.eq(COMMAND_STRING) {
                return Ok(Self::ClockSync);
            }
        }
        {
            const COMMAND_STRING: &str = "clock";
            if s.eq(COMMAND_STRING) {
                return Ok(Self::ShowClock);
            }
        }
        {
            const COMMAND_STRING: &str = "time ";
            if s.starts_with(COMMAND_STRING) {
                let values = scan!(s[COMMAND_STRING.len()..], u32);
                if let Some(time) = values.0 {
                    return Ok(Self::SetClock(time));
                } else {
                    return Err("'{epoch_string}' must be an integer epoch time");
                }
            }
        }
        {
            const COMMAND_STRING: &str = "advert";
            if s.eq(COMMAND_STRING) {
                return Ok(Self::SendFloodAdvert);
            }
        }
        {
            const COMMAND_STRING: &str = "advert.zerohop";
            if s.eq(COMMAND_STRING) {
                return Ok(Self::SendZeroHopAdvert);
            }
        }
        {
            const COMMAND_STRING: &str = "start ota";
            if s.eq(COMMAND_STRING) {
                return Ok(Self::StartOta);
            }
        }
        {
            const COMMAND_STRING: &str = "erase";
            if s.eq(COMMAND_STRING) {
                return Ok(Self::FactoryReset);
            }
        }
        {
            const COMMAND_STRING: &str = "neighbors";
            if s.eq(COMMAND_STRING) {
                return Ok(Self::ShowLastAdverts);
            }
        }
        {
            const COMMAND_STRING: &str = "neighbors.remove ";
            if s.starts_with(COMMAND_STRING) {
                let neighbor_string = &s[COMMAND_STRING.len()..];
                return Ok(Self::RemoveNeighbor(neighbor_string));
            }
        }
        {
            const COMMAND_STRING: &str = "discover.neighbors";
            if s.eq(COMMAND_STRING) {
                return Ok(Self::DiscoverZeroHopNeighbors);
            }
        }
        {
            const COMMAND_STRING: &str = "clear stats";
            if s.eq(COMMAND_STRING) {
                return Ok(Self::ClearStatistics);
            }
        }
        {
            const COMMAND_STRING: &str = "stats-core";
            if s.eq(COMMAND_STRING) {
                return Ok(Self::ShowCoreStats);
            }
        }
        {
            const COMMAND_STRING: &str = "stats-radio";
            if s.eq(COMMAND_STRING) {
                return Ok(Self::ShowRadioStats);
            }
        }
        {
            const COMMAND_STRING: &str = "stats-packet";
            if s.eq(COMMAND_STRING) {
                return Ok(Self::ShowPacketStats);
            }
        }
        {
            const COMMAND_STRING: &str = "log start";
            if s.eq(COMMAND_STRING) {
                return Ok(Self::StartRxLog);
            }
        }
        {
            const COMMAND_STRING: &str = "log stop";
            if s.eq(COMMAND_STRING) {
                return Ok(Self::StopRxLog);
            }
        }
        {
            const COMMAND_STRING: &str = "log erase";
            if s.eq(COMMAND_STRING) {
                return Ok(Self::EraseRxLog);
            }
        }
        {
            const COMMAND_STRING: &str = "log";
            if s.eq(COMMAND_STRING) {
                return Ok(Self::ShowRxLog);
            }
        }
        {
            const COMMAND_STRING: &str = "ver";
            if s.eq(COMMAND_STRING) {
                return Ok(Self::ShowVersion);
            }
        }
        {
            const COMMAND_STRING: &str = "board";
            if s.eq(COMMAND_STRING) {
                return Ok(Self::ShowHardwareName);
            }
        }
        {
            const COMMAND_STRING: &str = "get radio";
            if s.eq(COMMAND_STRING) {
                return Ok(Self::ShowRadioConfig);
            }
        }
        {
            const COMMAND_STRING: &str = "set radio ";
            if s.starts_with(COMMAND_STRING) {
                let values = scan!(s[COMMAND_STRING.len()..], f32, f32, u8, u8);
                if let Some(freq) = values.0 {
                    if let Some(bw) = values.1 {
                        if let Some(sf) = values.2 {
                            if let Some(cr) = values.3 {
                                return Ok(Self::SetRadioConfig { freq, bw, sf, cr });
                            } else {
                                return Err("failed to parse <cr>");
                            }
                        } else {
                            return Err("failed to parse <sf>");
                        }
                    } else {
                        return Err("failed to parse <bw> (should be in decimal Khz)");
                    }
                } else {
                    return Err("failed to parse <freq> (should be in decimal Mhz)");
                }
            }
        }
        {
            const COMMAND_STRING: &str = "get tx";
            if s.eq(COMMAND_STRING) {
                return Ok(Self::ShowTxPower);
            }
        }
        {
            const COMMAND_STRING: &str = "set tx ";
            if s.starts_with(COMMAND_STRING) {
                let values = scan!(s[COMMAND_STRING.len()..], i8);
                if let Some(tx_power) = values.0 {
                    return Ok(Self::SetTxPower(tx_power));
                } else {
                    return Err("failed to parse tx power, should be an integer");
                }
            }
        }
        {
            const COMMAND_STRING: &str = "tempradio ";
            if s.starts_with(COMMAND_STRING) {
                let values = scan!(s[COMMAND_STRING.len()..], f32, f32, u8, u8, u8);
                if let Some(freq) = values.0 {
                    if let Some(bw) = values.1 {
                        if let Some(sf) = values.2 {
                            if let Some(cr) = values.3 {
                                if let Some(duration_minutes) = values.4 {
                                    return Ok(Self::SetTempRadioConfig { freq, bw, sf, cr, duration_minutes })
                                }
                                else {
                                    return Err("failed to parse <timeout> (should be integer minutes)");
                                }
                            } else {
                                return Err("failed to parse <cr>");
                            }
                        } else {
                            return Err("failed to parse <sf>");
                        }
                    } else {
                        return Err("failed to parse <bw> (should be in decimal Khz)");
                    }
                } else {
                    return Err("failed to parse <freq> (should be in decimal Mhz)");
                }
            }
        }
        {
            const COMMAND_STRING: &str = "get freq";
            if s.eq(COMMAND_STRING) {
                return Ok(Self::ShowFreq);
            }
        }
        {
            const COMMAND_STRING: &str = "set freq ";
            if s.starts_with(COMMAND_STRING) {
                let values = scan!(s[COMMAND_STRING.len()..], f32);
                if let Some(freq) = values.0 {
                    return Ok(Self::SetFreq(freq));
                } else {
                    return Err("'{epoch_string}' must be an integer epoch time");
                }
            }
        }
        {
            const COMMAND_STRING: &str = "get radio.rxgain";
            if s.eq(COMMAND_STRING) {
                return Ok(Self::ShowRxGainStatus);
            }
        }
        {
            const COMMAND_STRING: &str = "set radio.rxgain ";
            if s.starts_with(COMMAND_STRING) {
                let setting_str = &s[COMMAND_STRING.len()..];
                if setting_str.eq("on") {
                    return Ok(Self::SetRxGain(true))
                }
                if setting_str.eq("off") {
                    return Ok(Self::SetRxGain(false))
                }
                return Err("faild to parse value - should be either 'on' or 'off' ")
            }
        }







        Err("unknown command {s}")
    }
}

#[derive(Debug, PartialEq)]
pub enum LoopDetection {
    Off,
    Minimal,
    Moderate,
    Strict,
}

#[derive(Debug, PartialEq)]
pub enum PermissionLevel {
    Guest,
    ReadOnly,
    Admin,
}

// TESTING
//--------------------------------------------------------------------------------
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn cli_commands() {
        {
            const COMMAND_STR: &str = "reboot";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::Reboot, command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "poweroff";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::PowerOff, command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "shutdown";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::PowerOff, command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "clkreboot";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::ResetClockAndReboot, command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "clock sync";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::ClockSync, command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "clock";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::ShowClock, command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const EPOCH_TIME: u32 = 1781660399;
            const COMMAND_STR: &str = "time 1781660399";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::SetClock(EPOCH_TIME), command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "advert";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::SendFloodAdvert, command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "advert.zerohop";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::SendZeroHopAdvert, command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "start ota";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::StartOta, command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "erase";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::FactoryReset, command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "neighbors";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::ShowLastAdverts, command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const NEIGHBOR_STRING: &str = "03";
            const COMMAND_STR: &str = "neighbors.remove 03";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::RemoveNeighbor(NEIGHBOR_STRING), command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "discover.neighbors";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::DiscoverZeroHopNeighbors, command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "clear stats";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::ClearStatistics, command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "stats-core";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::ShowCoreStats, command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "stats-radio";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::ShowRadioStats, command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "stats-packet";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::ShowPacketStats, command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "log start";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::StartRxLog, command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "log stop";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::StopRxLog, command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "log erase";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::EraseRxLog, command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "log";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::ShowRxLog, command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "ver";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::ShowVersion, command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "board";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::ShowHardwareName, command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "get radio";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::ShowRadioConfig, command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const FREQ: f32 = 869.525;
            const BW: f32 = 7.8;
            const SF: u8 = 5;
            const CR: u8 = 8;
            const COMMAND_STR: &str = "set radio 869.525,7.8,5,8,100";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(
                    CliCommands::SetRadioConfig {
                        freq: FREQ,
                        bw: BW,
                        sf: SF,
                        cr: CR
                    },
                    command
                );
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "get tx";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::ShowTxPower, command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const TX_POWER: i8 = 20;
            const COMMAND_STR: &str = "set tx 20";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::SetTxPower(TX_POWER), command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const FREQ: f32 = 869.525;
            const BW: f32 = 7.8;
            const SF: u8 = 5;
            const CR: u8 = 8;
            const TIMEOUT: u8 = 100;
            const COMMAND_STR: &str = "tempradio 869.525,7.8,5,8,100";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(
                    CliCommands::SetTempRadioConfig {
                        freq: FREQ,
                        bw: BW,
                        sf: SF,
                        cr: CR,
                        duration_minutes: TIMEOUT
                    },
                    command
                );
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "get freq";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::ShowFreq, command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const FREQ: f32 = 869.525;
            const COMMAND_STR: &str = "set freq 869.525";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::SetFreq(FREQ), command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "get radio.rxgain";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::ShowRxGainStatus, command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const RX_GAIN: bool = true;
            const COMMAND_STR: &str = "set radio.rxgain on";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::SetRxGain(RX_GAIN), command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }






    }
}
