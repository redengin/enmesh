#[derive(Default, Copy, Clone)]
pub struct Settings {
    pub ux_settings: UxSettings, 

    pub meshtastic_settings: MeshtasticSettings,
    pub meshcore_settings: MeshCoreSettings,
}

#[derive(Default, Copy, Clone)]
pub struct UxSettings {
    // nothing yet...
}

#[derive(Default, Copy, Clone)]
pub struct MeshtasticSettings {
    /// if enabled, the LoRa task will handle Meshtastic
    pub enabled: bool,
    pub lora_config: crate::lora::EnmeshLoRaConfig,
    /// determines how non-volatile storage to allocate to Meshtastic packets
    /// 0 - none, 100 - maximum storage
    pub storage_weight: u8,
}

#[derive(Default, Copy, Clone)]
pub struct MeshCoreSettings {
    /// if enabled, the LoRa task will handle MeshCore
    pub enabled: bool,
    pub lora_config: crate::lora::EnmeshLoRaConfig,

    /// determines how non-volatile storage to allocate to Meshtastic packets
    /// 0 - none, 100 - maximum storage
    pub storage_weight: u8,
}


// impl Settings {
//     pub async fn load(
//         partition: crate::storage::Partition + crate::storage::AsyncStorage
//     ) -> Self
//     {
//         Self {
//             ..Default::default()
//         }
//     }
// }