#[derive(Default)]
pub struct Settings {
    pub ux_settings: UxSettings, 

    pub meshtastic_settings: MeshtasticSettings,
    pub meshcore_settings: MeshCoreSettings,
}

#[derive(Default)]
pub struct UxSettings {
    // nothing yet...
}

#[derive(Default)]
pub struct MeshtasticSettings {
    /// if enabled, the LoRa task will handle Meshtastic
    pub enabled: bool,
    pub lora_config: crate::lora::EnmeshLoRaConfig,
    /// determines how non-volatile storage to allocate to Meshtastic packets
    /// 0 - none, 100 - maximum storage
    pub storage_weight: u8,
}

#[derive(Default)]
pub struct MeshCoreSettings {
    /// if enabled, the LoRa task will handle MeshCore
    pub enabled: bool,
    pub lora_config: crate::lora::EnmeshLoRaConfig,

    /// determines how non-volatile storage to allocate to Meshtastic packets
    /// 0 - none, 100 - maximum storage
    pub storage_weight: u8,
}
