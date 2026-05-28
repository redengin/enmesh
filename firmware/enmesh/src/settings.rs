// provide the common crates via re-export
use common::*;

// provide logging primitives
use log::*;
const TAG: &str = "[Settings]";

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

/// maximum number of ble bonds (i.e. stored pairings)
/// - only support one bond for now, as that is the current design of MeshCore
pub const MAX_BLE_BONDS: u8 = 1;
#[derive(Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct BleSettings {
    /// tracks wrapping of the array, so that when a new binding is added
    /// it will replace the oldest one
    pub oldest_bond_index: u8,
    pub bonds: [Option<trouble_host::BondInformation>; MAX_BLE_BONDS as usize],
}
impl BleSettings {
    /// add a new paired binding
    /// * if no binding slots are available, replaces the oldest binding slot
    pub fn add_binding(&mut self, binding: trouble_host::BondInformation) {
        // find an None to fill
        for bond in self.bonds.iter_mut() {
            if bond.is_none() {
                debug!("{TAG} using an empty bond");
                *bond = Some(binding);
                return;
            }
        }
        // replace the oldest
        debug!("{TAG} replacing the oldest bond @ {}", self.oldest_bond_index);
        self.bonds[self.oldest_bond_index as usize] = Some(binding);
        self.oldest_bond_index = (self.oldest_bond_index + 1) % MAX_BLE_BONDS;
        debug!("{TAG} next oldest bond @ {}", self.oldest_bond_index);
    }
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
