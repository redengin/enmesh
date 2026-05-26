// provide the common crates via re-export
use common::*;

// provide the serialization traits
use serde::{Serialize, Deserialize};

#[derive(Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct Settings {
    pub ux_settings: UxSettings,

    pub ble_settings: BleSettings,

    pub meshtastic_settings: MeshtasticSettings,
    pub meshcore_settings: MeshCoreSettings,
}

#[derive(Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct UxSettings {
    // nothing yet...
}

#[derive(Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct BleSettings {
    // TODO support multiple bonds
    pub bond: Option<trouble_host::BondInformation>,
}

#[derive(Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct MeshtasticSettings {
    /// if enabled, the LoRa task will handle Meshtastic
    pub enabled: bool,
    pub lora_config: crate::lora::EnmeshLoRaConfig,
    /// determines how non-volatile storage to allocate to Meshtastic packets
    /// 0 - none, 100 - maximum storage
    pub storage_weight: u8,
}

#[derive(Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct MeshCoreSettings {
    /// if enabled, the LoRa task will handle MeshCore
    pub enabled: bool,
    pub lora_config: crate::lora::EnmeshLoRaConfig,

    /// determines how non-volatile storage to allocate to Meshtastic packets
    /// 0 - none, 100 - maximum storage
    pub storage_weight: u8,
}
