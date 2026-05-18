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

// provide scheduling primitives
use crate::prelude::*;

/// manages the storage in non-volatile memory of settings
pub struct PersistedSettings<S1, S2>
    where S1: crate::storage::Storage + 'static,
          S2: crate::storage::Storage + 'static,
{
    global_state: &'static RwLock<NoopRawMutex, crate::State>,
    settings_partition_a: Option<&'static S1>,
    settings_partition_b: Option<&'static S2>,
}
impl<S1, S2> PersistedSettings<S1, S2>
    where S1: crate::storage::Storage + 'static,
          S2: crate::storage::Storage + 'static,
{
    pub fn new(
        global_state: &'static RwLock<NoopRawMutex, crate::State>,
        // settings_partition_a: Option<&impl crate::storage::Storage>,
        settings_partition_a: Option<&'static S1>,
        settings_partition_b: Option<&'static S2>,
        // settings_partition_b: Option<&impl crate::storage::Storage>,
    ) -> Self {
        Self {
            global_state,
            settings_partition_a,
            settings_partition_b,
        }
    }
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
