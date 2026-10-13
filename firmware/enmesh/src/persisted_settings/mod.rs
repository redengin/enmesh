/// provide the shared crates via re-export
use common::*;

/// provide logging primitives
#[allow(unused_imports)]
use log::*;
const TAG: &str = "[PersistedSettings]";

/// provide enmesh firmware primitives
use crate::prelude::*;

pub struct PersistedSettingsManager {
    persisted_settings: Option<PersistedSettings>,
}
impl PersistedSettingsManager {
    /// loads the persisted settings and updates the global settings
    pub async fn init(
        global_state: &'static RwLock<NoopRawMutex, crate::State>,
        settings_partition_a: Option<&mut impl crate::storage::Storage>,
        settings_partition_b: Option<&mut impl crate::storage::Storage>,
    ) -> Self {
        // try loading settings from parition A
        let persisted_settings_a = match settings_partition_a {
            Some(partition) => PersistedSettings::load(partition),
            None => None,
        };

        // try loading settings from parition B
        let persisted_settings_b = match settings_partition_b {
            Some(partition) => PersistedSettings::load(partition),
            None => None,
        };

        // choose latest persisted settings
        let persisted_settings =
            choose_latest_settings(&persisted_settings_a, &persisted_settings_b);

        // update global state with settings
        if let Some(ref persistant_settings) = persisted_settings {
            let mut global_state_lock = global_state.write().await;
            global_state_lock.settings = persistant_settings.settings.clone();
            drop(global_state_lock);
        }

        Self { persisted_settings }
    }

    /// periodically persists updated settings
    /// * allows settings changes to coalesce over a short duration
    pub async fn run(
        &mut self,
        global_state: &'static RwLock<NoopRawMutex, crate::State>,
        mut settings_partition_a: Option<&mut impl crate::storage::Storage>,
        mut settings_partition_b: Option<&mut impl crate::storage::Storage>,
    ) {
        // update rate
        const PERSISTED_SETTINGS_UPDATE_PERIOD: Duration = Duration::from_secs(10);
        loop {
            // grab a copy of the current state
            let global_state_lock = global_state.read().await;
            let active_settings = global_state_lock.settings.clone();
            drop(global_state_lock);

            // persist changed settings
            let needs_update =
                if let Some(persisted_settings) = &self.persisted_settings
                {
                    persisted_settings.settings != active_settings
                }
                else {
                    true
                };

            if needs_update {
                // persist the new settings under the next id
                let id = if let Some(persisted_settings) = &self.persisted_settings {
                    persisted_settings.id + 1
                }
                else { 0 };

                // store the new settings on partition A
                if let Some(ref mut partition) = settings_partition_a {
                    PersistedSettings::store(*partition, id, &active_settings);
                }
                // store the new settings on partition B
                if let Some(ref mut partition) = settings_partition_b {
                    PersistedSettings::store(*partition, id, &active_settings);
                }

                // memo our new persisted settings
                self.persisted_settings = Some(PersistedSettings{id, settings: active_settings});
            }

            // wait for the next period
            embassy_time::Timer::after(PERSISTED_SETTINGS_UPDATE_PERIOD).await;
        }
    }
}

/// returns latest version of persisted settings or None
fn choose_latest_settings(
    settings_a: &Option<PersistedSettings>,
    settings_b: &Option<PersistedSettings>,
) -> Option<PersistedSettings> {
    if let Some(a) = settings_a {
        if let Some(b) = settings_b {
            // support wrapping max id
            if (a.id == u8::MIN) && (b.id == u8::MAX) {
                return Some(a.clone());
            } else if (b.id == u8::MIN) && (a.id == u8::MAX) {
                return Some(b.clone());
            }
            // support max id
            else if a.id > b.id {
                return Some(a.clone());
            } else {
                return Some(b.clone());
            }
        }
        // only A so return A
        return Some(a.clone());
    } else if let Some(b) = settings_b {
        // only B so return B
        return Some(b.clone());
    }

    // no persisted settings
    None
}

// provide the serialization traits
use serde::{Deserialize, Serialize};

/// size (in bytes) of the serialized header
const PERSISTED_SETTINGS_SZ: usize = 100;
/// size (in bytes) of the buffer (padded for generic alignment)
const PERSISTED_SETTINGS_BUFFER_SZ: usize =
    crate::storage::utils::buffer_size(PERSISTED_SETTINGS_SZ, crate::storage::WordSize::max());
#[derive(Default, Serialize, Deserialize, Clone)]
struct PersistedSettings {
    /// The id is used to find the most recent copy of the settings</summary>
    /// * This implementation uses sequential ids (i.e. increment by one with wrap-around support)
    pub id: u8,
    pub settings: crate::Settings,
}

// provide support for converting older versions of persisted settings
mod versions;
use versions::{PERSISTED_SETTINGS_HEADER_BUFFER_SZ, PersistedSettingsHeader};

impl PersistedSettings {
    pub fn load(settings_partition: &mut impl crate::storage::Storage) -> Option<Self> {
        // load the header
        match PersistedSettingsHeader::load(settings_partition) {
            Some(header) => {
                match header.version {
                    versions::CURRENT_VERSION => {
                        // continue with serialization/deserialization using current version
                        debug!("{TAG} header found using current version");
                    }
                    _ => {
                        error!("{TAG} support for backward compatabilty not implemented");
                        // TODO versions::PersistedSettings_v1::load(settings_partition) -> Option<PersistedSettings>
                        // implement just like below but convert previous PersistedSettings version to current
                        return None;
                    }
                }
            }
            None => {
                debug!("{TAG} no header found");
                return None;
            }
        }

        // read the data from storage
        let mut buffer = [0u8; PERSISTED_SETTINGS_BUFFER_SZ];
        match settings_partition.read(PERSISTED_SETTINGS_HEADER_BUFFER_SZ, &mut buffer) {
            Ok(()) => {
                debug!("{TAG} storage read successful");
            }
            Err(e) => {
                error!(
                    "{TAG} storage read failed [buffer_sz: {}]: {:?}",
                    buffer.len(),
                    e
                );
                return None;
            }
        }

        // deserialize the buffer (using current version)
        return match postcard::from_bytes::<PersistedSettings>(&buffer[..PERSISTED_SETTINGS_SZ]) {
            Ok(settings) => {
                debug!("{TAG} deserialized settings");
                Some(settings)
            }
            Err(e) => {
                error!(
                    "{TAG} deserialization failed [buffer_sz: {}]: {e}",
                    buffer.len()
                );
                None
            }
        };
    }

    /// Write the persisted settings to storage
    /// * assumes destination is erased
    pub fn store(
        settings_partition: &mut impl crate::storage::Storage,
        next_id: u8,
        settings: &crate::Settings,
    ) {
        // erase the partition to prepare for writing
        settings_partition.erase().ok();

        // serialize the data
        let settings = PersistedSettings {
            id: next_id,
            settings: settings.clone(),
        };
        let mut buffer = [0u8; PERSISTED_SETTINGS_BUFFER_SZ];
        match postcard::to_slice(&settings, &mut buffer) {
            Ok(bytes) => {
                debug!("{TAG} serialization successful");
                if PERSISTED_SETTINGS_SZ < bytes.len() {
                    // (see test validate_PERSISTED_SETTINGS_SZ to find correct value)
                    warn!(
                        "{TAG} incorrect PERSISTED_SETTINGS_SZ \
                           (is: {PERSISTED_SETTINGS_SZ}, should be at least: {})",
                        bytes.len()
                    );
                }
            }
            Err(e) => {
                error!(
                    "{TAG} serialization failed [buffer_sz: {PERSISTED_SETTINGS_BUFFER_SZ}]: {e}"
                );
            }
        }
        
        // write the header to storage
        PersistedSettingsHeader::store(settings_partition);

        // write the data to storage
        match settings_partition.write(PERSISTED_SETTINGS_HEADER_BUFFER_SZ, &buffer) {
            Ok(()) => {
                debug!("{TAG} wrote to storage");
            }
            Err(e) => {
                error!("{TAG} failed write to storage: {:?}", e);
            }
        };
    }
}

// TESTING
//--------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(non_snake_case)]
    fn validate_PERSISTED_SETTINGS_SZ() {
        // FIXME postcard uses COBS, which has a dynamic serialized size

        // create some settings
        let settings = PersistedSettings::default();

        // serialize the settings
        const PERSISTED_SETTINGS_BUFFER_SZ_MAX: usize = 4096;
        let mut buffer = [0u8; PERSISTED_SETTINGS_BUFFER_SZ_MAX];
        match postcard::to_slice(&settings, &mut buffer) {
            Ok(bytes) => {
                let actual_len = bytes.len();
                assert!(
                    PERSISTED_SETTINGS_SZ >= actual_len,
                    "incorrect PERSISTED_SETTINGS_SZ  (is: {PERSISTED_SETTINGS_SZ}, should be at least: {actual_len})"
                );
            }
            Err(e) => {
                panic!("failed to serialize: {e}");
            }
        }
    }

    #[test]
    fn test_choose_latest_setting() {
        // choose(None, None)
        let chosen = choose_latest_settings(&None, &None);
        assert!(chosen.is_none(), "unexepctedly returned a choice");

        // choose(Some, None)
        {
            let settings = PersistedSettings {
                id: 0,
                settings: Default::default(),
            };
            let chosen = choose_latest_settings(&Some(settings), &None);
            assert!(chosen.is_some(), "no choice made for choose(Some, None)");
        }
        // choose(None, Some)
        {
            let settings = PersistedSettings {
                id: 0,
                settings: Default::default(),
            };

            let chosen = choose_latest_settings(&None, &Some(settings));
            assert!(chosen.is_some(), "no choice made for choose(None, Some)");
        }
        // choose(Some, Some) max id
        {
            const ID_A: u8 = 0;
            let settings_a = PersistedSettings {
                id: ID_A,
                settings: Default::default(),
            };
            const ID_B: u8 = 100;
            let settings_b = PersistedSettings {
                id: ID_B,
                settings: Default::default(),
            };

            let chosen = choose_latest_settings(&Some(settings_a), &Some(settings_b));
            assert!(chosen.is_some(), "no choice made for choose(Some, Some)");
            if let Some(choice) = chosen {
                assert_eq!(core::cmp::max(ID_A, ID_B), choice.id);
            }
        }
        // choose(Some, Some) wrapping sequential id
        {
            const ID_A: u8 = u8::MIN;
            let settings_a = PersistedSettings {
                id: ID_A,
                settings: Default::default(),
            };
            const ID_B: u8 = u8::MAX;
            let settings_b = PersistedSettings {
                id: ID_B,
                settings: Default::default(),
            };

            let chosen = choose_latest_settings(&Some(settings_a), &Some(settings_b));
            assert!(chosen.is_some(), "no choice made for choose(Some, Some)");
            if let Some(choice) = chosen {
                assert_eq!(ID_A, choice.id);
            }
        }
    }
}
