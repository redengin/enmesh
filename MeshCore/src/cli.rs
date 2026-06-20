use core::f32::consts::E;

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
    ShowBatteryGain,
    /// "set adc.multiplier <value>" - set the battery ADC scaling
    SetBatteryGain(f32),
    /// "get public.key" - show the public key
    ShowPublicKey,
    /// "get role" - show this node's role
    ShowRole,
    /// "powersaving" - show if power saving is enabled
    ShowPowerSavingState,
    /// "powersaving <value>" - "on":enable power saving, "off": disable power saving
    SetPowerSavingState(bool),

    /// "get repeat" - show if repeating is enabled
    ShowRepeatState,
    /// "set repeat <state>" - "on": enable / "off": disable
    SetRepeatState(bool),
    /// "get path.hash.mode" - show advert path hash-size
    ShowHashSize,
    /// "set path.hash.mode <value>" - 0: 1 byte hash, 1: 2 byte hash, 2: 3 byte hash
    SetHashSize(MeshCoreHashSize),
    /// "get loop.detect" - show if loop-detection enabled
    ShowLoopDetectState,
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
    SetDutyCycle(u8),
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
        pubkey: &'a str,
        permissions: PermissionLevel,
    },
    /// "get acl" - show the ACL
    ShowAcl,
    /// "get allow.read.only" - show if this room is read-only
    ShowRoomAccess,
    /// "set allow.read.only <state>" - "on": read-only, "off" - read-write
    SetRoomAccess(RoomAccess),

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
                return Ok(Self::ShowName);
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
                return Ok(Self::ShowLat);
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
                return Ok(Self::ShowLon);
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
                return Ok(Self::ShowPrivateKey);
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
                return Ok(Self::ShowGuestPassword);
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
                return Ok(Self::ShowOwnerInfo);
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
                return Ok(Self::ShowBatteryGain);
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
                return Ok(Self::ShowPublicKey);
            }
        }
        {
            const COMMAND_STRING: &str = "get role";
            if s.eq(COMMAND_STRING) {
                return Ok(Self::ShowRole);
            }
        }
        {
            const COMMAND_STRING: &str = "powersaving";
            if s.eq(COMMAND_STRING) {
                return Ok(Self::ShowPowerSavingState);
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
                return Ok(Self::ShowRepeatState);
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
                return Ok(Self::ShowHashSize);
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
                return Ok(Self::ShowLoopDetectState);
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
                return Ok(Self::ShowReceiveDelay);
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
                return Ok(Self::ShowDutyCycle);
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
                return Ok(Self::ShowAirtimeFactor);
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
                return Ok(Self::ShowInterferenceThreshold);
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
                return Ok(Self::ShowAgcResetInterval);
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
                return Ok(Self::ShowMultiAcksEnabled);
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
                return Ok(Self::ShowAdvertInterval);
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
                return Ok(Self::ShowUnscopedMaxHopCount);
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
                return Ok(Self::ShowFloodAdvertMaxHopCount);
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
                            return Ok(Self::SetAclPermissions { pubkey, permissions })
                        }
                        else {
                            return Err("unsupported level: {level_value}")
                        }
                    }
                }
            }
        }
        {
            const COMMAND_STRING: &str = "get acl";
            if s.eq(COMMAND_STRING) {
                return Ok(Self::ShowAcl);
            }
        }
        {
            const COMMAND_STRING: &str = "get allow.read.only";
            if s.eq(COMMAND_STRING) {
                return Ok(Self::ShowRoomAccess);
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
                            'F' => return Ok(Self::SetRegion{name, allow_flood: true}),
                            _ => return Err("flood flag unknown, expected 'F'")
                        }
                    }
                }
                else {
                    let name = &s[used..];
                    return Ok(Self::SetRegion{name, allow_flood: false})
                }
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
        }
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

    use core::time::Duration;

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
        {
            const COMMAND_STR: &str = "get name";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::ShowName, command);
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
                assert_eq!(CliCommands::ShowLat, command);
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
                assert_eq!(CliCommands::ShowLon, command);
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
                assert_eq!(CliCommands::ShowPrivateKey, command);
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
                assert_eq!(CliCommands::ShowGuestPassword, command);
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
                assert_eq!(CliCommands::ShowOwnerInfo, command);
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
                assert_eq!(CliCommands::ShowBatteryGain, command);
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
                assert_eq!(CliCommands::ShowPublicKey, command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "get role";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::ShowRole, command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "powersaving";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::ShowPowerSavingState, command);
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
                assert_eq!(CliCommands::ShowRepeatState, command);
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
                assert_eq!(CliCommands::ShowHashSize, command);
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
                assert_eq!(CliCommands::ShowLoopDetectState, command);
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
                assert_eq!(CliCommands::ShowReceiveDelay, command);
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
                assert_eq!(CliCommands::ShowDutyCycle, command);
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
                assert_eq!(CliCommands::ShowAirtimeFactor, command);
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
                assert_eq!(CliCommands::ShowInterferenceThreshold, command);
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
                assert_eq!(CliCommands::ShowAgcResetInterval, command);
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
                assert_eq!(CliCommands::ShowMultiAcksEnabled, command);
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
                assert_eq!(CliCommands::ShowAdvertInterval, command);
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
                assert_eq!(CliCommands::ShowUnscopedMaxHopCount, command);
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
                assert_eq!(CliCommands::ShowFloodAdvertMaxHopCount, command);
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
                assert_eq!(CliCommands::ShowAcl, command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "get allow.read.only";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(CliCommands::ShowRoomAccess, command);
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "set allow.read.only off";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(
                    CliCommands::SetRoomAccess(RoomAccess::ReadWrite),
                    command
                );
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "set allow.read.only on";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(
                    CliCommands::SetRoomAccess(RoomAccess::ReadOnly),
                    command
                );
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }
        {
            const COMMAND_STR: &str = "region load *";
            if let Ok(command) = CliCommands::from_string(COMMAND_STR) {
                assert_eq!(
                    CliCommands::SetRegion{name: "*", allow_flood: false},
                    command
                );
            } else {
                panic!("failed to parse '{COMMAND_STR}'");
            }
        }







    }
}
