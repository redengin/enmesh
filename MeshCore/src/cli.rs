use crate::lora::MeshCoreHashSize;

/// https://docs.meshcore.io/cli_commands/
///
/// commands that can be sent to MeshCore Repeaters, Room Servers and Sensors.
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
    RemoveNeighbor(&'a [u8]),
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
    EraseLog,
    /// "log" - show the logged data
    ShowLog,

    /// "ver" - show version
    ShowVersion,
    /// "board" - show hardware name
    ShowHardwareName,

    /// "get radio" - show radio config (<freq>,<bw>,<sf>,<cr>)
    ShowRadioConfig,
    /// "set radio <freq>,<bw>,<sf>,<cr>" - set the radio config
    SetRadioConfig { freq: f32, bw: u16, sf: u8, cr: u8 },
    /// "get tx" - show tx power (integer dBm)
    ShowTxPower,
    /// "set tx <dbm>" - set tx power (integer dBm)
    SetTxPower(i8),
    /// "tempradio <freq>,<bw>,<sf>,<cr>,<timeout_mins>" - change radio parameters for a duration (minutes)
    SetTempRadioConfig {
        freq: f32,
        bw: u16,
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
    /// "set radio.rxgain <state>" - enable/disable rxgain
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
    SetBridgeDelay{delay_ms: u16},
    /// "get bridge.source" - show the number of pakcets on the bridge
    ShowBridgeSource,
    /// "set bridge.source <source>" - //TODO wtf is this
    SetBridgeSource(),
    /// "get bridge.baud" - show the baudrate supported by the bridge
    ShowBridgeBaud,
    /// "set bridge.baud <rate>" - rate: [9600, 19200, 38400, 57600, 115200]
    SetBridgeBaud{baud: u16},
    /// "get bridge.channel" - show the channel for the bridge
    ShowBridgeChannel,
    /// "set bridge.channel <channel>" - channel: [1 .. 14]
    SetBridgeChannel{channel: u8},
    /// "get bridge.secret" - show the ESP-NOW secret
    ShowBridgeSecret,
    /// "set bridge.secret <secret>" - set the ESP-NOW secret
    SetBridgeSecret{ secret: &'a [u8] },
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

pub enum LoopDetection {
    Off,
    Minimal,
    Moderate,
    Strict,
}

pub enum PermissionLevel {
    Guest,
    ReadOnly,
    Admin,
}
