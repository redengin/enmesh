#[derive(Default)]
pub struct Settings {
    ux_settings: UxSettings, 

    meshtastic_settings: MeshtasticSettings,
    meshcore_settings: MeshCoreSettings,
}


#[derive(Default)]
pub struct UxSettings {

}

#[derive(Default)]
pub struct MeshtasticSettings {
    /// if enabled, the LoRa task will handle Meshtastic
    enabled: bool,
    lora_config: enmesh::lora::EnmeshLoRaConfig,
    /// determines how non-volatile storage to allocate to Meshtastic packets
    /// 0 - none, 100 - maximum storage
    storage_weight: u8,
}

#[derive(Default)]
pub struct MeshCoreSettings {
    /// if enabled, the LoRa task will handle MeshCore
    enabled: bool,
    lora_config: enmesh::lora::EnmeshLoRaConfig,

    /// determines how non-volatile storage to allocate to Meshtastic packets
    /// 0 - none, 100 - maximum storage
    storage_weight: u8,
}
