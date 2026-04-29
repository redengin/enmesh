


/// globally shared state for firmware
#[derive(Default)]
pub struct State {
    pub firmware_version: &'static str,

    /// settings (persisted in non-volatile memory)
    pub settings: crate::Settings,

    pub battery_percent: u8,

    /// used by UX for display, set by lora thread
    pub current_protocol: LoRaProtocol,
    /// used by UX for display and LEDs, set by lora thread
    pub current_radio_mode: LoRaRadioMode,

    pub wifi_status: WiFiStatus,
    pub ble_status: BleStatus,

    /// used by UX for display, set by storage thread
    pub storage_status: StorageStatus,
}
impl State {
    pub fn new() -> Self {
        Self {
            // FIXME bind to actual firmware version
            firmware_version: "v0.0.1",
            ..Default::default()
        }
    }
}

#[derive(Default)]
pub enum LoRaRadioMode {
    #[default]
    Sleep,
    Standby,
    Transmit,
    Receive,
}
impl core::fmt::Display for LoRaRadioMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            LoRaRadioMode::Sleep => f.write_str("Sleep"),
            LoRaRadioMode::Standby => f.write_str("Standby"),
            LoRaRadioMode::Transmit => f.write_str("Transmit"),
            LoRaRadioMode::Receive => f.write_str("Receive"),
        }
    }
}

#[derive(Default)]
pub enum LoRaProtocol {
    #[default]
    Meshtastic,
    MeshCore,
}
impl core::fmt::Display for LoRaProtocol {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            LoRaProtocol::Meshtastic => f.write_str("Meshtastic"),
            LoRaProtocol::MeshCore => f.write_str("MeshCore"),
        }
    }
}

#[derive(Default)]
pub enum WiFiStatus {
    NotConfigured,
    Connected,
    Disconnected,
    #[default]
    NotAvailable,
}
impl core::fmt::Display for WiFiStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NotConfigured => f.write_str("Not Configured"),
            Self::Connected => f.write_str("connected"),
            Self::Disconnected => f.write_str("connecting..."),
            Self::NotAvailable => f.write_str("N/A"),
        }
    }
}

#[derive(Default)]
pub enum BleStatus
{
    Connected,
    Disconnected,
    #[default]
    NotAvailable,
}
impl core::fmt::Display for BleStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Connected => f.write_str("connected"),
            Self::Disconnected => f.write_str("not connected"),
            Self::NotAvailable => f.write_str("N/A"),
        }
    }
}

#[derive(Default)]
pub struct StorageStatus {
    pub meshtastic: ProtocolStorageStatus,
    pub meshcore: ProtocolStorageStatus,
}
#[derive(Default)]
pub struct ProtocolStorageStatus {
    pub size: usize,
    pub used: usize,
}

