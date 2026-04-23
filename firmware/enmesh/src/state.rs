


/// globally shared state for firmware
#[derive(Default)]
pub struct State {
    pub firmware_version: &'static str,

    /// settings (persisted in non-volatile memory)
    pub settings: crate::Settings,

    pub battery_percent: u8,

    /// used by UX for display and LEDs, set by lora thread
    pub lora_mode: LoRaRadioMode,
    /// used by UX for display, set by lora thread
    pub current_protocol: LoRaProtocol,

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

#[derive(Default)]
pub enum LoRaProtocol {
    #[default]
    Meshtastic,
    MeshCore,
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

