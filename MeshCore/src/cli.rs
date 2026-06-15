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


    // TODO https://docs.meshcore.io/cli_commands/#routing


}
