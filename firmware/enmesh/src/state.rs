


/// globally shared state for firmware
#[derive(Default)]
pub struct State {
    /// settings (persisted in non-volatile memory)
    settings: Settings,

    /// used by UX for display and LEDs, set by lora thread
    lora_mode: LoRaMode,
    /// used by UX for display, set by lora thread
    current_protocol: LoRaProtocol,

    /// used by UX for display, set by storage thread
    storage_status: StorageStatus,
}
impl State {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

#[derive(Default)]
enum LoRaMode {
    #[default]
    Sleep,
    Standby,
    Transmit,
    Receive,
}

#[derive(Default)]
enum LoRaProtocol {
    #[default]
    Meshtastic,
    MeshCore,
}

#[derive(Default)]
pub struct StorageStatus {
    meshtastic: ProtocolStorageStatus,
    meshcore: ProtocolStorageStatus,
}
#[derive(Default)]
pub struct ProtocolStorageStatus {
    size: usize,
    used: usize,
}

