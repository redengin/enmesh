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
    GetCoreStats,
    /// "stats-radio" - show noise floor, last rssi/snr, airtime, receive errors
    GetRadioStats,
    /// "stats-packet" - show packet counters (received, sent)
    GetPacketStats,

    /// "log start" - begin logging rx
    StartRxLog,
    /// "log stop" - stop logging rx
    StopRxLog,
    /// "log erase" - erase the logged data
    EraseRxLog,
    /// "log" - show the logged data
    GetRxLog,

    /// "ver" - show version
    GetVersion,
    /// "board" - show hardware name
    GetHardwareName,

    /// "get radio" - show radio config (<freq>,<bw>,<sf>,<cr>)
    GetRadioConfig,
    /// "set radio <freq>,<bw>,<sf>,<cr>" - set the radio config
    SetRadioConfig { freq: f32, bw: f32, sf: u8, cr: u8 },
    /// "get tx" - show tx power (integer dBm)
    GetTxPower,
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
    GetFreq,
    /// "set freq <frequency>" - set the radio frequency (in MHz)
    SetFreq(f32),
    /// "get radio.rxgain" - show if rxgain is enabled
    GetRxGainStatus,
    /// "set radio.rxgain <state>" - "on": enable rx gain
    ///                              "off": disable rx gain
    SetRxGain(bool),

    /// "get name" - show the name of this device
    GetName,
    /// "set name <name>" - set the name of this device
    SetName(&'a str),
    /// "get lat" - show the latitude of the device
    GetLat,
    /// "set lat <degrees>" - set the longitude
    SetLat(f32),
    /// "get lon" - show the longitude of the device
    GetLon,
    /// "set lon <degrees>" - set the longitude
    SetLon(f32),
    /// "get prv.key" - show the private key
    GetPrivateKey,
    /// "set prv.key <private key>" - set the private key
    SetPrivateKey(&'a str),
    /// "password <new password>" - change the admin password
    SetAdminPassword(&'a str),
    /// "get guest.password" - show the guest password
    GetGuestPassword,
    /// "set guest.password <password>" - set the guest password
    SetGuestPassword(&'a str),
    /// "get owner.info" - show the owner information (text where '|' treated as newline)
    GetOwnerInfo,
    /// "set owner.info <text>" - set the owner information
    SetOwnerInfo(&'a str),
    /// "get adc.multiplier" - show the battery ADC scaling
    GetBatteryGain,
    /// "set adc.multiplier <value>" - set the battery ADC scaling
    SetBatteryGain(f32),
    /// "get public.key" - show the public key
    GetPublicKey,
    /// "get role" - show this node's role
    GetRole,
    /// "powersaving" - show if power saving is enabled
    GetPowerSavingState,
    /// "powersaving <value>" - "on":enable power saving, "off": disable power saving
    SetPowerSavingState(bool),

    /// "get repeat" - show if repeating is enabled
    GetRepeatState,
    /// "set repeat <state>" - "on": enable / "off": disable
    SetRepeatState(bool),
    /// "get path.hash.mode" - show advert path hash-size
    GetHashSize,
    /// "set path.hash.mode <value>" - 0: 1 byte hash, 1: 2 byte hash, 2: 3 byte hash
    SetHashSize(MeshCoreHashSize),
    /// "get loop.detect" - show if loop-detection enabled
    GetLoopDetectState,
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
    GetReceiveDelay,
    /// "set rxdelay <value>" - [0.0 .. 20.0]
    SetReceiveDelay(f32),
    /// "get dutycycle" - show duty cycle
    GetDutyCycle,
    /// "set dutycycle <value>" - [0 .. 100]
    SetDutyCycle(u8),
    /// "get af" - show airtime factor
    GetAirtimeFactor,
    /// "set af <value>" - [0.0 .. 9.0]
    SetAirtimeFactor(f32),
    /// "get int.thresh" - show the interference threshold
    GetInterferenceThreshold,
    /// "set int.thresh <value>"
    SetInterferenceThreshold(f32),
    /// "get agc.reset.interval" - show AGC reset interval
    GetAgcResetInterval,
    /// "set agc.reset.interval <value>" - multiple of 4 seconds (rounds down)
    SetAgcResetInterval(u8),
    /// "get multi.acks" - show if Multi-Acks enabled
    GetMultiAcksEnabled,
    /// "set multi.acks <state>" - 0: disable, 1: enable
    SetMultiAcksEnabled(bool),
    /// "get advert.interval" - show the advertisement interval
    GetAdvertInterval,
    /// "set advert.interval <minutes>" - multiple of 2 seconds (rounds down) [60 .. 240]
    SetAdvertInterval(u8),
    /// "get flood.max.unscoped" - show max hop count for unscoped packets
    GetUnscopedMaxHopCount,
    /// "set flood.max.unscoped <value>" - [0 .. 64]
    SetUnscopedMaxHopCount(u8),
    /// "get flood.max.advert" - show max hop count for flood advert
    GetFloodAdvertMaxHopCount,
    /// "set flood.max.advert <value>" - [0 .. 64]
    SetFloodAdvertMaxHopCount(u8),

    /// "setperm <pubkey> <permissions>"
    SetAclPermissions {
        pubkey: &'a str,
        permissions: PermissionLevel,
    },
    /// "get acl" - show the ACL
    GetAcl,
    /// "get allow.read.only" - show if this room is read-only
    GetRoomAccess,
    /// "set allow.read.only <state>" - "on": read-only, "off" - read-write
    SetRoomAccess(RoomAccess),

    /// "region load <name> [flood_flag]" - name: "*" represents wildcard region
    ///                                     (optional)flood_flag: "F" to allow flooding
    LoadRegionSettings { name: &'a str, allow_flood: bool },
    /// "region save" - save changes to region
    SaveRegionSettings,
    /// "region allowf <name>" - allow forwarding for region, name: "*" represents wildcard region
    AllowRegionForwarding { name: &'a str },
    /// "region denyf <name>" - deny forwarding for region, name: "*" represents wildcard region
    DenyRegionForwarding { name: &'a str },
    /// "region get <name>" - show information for region
    GetRegion { name: &'a str },
    /// "region home" - show home region of this node
    GetHomeRegion,
    /// "region home <name>" - name: <null> to remove the region
    SetHomeRegion { name: Option<&'a str> },
    /// "region default" - show default scope region for this node
    GetDefaultRegion,
    /// "region default {name|<null>}" - set the default region
    SetDefaultRegion { name: Option<&'a str> },
    /// "region put <name> [parent_name]" - create a new region
    CreateRegion {
        name: &'a str,
        parent_name: Option<&'a str>,
    },
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
    GetRegionList { filter: &'a str },

    /// "gps" - show if GPS is enabled
    GetGps,
    /// "gps <state>" - <state> "on": enable GPS, "off": disable GPS
    SetGpsEnabled(bool),
    /// "gps sync" - sync time with GPS
    SyncGpsTime,
    /// "gps setloc" - use GPS to set the location
    SyncGpsLocation,
    /// "gps advert" - show the GPS advert policy
    GetGpsAdvertPolicy,
    /// "gps advert <policy>" - set the GPS advert policy
    SetGpsAdvertPolicy(&'a str),

    /// "sensor list [start]" - show sensors, optionally start at [start] index
    GetSensors { start_index: u8 },
    /// "sensor get <key>" - show the value of a sensor
    GetSensor { key: &'a str },
    /// "sensor set <key> <value>" - set the value of a sensor
    SetSensor { key: &'a str, value: f32 },

    /// "get bridge.type" - show the bridging mode
    GetBridgeType,
    /// "get bridge.enabled" - show whether bridging is enabled
    GetBridgeEnabled,
    /// "set bridge.enabled <state>" - <state> "on": enabled, "off": disabled
    SetBridgeEnabled(bool),
    /// "get bridge.delay" - show the bridge delay
    GetBridgeDelay,
    /// "set bridge.delay <ms>" - set the bridge delay in ms
    SetBridgeDelay(u16),
    /// "get bridge.source" - show the number of pakcets on the bridge
    GetBridgeSource,
    /// "set bridge.source <source>" - //TODO wtf is this
    SetBridgeSource(&'a str),
    /// "get bridge.baud" - show the baudrate supported by the bridge
    GetBridgeBaud,
    /// "set bridge.baud <rate>" - rate: [9600, 19200, 38400, 57600, 115200]
    SetBridgeBaud(u32),
    /// "get bridge.channel" - show the channel for the bridge
    GetBridgeChannel,
    /// "set bridge.channel <channel>" - channel: [1 .. 14]
    SetBridgeChannel(u8),
    /// "get bridge.secret" - show the ESP-NOW secret
    GetBridgeSecret,
    /// "set bridge.secret <secret>" - set the ESP-NOW secret
    SetBridgeSecret(&'a str),
    /// "get bootloader.ver" - show the NRF52 bootloader version
    GetBootLoaderVersion,
    /// "get pwrmgt.support" - show the power management support
    GetPowerManagementSupport,
    /// "get pwrmgt.source" - show the power source
    GetPowerSource,
    /// "get pwrmgmt.bootreason"
    GetBootReason,
    /// "get pwrmgt.bootmv" - show the boot voltage
    GetBootVoltage,
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
                return Ok(Self::GetCoreStats);
            }
        }
        {
            const COMMAND_STRING: &str = "stats-radio";
            if s.eq(COMMAND_STRING) {
                return Ok(Self::GetRadioStats);
            }
        }
        {
            const COMMAND_STRING: &str = "stats-packet";
            if s.eq(COMMAND_STRING) {
                return Ok(Self::GetPacketStats);
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
                return Ok(Self::GetRxLog);
            }
        }
        {
            const COMMAND_STRING: &str = "ver";
            if s.eq(COMMAND_STRING) {
                return Ok(Self::GetVersion);
            }
        }
        {
            const COMMAND_STRING: &str = "board";
            if s.eq(COMMAND_STRING) {
                return Ok(Self::GetHardwareName);
            }
        }
        {
            const COMMAND_STRING: &str = "get radio";
            if s.eq(COMMAND_STRING) {
                return Ok(Self::GetRadioConfig);
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
                return Ok(Self::GetTxPower);
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
                                    return Ok(Self::SetTempRadioConfig {
                                        freq,
                                        bw,
                                        sf,
                                        cr,
                                        duration_minutes,
                                    });
                                } else {
                                    return Err(
                                        "failed to parse <timeout> (should be integer minutes)",
                                    );
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
                return Ok(Self::GetFreq);
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
                return Ok(Self::GetRxGainStatus);
            }
        }
        {
            const COMMAND_STRING: &str = "set radio.rxgain ";
            if s.starts_with(COMMAND_STRING) {
                let setting_str = &s[COMMAND_STRING.len()..];
                if setting_str.eq("on") {
                    return Ok(Self::SetRxGain(true));
                }
                if setting_str.eq("off") {
                    return Ok(Self::SetRxGain(false));
                }
                return Err("failed to parse value - should be either 'on' or 'off' ");
            }
        }
        {
            const COMMAND_STRING: &str = "get name";
            if s.eq(COMMAND_STRING) {
                return Ok(Self::GetName);
            }
        }
        {
            const COMMAND_STRING: &str = "set name ";
            if s.starts_with(COMMAND_STRING) {
                let name = &s[COMMAND_STRING.len()..];
                if name.len() > 0 {
                    return Ok(Self::SetName(name));
                } else {
                    return Err("<name> no provided");
                }
            }
        }
        {
            const COMMAND_STRING: &str = "get lat";
            if s.eq(COMMAND_STRING) {
                return Ok(Self::GetLat);
            }
        }
        {
            const COMMAND_STRING: &str = "set lat ";
            if s.starts_with(COMMAND_STRING) {
                let values = scan!(s[COMMAND_STRING.len()..], f32);
                if let Some(lat) = values.0 {
                    return Ok(Self::SetLat(lat));
                } else {
                    return Err("<lat> must be a decimal lattitude");
                }
            }
        }
        {
            const COMMAND_STRING: &str = "get lon";
            if s.eq(COMMAND_STRING) {
                return Ok(Self::GetLon);
            }
        }
        {
            const COMMAND_STRING: &str = "set lon ";
            if s.starts_with(COMMAND_STRING) {
                let values = scan!(s[COMMAND_STRING.len()..], f32);
                if let Some(lon) = values.0 {
                    return Ok(Self::SetLon(lon));
                } else {
                    return Err("<lon> must be a decimal longitude");
                }
            }
        }
        {
            const COMMAND_STRING: &str = "get prv.key";
            if s.eq(COMMAND_STRING) {
                return Ok(Self::GetPrivateKey);
            }
        }
        {
            const COMMAND_STRING: &str = "set prv.key ";
            if s.starts_with(COMMAND_STRING) {
                let prv_key = &s[COMMAND_STRING.len()..];
                if prv_key.len() > 0 {
                    return Ok(Self::SetPrivateKey(prv_key));
                }
            }
        }
        {
            const COMMAND_STRING: &str = "password ";
            if s.starts_with(COMMAND_STRING) {
                let password = &s[COMMAND_STRING.len()..];
                if password.len() > 0 {
                    return Ok(Self::SetAdminPassword(password));
                }
            }
        }
        {
            const COMMAND_STRING: &str = "get guest.password";
            if s.eq(COMMAND_STRING) {
                return Ok(Self::GetGuestPassword);
            }
        }
        {
            const COMMAND_STRING: &str = "set guest.password ";
            if s.starts_with(COMMAND_STRING) {
                let password = &s[COMMAND_STRING.len()..];
                if password.len() > 0 {
                    return Ok(Self::SetGuestPassword(password));
                }
            }
        }
        {
            const COMMAND_STRING: &str = "get owner.info";
            if s.eq(COMMAND_STRING) {
                return Ok(Self::GetOwnerInfo);
            }
        }
        {
            const COMMAND_STRING: &str = "set owner.info ";
            if s.starts_with(COMMAND_STRING) {
                let info = &s[COMMAND_STRING.len()..];
                return Ok(Self::SetOwnerInfo(info));
            }
        }
        {
            const COMMAND_STRING: &str = "get adc.multiplier";
            if s.eq(COMMAND_STRING) {
                return Ok(Self::GetBatteryGain);
            }
        }
        {
            const COMMAND_STRING: &str = "set adc.multiplier ";
            if s.starts_with(COMMAND_STRING) {
                let values = scan!(s[COMMAND_STRING.len()..], f32);
                if let Some(gain) = values.0 {
                    return Ok(Self::SetBatteryGain(gain));
                } else {
                    return Err("<gain> must be a decimal");
                }
            }
        }
        {
            const COMMAND_STRING: &str = "get public.key";
            if s.eq(COMMAND_STRING) {
                return Ok(Self::GetPublicKey);
            }
        }
        {
            const COMMAND_STRING: &str = "get role";
            if s.eq(COMMAND_STRING) {
                return Ok(Self::GetRole);
            }
        }
        {
            const COMMAND_STRING: &str = "powersaving";
            if s.eq(COMMAND_STRING) {
                return Ok(Self::GetPowerSavingState);
            }
        }
        {
            const COMMAND_STRING: &str = "powersaving ";
            if s.starts_with(COMMAND_STRING) {
                let setting_str = &s[COMMAND_STRING.len()..];
                if setting_str.eq("on") {
                    return Ok(Self::SetPowerSavingState(true));
                }
                if setting_str.eq("off") {
                    return Ok(Self::SetPowerSavingState(false));
                }
                return Err("failed to parse value - should be either 'on' or 'off' ");
            }
        }
        {
            const COMMAND_STRING: &str = "get repeat";
            if s.eq(COMMAND_STRING) {
                return Ok(Self::GetRepeatState);
            }
        }
        {
            const COMMAND_STRING: &str = "set repeat ";
            if s.starts_with(COMMAND_STRING) {
                let setting_str = &s[COMMAND_STRING.len()..];
                if setting_str.eq("on") {
                    return Ok(Self::SetRepeatState(true));
                }
                if setting_str.eq("off") {
                    return Ok(Self::SetRepeatState(false));
                }
                return Err("failed to parse value - should be either 'on' or 'off' ");
            }
        }
        {
            const COMMAND_STRING: &str = "get path.hash.mode";
            if s.eq(COMMAND_STRING) {
                return Ok(Self::GetHashSize);
            }
        }
        {
            const COMMAND_STRING: &str = "set path.hash.mode ";
            if s.starts_with(COMMAND_STRING) {
                let values = scan!(s[COMMAND_STRING.len()..], u8);
                if let Some(value) = &values.0 {
                    if let Some(hash_size) = MeshCoreHashSize::from_byte(value) {
                        return Ok(Self::SetHashSize(hash_size));
                    } else {
                        return Err("<value> must be a decimal [0..2]");
                    }
                }
            }
        }
        {
            const COMMAND_STRING: &str = "get loop.detect";
            if s.eq(COMMAND_STRING) {
                return Ok(Self::GetLoopDetectState);
            }
        }
        {
            const COMMAND_STRING: &str = "set loop.detect ";
            if s.starts_with(COMMAND_STRING) {
                let setting_str = &s[COMMAND_STRING.len()..];
                return match setting_str {
                    "off" => Ok(Self::SetLoopDetection(LoopDetection::Off)),
                    "minimal" => Ok(Self::SetLoopDetection(LoopDetection::Minimal)),
                    "moderate" => Ok(Self::SetLoopDetection(LoopDetection::Moderate)),
                    "strict" => Ok(Self::SetLoopDetection(LoopDetection::Strict)),
                    _ => Err("<value> should be one of [off, minimal, moderate, strict]"),
                };
            }
        }
        {
            const COMMAND_STRING: &str = "get txdelay";
            if s.eq(COMMAND_STRING) {
                return Ok(Self::ShowTransmitDelay);
            }
        }
        {
            const COMMAND_STRING: &str = "set txdelay ";
            if s.starts_with(COMMAND_STRING) {
                let values = scan!(s[COMMAND_STRING.len()..], f32);
                if let Some(tx_delay) = values.0 {
                    return Ok(Self::SetTransmitDelay(tx_delay));
                } else {
                    return Err("<tx delay> must be a decimal");
                }
            }
        }
        {
            const COMMAND_STRING: &str = "get rxdelay";
            if s.eq(COMMAND_STRING) {
                return Ok(Self::GetReceiveDelay);
            }
        }
        {
            const COMMAND_STRING: &str = "set rxdelay ";
            if s.starts_with(COMMAND_STRING) {
                let values = scan!(s[COMMAND_STRING.len()..], f32);
                if let Some(rx_delay) = values.0 {
                    return Ok(Self::SetReceiveDelay(rx_delay));
                } else {
                    return Err("<rx delay> must be a decimal");
                }
            }
        }
        {
            const COMMAND_STRING: &str = "get dutycycle";
            if s.eq(COMMAND_STRING) {
                return Ok(Self::GetDutyCycle);
            }
        }
        {
            const COMMAND_STRING: &str = "set dutycycle ";
            if s.starts_with(COMMAND_STRING) {
                let values = scan!(s[COMMAND_STRING.len()..], u8);
                if let Some(duty_cycle) = values.0 {
                    return Ok(Self::SetDutyCycle(duty_cycle));
                } else {
                    return Err("<duty cycle> must be an integer [0..100]%");
                }
            }
        }
        {
            const COMMAND_STRING: &str = "get af";
            if s.eq(COMMAND_STRING) {
                return Ok(Self::GetAirtimeFactor);
            }
        }
        {
            const COMMAND_STRING: &str = "set af ";
            if s.starts_with(COMMAND_STRING) {
                let values = scan!(s[COMMAND_STRING.len()..], f32);
                if let Some(af) = values.0 {
                    return Ok(Self::SetAirtimeFactor(af));
                } else {
                    return Err("<af> must be a decimal");
                }
            }
        }
        {
            const COMMAND_STRING: &str = "get int.thresh";
            if s.eq(COMMAND_STRING) {
                return Ok(Self::GetInterferenceThreshold);
            }
        }
        {
            const COMMAND_STRING: &str = "set int.thresh ";
            if s.starts_with(COMMAND_STRING) {
                let values = scan!(s[COMMAND_STRING.len()..], f32);
                if let Some(int_thresh) = values.0 {
                    return Ok(Self::SetInterferenceThreshold(int_thresh));
                } else {
                    return Err("<int_thresh> must be a decimal");
                }
            }
        }
        {
            const COMMAND_STRING: &str = "get agc.reset.interval";
            if s.eq(COMMAND_STRING) {
                return Ok(Self::GetAgcResetInterval);
            }
        }
        {
            const COMMAND_STRING: &str = "set agc.reset.interval ";
            if s.starts_with(COMMAND_STRING) {
                let values = scan!(s[COMMAND_STRING.len()..], u8);
                if let Some(agc_reset_interval) = values.0 {
                    return Ok(Self::SetAgcResetInterval(agc_reset_interval));
                } else {
                    return Err("<interval> must be an integer");
                }
            }
        }
        {
            const COMMAND_STRING: &str = "get multi.acks";
            if s.eq(COMMAND_STRING) {
                return Ok(Self::GetMultiAcksEnabled);
            }
        }
        {
            const COMMAND_STRING: &str = "set multi.acks ";
            if s.starts_with(COMMAND_STRING) {
                let values = scan!(s[COMMAND_STRING.len()..], u8);
                if let Some(mode) = values.0 {
                    match mode {
                        0 => return Ok(Self::SetMultiAcksEnabled(false)),
                        1 => return Ok(Self::SetMultiAcksEnabled(true)),
                        _ => return Err("unsupported multi.acks mode '{mode}'"),
                    }
                } else {
                    return Err("<mode> must be an integer");
                }
            }
        }
        {
            const COMMAND_STRING: &str = "get advert.interval";
            if s.eq(COMMAND_STRING) {
                return Ok(Self::GetAdvertInterval);
            }
        }
        {
            const COMMAND_STRING: &str = "set advert.interval ";
            if s.starts_with(COMMAND_STRING) {
                let values = scan!(s[COMMAND_STRING.len()..], u8);
                if let Some(interval) = values.0 {
                    return Ok(Self::SetAdvertInterval(interval));
                } else {
                    return Err("<interval> must be an integer");
                }
            }
        }
        {
            const COMMAND_STRING: &str = "get flood.max.unscoped";
            if s.eq(COMMAND_STRING) {
                return Ok(Self::GetUnscopedMaxHopCount);
            }
        }
        {
            const COMMAND_STRING: &str = "set flood.max.unscoped ";
            if s.starts_with(COMMAND_STRING) {
                let values = scan!(s[COMMAND_STRING.len()..], u8);
                if let Some(hops) = values.0 {
                    return Ok(Self::SetUnscopedMaxHopCount(hops));
                } else {
                    return Err("<interval> must be an integer");
                }
            }
        }
        {
            const COMMAND_STRING: &str = "get flood.max.advert";
            if s.eq(COMMAND_STRING) {
                return Ok(Self::GetFloodAdvertMaxHopCount);
            }
        }
        {
            const COMMAND_STRING: &str = "set flood.max.advert ";
            if s.starts_with(COMMAND_STRING) {
                let values = scan!(s[COMMAND_STRING.len()..], u8);
                if let Some(hops) = values.0 {
                    return Ok(Self::SetFloodAdvertMaxHopCount(hops));
                } else {
                    return Err("<interval> must be an integer");
                }
            }
        }
        {
            const COMMAND_STRING: &str = "setperm ";
            if s.starts_with(COMMAND_STRING) {
                let mut used = COMMAND_STRING.len();
                if let Some(pubkey_end) = s[used..].find(' ') {
                    let pubkey = &s[used..(used + pubkey_end)];
                    used += pubkey_end + 1;

                    let values = scan!(s[used..], u8);
                    if let Some(level_value) = values.0 {
                        if let Some(permissions) = PermissionLevel::from_byte(level_value) {
                            return Ok(Self::SetAclPermissions {
                                pubkey,
                                permissions,
                            });
                        } else {
                            return Err("unsupported level: {level_value}");
                        }
                    }
                }
            }
        }
        {
            const COMMAND_STRING: &str = "get acl";
            if s.eq(COMMAND_STRING) {
                return Ok(Self::GetAcl);
            }
        }
        {
            const COMMAND_STRING: &str = "get allow.read.only";
            if s.eq(COMMAND_STRING) {
                return Ok(Self::GetRoomAccess);
            }
        }
        {
            const COMMAND_STRING: &str = "set allow.read.only ";
            if s.starts_with(COMMAND_STRING) {
                let setting_str = &s[COMMAND_STRING.len()..];
                if setting_str.eq("on") {
                    return Ok(Self::SetRoomAccess(RoomAccess::ReadOnly));
                }
                if setting_str.eq("off") {
                    return Ok(Self::SetRoomAccess(RoomAccess::ReadWrite));
                }
                return Err("failed to parse value - should be either 'on' or 'off' ");
            }
        }
        {
            const COMMAND_STRING: &str = "region load ";
            if s.starts_with(COMMAND_STRING) {
                let mut used = COMMAND_STRING.len();
                if let Some(name_end) = s[used..].find(' ') {
                    let name = &s[used..(used + name_end)];
                    used += name_end + 1;

                    let values = scan!(s[used..], char);
                    if let Some(flood_flag) = values.0 {
                        match flood_flag {
                            'F' => {
                                return Ok(Self::LoadRegionSettings {
                                    name,
                                    allow_flood: true,
                                });
                            }
                            _ => return Err("flood flag unknown, expected 'F'"),
                        }
                    }
                } else {
                    let name = &s[used..];
                    return Ok(Self::LoadRegionSettings {
                        name,
                        allow_flood: false,
                    });
                }
            }
        }
        {
            const COMMAND_STRING: &str = "region save";
            if s.eq(COMMAND_STRING) {
                return Ok(Self::SaveRegionSettings);
            }
        }
        {
            const COMMAND_STRING: &str = "region allowf ";
            if s.starts_with(COMMAND_STRING) {
                let name = &s[COMMAND_STRING.len()..];
                if name.len() > 0 {
                    return Ok(Self::AllowRegionForwarding { name });
                }
            }
        }
        {
            const COMMAND_STRING: &str = "region denyf ";
            if s.starts_with(COMMAND_STRING) {
                let name = &s[COMMAND_STRING.len()..];
                if name.len() > 0 {
                    return Ok(Self::DenyRegionForwarding { name });
                }
            }
        }
        {
            const COMMAND_STRING: &str = "region get ";
            if s.starts_with(COMMAND_STRING) {
                let name = &s[COMMAND_STRING.len()..];
                if name.len() > 0 {
                    return Ok(Self::GetRegion { name });
                }
            }
        }
        {
            const COMMAND_STRING: &str = "region home";
            if s.starts_with(COMMAND_STRING) {
                if s.len() > COMMAND_STRING.len() {
                    let name = &s[COMMAND_STRING.len() + 1..];
                    if name.len() > 0 {
                        return Ok(Self::SetHomeRegion { name: Some(name) });
                    } else {
                        return Ok(Self::SetHomeRegion { name: None });
                    }
                }
                return Ok(Self::GetHomeRegion);
            }
        }
        {
            const COMMAND_STRING: &str = "region default";
            if s.starts_with(COMMAND_STRING) {
                if s.len() > COMMAND_STRING.len() {
                    let name = &s[COMMAND_STRING.len() + 1..];
                    if name.len() > 0 {
                        return match name {
                            "<null>" => Ok(Self::SetDefaultRegion { name: None }),
                            _ => Ok(Self::SetDefaultRegion { name: Some(name) }),
                        };
                    } else {
                        return Ok(Self::SetDefaultRegion { name: None });
                    }
                }
                return Ok(Self::GetDefaultRegion);
            }
        }
        {
            const COMMAND_STRING: &str = "region put ";
            if s.starts_with(COMMAND_STRING) {
                let mut used = COMMAND_STRING.len();
                if let Some(name_end) = s[used..].find(' ') {
                    let name = &s[used..(used + name_end)];
                    used += name_end + 1;

                    let parent_name = &s[used..];
                    if parent_name.len() > 0 {
                        return Ok(Self::CreateRegion {
                            name,
                            parent_name: Some(parent_name),
                        });
                    } else {
                        return Ok(Self::CreateRegion {
                            name,
                            parent_name: None,
                        });
                    }
                } else {
                    let name = &s[used..];
                    return Ok(Self::CreateRegion {
                        name,
                        parent_name: None,
                    });
                }
            }
        }
        // FIXME
        // {
        //     const COMMAND_STRING: &str = "region def ";
        //     if s.starts_with(COMMAND_STRING) {
        //         let mut used = COMMAND_STRING.len();
        //         if let Some(name_end) = s[used..].find(' ') {
        //             let name = &s[used..(used + name_end)];
        //             used += name_end + 1;

        //         }
        //         else {
        //             let name = &s[used..];
        //             return Ok(Self::CreateRegion { name, parent_name: None })
        //         }
        //     }
        // }
        {
            const COMMAND_STRING: &str = "region remove ";
            if s.starts_with(COMMAND_STRING) {
                let name = &s[COMMAND_STRING.len()..];
                if name.len() > 0 {
                    return Ok(Self::RemoveRegion { name });
                }
            }
        }
        {
            const COMMAND_STRING: &str = "region list ";
            if s.starts_with(COMMAND_STRING) {
                let filter = &s[COMMAND_STRING.len()..];
                if filter.len() > 0 {
                    return Ok(Self::GetRegionList { filter });
                }
            }
        }
        {
            const COMMAND_STRING: &str = "gps";
            if s.eq(COMMAND_STRING) {
                return Ok(Self::GetGps);
            }
        }
        {
            const COMMAND_STRING: &str = "gps ";
            if s.starts_with(COMMAND_STRING) {
                let subcommand = &s[COMMAND_STRING.len()..];
                if subcommand == "on" {
                    return Ok(Self::SetGpsEnabled(true));
                };
                if subcommand == "off" {
                    return Ok(Self::SetGpsEnabled(false));
                };
                if subcommand == "sync" {
                    return Ok(Self::SyncGpsTime);
                };
                if subcommand == "setloc" {
                    return Ok(Self::SyncGpsLocation);
                };
                if subcommand == "advert" {
                    return Ok(Self::GetGpsAdvertPolicy);
                };
                if subcommand.starts_with("advert ") {
                    const SUBCOMMAND_LEN: usize = "advert ".len();
                    let policy = &subcommand[SUBCOMMAND_LEN..];
                    return Ok(Self::SetGpsAdvertPolicy(policy));
                }

                return Err("{subcommand} not recognized");
            };
        }
        {
            const COMMAND_STRING: &str = "sensor list";
            if s.starts_with(COMMAND_STRING) {
                return if COMMAND_STRING.len() < s.len() {
                    let used = COMMAND_STRING.len() + 1;
                    let values = scan!(s[used..], u8);
                    if let Some(start_index) = values.0 {
                        Ok(Self::GetSensors { start_index })
                    } else {
                        Err("<start_index> should be an integer")
                    }
                } else {
                    Ok(Self::GetSensors { start_index: 0 })
                };
            }
        }
        {
            const COMMAND_STRING: &str = "sensor get ";
            if s.starts_with(COMMAND_STRING) {
                let used = COMMAND_STRING.len();
                let key = &s[used..];
                return Ok(Self::GetSensor { key });
            }
        }
        {
            const COMMAND_STRING: &str = "sensor set ";
            if s.starts_with(COMMAND_STRING) {
                let mut used = COMMAND_STRING.len();
                if let Some(key_end) = s[used..].find(' ') {
                    let key = &s[used..(used + key_end)];
                    used += key_end + 1;
                    let values = scan!(s[used..], f32);
                    if let Some(value) = values.0 {
                        return Ok(Self::SetSensor { key, value });
                    }
                } else {
                    let key = &s[used..];
                    return Ok(Self::GetSensor { key });
                }
            }
        }
        {
            const COMMAND_STRING: &str = "get bridge.type";
            if s.eq(COMMAND_STRING) {
                return Ok(Self::GetBridgeType);
            }
        }
        {
            const COMMAND_STRING: &str = "get bridge.enabled";
            if s.eq(COMMAND_STRING) {
                return Ok(Self::GetBridgeEnabled);
            }
        }
        {
            const COMMAND_STRING: &str = "set bridge.enabled ";
            if s.starts_with(COMMAND_STRING) {
                let setting_str = &s[COMMAND_STRING.len()..];
                if setting_str.eq("on") {
                    return Ok(Self::SetBridgeEnabled(true));
                }
                if setting_str.eq("off") {
                    return Ok(Self::SetBridgeEnabled(false));
                }
                return Err("failed to parse value - should be either 'on' or 'off' ");
            }
        }
        {
            const COMMAND_STRING: &str = "get bridge.delay";
            if s.eq(COMMAND_STRING) {
                return Ok(Self::GetBridgeDelay);
            }
        }
        {
            const COMMAND_STRING: &str = "set bridge.delay ";
            if s.starts_with(COMMAND_STRING) {
                let used = COMMAND_STRING.len();
                let values = scan!(s[used..], u16);
                if let Some(delay) = values.0 {
                    return Ok(Self::SetBridgeDelay(delay));
                }
            }
        }
        {
            const COMMAND_STRING: &str = "get bridge.source";
            if s.eq(COMMAND_STRING) {
                return Ok(Self::GetBridgeSource);
            }
        }
        {
            const COMMAND_STRING: &str = "set bridge.source ";
            if s.starts_with(COMMAND_STRING) {
                let used = COMMAND_STRING.len();
                let source = &s[used..];
                return Ok(Self::SetBridgeSource(source));
            }
        }
        {
            const COMMAND_STRING: &str = "get bridge.baud";
            if s.eq(COMMAND_STRING) {
                return Ok(Self::GetBridgeBaud);
            }
        }
        {
            const COMMAND_STRING: &str = "set bridge.baud ";
            if s.starts_with(COMMAND_STRING) {
                let used = COMMAND_STRING.len();
                let values = scan!(s[used..], u32);
                if let Some(baud) = values.0 {
                    return Ok(Self::SetBridgeBaud(baud));
                }
            }
        }
        {
            const COMMAND_STRING: &str = "get bridge.channel";
            if s.eq(COMMAND_STRING) {
                return Ok(Self::GetBridgeChannel);
            }
        }
        {
            const COMMAND_STRING: &str = "set bridge.channel ";
            if s.starts_with(COMMAND_STRING) {
                let used = COMMAND_STRING.len();
                let values = scan!(s[used..], u8);
                if let Some(channel) = values.0 {
                    return Ok(Self::SetBridgeChannel(channel));
                }
            }
        }
        {
            const COMMAND_STRING: &str = "get bridge.secret";
            if s.eq(COMMAND_STRING) {
                return Ok(Self::GetBridgeSecret);
            }
        }
        {
            const COMMAND_STRING: &str = "set bridge.secret ";
            if s.starts_with(COMMAND_STRING) {
                let used = COMMAND_STRING.len();
                let secret = &s[used..];
                return Ok(Self::SetBridgeSecret(secret));
            }
        }
        {
            const COMMAND_STRING: &str = "get bootloader.ver";
            if s.eq(COMMAND_STRING) {
                return Ok(Self::GetBootLoaderVersion);
            }
        }
        {
            const COMMAND_STRING: &str = "get pwrmgt.source";
            if s.eq(COMMAND_STRING) {
                return Ok(Self::GetPowerSource);
            }
        }
        {
            const COMMAND_STRING: &str = "get pwrmgt.support";
            if s.eq(COMMAND_STRING) {
                return Ok(Self::GetPowerManagementSupport);
            }
        }
        {
            const COMMAND_STRING: &str = "get pwrmgt.bootreason";
            if s.eq(COMMAND_STRING) {
                return Ok(Self::GetBootReason);
            }
        }
        {
            const COMMAND_STRING: &str = "get pwrmgt.bootmv";
            if s.eq(COMMAND_STRING) {
                return Ok(Self::GetBootVoltage);
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
    ReadWrite,
    Admin,
}
impl PermissionLevel {
    fn from_byte(value: u8) -> Option<Self> {
        return match value {
            0 => Some(Self::Guest),
            1 => Some(Self::ReadOnly),
            2 => Some(Self::ReadWrite),
            3 => Some(Self::Admin),
            _ => None,
        };
    }
}

#[derive(Debug, PartialEq)]
pub enum RoomAccess {
    ReadOnly,
    ReadWrite,
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
                assert_eq!(CliCommands::GetCoreStats, command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "stats-radio";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::GetRadioStats, command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "stats-packet";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::GetPacketStats, command);
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
                assert_eq!(CliCommands::GetRxLog, command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "ver";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::GetVersion, command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "board";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::GetHardwareName, command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "get radio";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::GetRadioConfig, command);
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
                assert_eq!(CliCommands::GetTxPower, command);
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
                assert_eq!(CliCommands::GetFreq, command);
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
                assert_eq!(CliCommands::GetRxGainStatus, command);
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
        {
            const COMMAND_STR: &str = "get name";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::GetName, command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const NAME: &str = "test-name";
            const COMMAND_STR: &str = "set name test-name";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::SetName(NAME), command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "get lat";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::GetLat, command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const LAT: f32 = 31.5;
            const COMMAND_STR: &str = "set lat 31.5";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::SetLat(LAT), command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "get lon";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::GetLon, command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const LON: f32 = 31.5;
            const COMMAND_STR: &str = "set lon 31.5";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::SetLon(LON), command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "get prv.key";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::GetPrivateKey, command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const PRV_KEY: &str = "123456789";
            const COMMAND_STR: &str = "set prv.key 123456789";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::SetPrivateKey(PRV_KEY), command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const PASSWORD: &str = "secret";
            const COMMAND_STR: &str = "password secret";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::SetAdminPassword(PASSWORD), command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "get guest.password";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::GetGuestPassword, command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const PASSWORD: &str = "secret";
            const COMMAND_STR: &str = "set guest.password secret";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::SetGuestPassword(PASSWORD), command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "get owner.info";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::GetOwnerInfo, command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const OWNER_INFO: &str = "it's a me mario";
            const COMMAND_STR: &str = "set owner.info it's a me mario";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::SetOwnerInfo(OWNER_INFO), command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "get adc.multiplier";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::GetBatteryGain, command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const BATTERY_GAIN: f32 = 30.1;
            const COMMAND_STR: &str = "set adc.multiplier 30.1";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::SetBatteryGain(BATTERY_GAIN), command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "get public.key";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::GetPublicKey, command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "get role";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::GetRole, command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "powersaving";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::GetPowerSavingState, command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "powersaving on";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::SetPowerSavingState(true), command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "powersaving off";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::SetPowerSavingState(false), command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "get repeat";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::GetRepeatState, command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "set repeat on";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::SetRepeatState(true), command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "set repeat off";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::SetRepeatState(false), command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "get path.hash.mode";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::GetHashSize, command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const HASH_SIZE: MeshCoreHashSize = MeshCoreHashSize::_3;
            const COMMAND_STR: &str = "set path.hash.mode 2";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::SetHashSize(HASH_SIZE), command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "get loop.detect";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::GetLoopDetectState, command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const LOOP_DETECT: LoopDetection = LoopDetection::Off;
            const COMMAND_STR: &str = "set loop.detect off";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::SetLoopDetection(LOOP_DETECT), command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const LOOP_DETECT: LoopDetection = LoopDetection::Minimal;
            const COMMAND_STR: &str = "set loop.detect minimal";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::SetLoopDetection(LOOP_DETECT), command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const LOOP_DETECT: LoopDetection = LoopDetection::Moderate;
            const COMMAND_STR: &str = "set loop.detect moderate";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::SetLoopDetection(LOOP_DETECT), command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const LOOP_DETECT: LoopDetection = LoopDetection::Strict;
            const COMMAND_STR: &str = "set loop.detect strict";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::SetLoopDetection(LOOP_DETECT), command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "get txdelay";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::ShowTransmitDelay, command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const TX_DELAY: f32 = 30.5;
            const COMMAND_STR: &str = "set txdelay 30.5";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::SetTransmitDelay(TX_DELAY), command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "get rxdelay";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::GetReceiveDelay, command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const RX_DELAY: f32 = 30.5;
            const COMMAND_STR: &str = "set rxdelay 30.5";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::SetReceiveDelay(RX_DELAY), command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "get dutycycle";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::GetDutyCycle, command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const DUTY_CYCLE: u8 = 30;
            const COMMAND_STR: &str = "set dutycycle 30";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::SetDutyCycle(DUTY_CYCLE), command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "get af";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::GetAirtimeFactor, command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const AF: f32 = 30.5;
            const COMMAND_STR: &str = "set af 30.5";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::SetAirtimeFactor(AF), command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "get int.thresh";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::GetInterferenceThreshold, command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const INT_THRESH: f32 = 30.5;
            const COMMAND_STR: &str = "set int.thresh 30.5";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::SetInterferenceThreshold(INT_THRESH), command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "get agc.reset.interval";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::GetAgcResetInterval, command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const AGC_RESET_INTERVAL: u8 = 30;
            const COMMAND_STR: &str = "set agc.reset.interval 30";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(
                    CliCommands::SetAgcResetInterval(AGC_RESET_INTERVAL),
                    command
                );
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "get multi.acks";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::GetMultiAcksEnabled, command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "set multi.acks 0";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::SetMultiAcksEnabled(false), command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "set multi.acks 1";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::SetMultiAcksEnabled(true), command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "get advert.interval";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::GetAdvertInterval, command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const ADVERT_INTERVAL: u8 = 3;
            const COMMAND_STR: &str = "set advert.interval 3";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::SetAdvertInterval(ADVERT_INTERVAL), command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "get flood.max.unscoped";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::GetUnscopedMaxHopCount, command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const HOPS: u8 = 3;
            const COMMAND_STR: &str = "set flood.max.unscoped 3";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::SetUnscopedMaxHopCount(HOPS), command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "get flood.max.advert";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::GetFloodAdvertMaxHopCount, command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const HOPS: u8 = 3;
            const COMMAND_STR: &str = "set flood.max.advert 3";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::SetFloodAdvertMaxHopCount(HOPS), command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "setperm pubkey1 0";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(
                    CliCommands::SetAclPermissions {
                        pubkey: "pubkey1",
                        permissions: PermissionLevel::Guest
                    },
                    command
                );
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "get acl";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::GetAcl, command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "get allow.read.only";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::GetRoomAccess, command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "set allow.read.only off";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::SetRoomAccess(RoomAccess::ReadWrite), command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "set allow.read.only on";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::SetRoomAccess(RoomAccess::ReadOnly), command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "region load *";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(
                    CliCommands::LoadRegionSettings {
                        name: "*",
                        allow_flood: false
                    },
                    command
                );
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "region save";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::SaveRegionSettings, command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const REGION: &str = "region1";
            const COMMAND_STR: &str = "region allowf region1";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::AllowRegionForwarding { name: REGION }, command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const REGION: &str = "region1";
            const COMMAND_STR: &str = "region denyf region1";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::DenyRegionForwarding { name: REGION }, command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const REGION: &str = "region1";
            const COMMAND_STR: &str = "region get region1";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::GetRegion { name: REGION }, command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "region home";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::GetHomeRegion, command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const REGION: &str = "region1";
            const COMMAND_STR: &str = "region home region1";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::SetHomeRegion { name: Some(REGION) }, command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "region home ";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::SetHomeRegion { name: None }, command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "region default";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::GetDefaultRegion, command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const REGION: &str = "region1";
            const COMMAND_STR: &str = "region default region1";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(
                    CliCommands::SetDefaultRegion { name: Some(REGION) },
                    command
                );
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "region default <null>";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::SetDefaultRegion { name: None }, command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const REGION: &str = "region";
            const COMMAND_STR: &str = "region put region";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(
                    CliCommands::CreateRegion {
                        name: REGION,
                        parent_name: None
                    },
                    command
                );
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const REGION: &str = "region";
            const PARENT_REGION: &str = "parent_region";
            const COMMAND_STR: &str = "region put region parent_region";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(
                    CliCommands::CreateRegion {
                        name: REGION,
                        parent_name: Some(PARENT_REGION)
                    },
                    command
                );
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        // FIXME
        // {
        //     const REGION: &str = "region";
        //     const TOKEN1: &str = "token1";
        //     const TOKEN2: &str = "token2";
        //     const COMMAND_STR: &str = "region def region token1 token2";
        //     if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
        //         assert_eq!(
        //             CliCommands::DefineRegionHierarchy { region: REGION, tokens: &[TOKEN1, TOKEN2]},
        //             command
        //         );
        //     } else {
        //         panic!("failed to parse '{COMMAND_STR}'");
        //     }
        // }
        {
            const REGION: &str = "region";
            const COMMAND_STR: &str = "region remove region";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::RemoveRegion { name: REGION }, command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "region list allowed";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::GetRegionList { filter: "allowed" }, command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "region list denied";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::GetRegionList { filter: "denied" }, command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "gps";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::GetGps, command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "gps on";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::SetGpsEnabled(true), command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "gps off";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::SetGpsEnabled(false), command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "gps sync";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::SyncGpsTime, command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "gps setloc";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::SyncGpsLocation, command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "gps advert";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::GetGpsAdvertPolicy, command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "gps advert POLICY";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::SetGpsAdvertPolicy("POLICY"), command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "sensor list";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::GetSensors { start_index: 0 }, command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "sensor list 100";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::GetSensors { start_index: 100 }, command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "sensor get SENSOR1";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::GetSensor { key: "SENSOR1" }, command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "sensor set SENSOR1 31.5";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(
                    CliCommands::SetSensor {
                        key: "SENSOR1",
                        value: 31.5
                    },
                    command
                );
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "get bridge.type";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::GetBridgeType, command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "get bridge.enabled";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::GetBridgeEnabled, command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "set bridge.enabled on";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::SetBridgeEnabled(true), command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "set bridge.enabled off";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::SetBridgeEnabled(false), command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "get bridge.delay";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::GetBridgeDelay, command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "set bridge.delay 1000";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::SetBridgeDelay(1000), command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "get bridge.source";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::GetBridgeSource, command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "set bridge.source logRx";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::SetBridgeSource("logRx"), command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "get bridge.baud";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::GetBridgeBaud, command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "set bridge.baud 115200";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::SetBridgeBaud(115200), command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "get bridge.channel";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::GetBridgeChannel, command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "set bridge.channel 14";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::SetBridgeChannel(14), command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "get bridge.secret";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::GetBridgeSecret, command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "set bridge.secret SECRET";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::SetBridgeSecret("SECRET"), command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "get bootloader.ver";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::GetBootLoaderVersion, command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "get pwrmgt.support";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::GetPowerManagementSupport, command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "get pwrmgt.source";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::GetPowerSource, command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "get pwrmgt.bootreason";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::GetBootReason, command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "get pwrmgt.bootmv";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::GetBootVoltage, command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
    }
}
