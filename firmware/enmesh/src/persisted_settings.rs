// provide the shared crates via re-export
use common::*;

// provide logging primitives
use log::*;
const TAG: &str = "[PersistedSettings]";

// provide scheduling primitives
use crate::prelude::*;

// provide the serialization traits
use serde::{Deserialize, Serialize};

/// incremental version for persisted settings
/// * only changes when there is a change to crate::Settings
const VERSION: u8 = 0;
/// size (in bytes) of the serialized header
const PERSISTED_SETTINGS_HEADER_SZ: usize = 100;
/// stored version of Settings
#[derive(Serialize, Deserialize)]
struct PersistedSettingsHeader {
    /// format used in this store (aka PersistedSettings_0, PersistedSettings_1, etc.)
    /// * if the store version differs from the current version, the settings will
    ///      be transmuted to the current version (using defaults for non-stored values)
    version: u8,
}
impl PersistedSettingsHeader {
    pub fn load(settings_partition: &mut impl crate::storage::Storage) -> Option<Self> {
        // load the header
        let mut buffer = [0u8; 10];
        match settings_partition.read(0, &mut buffer) {
            Ok(()) => {}
            Err(e) => {
                error!("{TAG} failed to read header from: {:?}", e);
                return None;
            }
        }

        return match postcard::from_bytes(&buffer) {
            Ok(header) => Some(header),
            Err(e) => {
                warn!("{TAG} unable to deserialize header");
                None
            }
        };
    }

    pub fn store(
        &self,
        _settings_partition: &mut impl crate::storage::Storage,
        _next_id: u8,
        _settings: &crate::Settings,
    ) -> Result<(), ()> {
        Err(())
    }
}



/// size (in bytes) of persisted storage (header and data)
const PERSISTED_STORAGE_SZ_MAX: usize = 100;

#[derive(Serialize, Deserialize)]
struct PersistedSettings {
    /// <details>
    ///     <summary> the id is used to find the most recent copy of the settings</summary>
    ///     This implementation uses sequential ids (i.e. increment by one with wrap-around support)
    ///     * To support wrap-around, this implementation updates both copies with the same id.
    ///         * should non-sequential ids be found, the implementation will use the largest id
    ///             * upon wrap-around this would provide the incorrect values
    /// </details>
    id: u8,
    settings: crate::Settings,
}

impl PersistedSettings {
    pub fn load(_settings_partition: &mut impl crate::storage::Storage) -> Option<Self> {
        None
    }

    pub fn store(
        &self,
        _settings_partition: &mut impl crate::storage::Storage,
        _next_id: u8,
        _settings: &crate::Settings,
    ) -> Result<(), ()> {
        Err(())
        // let persisted_settings = Self {
        //     version: VERSION,
        //     id: next_id,
        //     settings: settings.clone(),
        // };

        // // create the byte buffer to be written
        // return match postcard::to_vec::<PersistedSettings, PERSISTED_STORAGE_SZ_MAX>(&persisted_settings)
        // {
        //     Ok(bytes) => {
        //         // erase enough sectors to hold the byte buffer
        //         let sector_count = bytes.len().div_ceil(settings_partition.sector_size());
        //         match settings_partition.erase_sectors(0, sector_count)
        //         {
        //             Ok(_) => {
        //                 // write the data
        //                 match settings_partition.write(0, &bytes[0..])
        //                 {
        //                     Ok(_) => { Ok(()) }
        //                     Err(_) => {
        //                         warn!("{TAG} failed to store");
        //                         Err(())
        //                     }
        //                 }
        //             }
        //             Err(_e) => {
        //                 // FIXME storage should expose the underlying error
        //                 // warn!("{TAG} failed to erase sectors, aborting store [{:?}]", e);
        //                 warn!("{TAG} failed to erase sectors, aborting store");
        //                 Err(())
        //             }
        //         }
        //     }
        //     Err(_) => {
        //         error!(
        //             "{TAG} failed to serialize settings, \
        //                 check PERSISTED_SETTINGS_SZ_MAX [{PERSISTED_STORAGE_SZ_MAX}]"
        //         );
        //         Err(())
        //     }
        // }
    }
}

pub async fn run(
    global_state: &'static RwLock<NoopRawMutex, crate::State>,
    settings_partition_a: Option<&mut impl crate::storage::Storage>,
    settings_partition_b: Option<&mut impl crate::storage::Storage>,
) {
    let mut persisted_settings_a: Option<PersistedSettings> = None;
    let mut persisted_settings_b: Option<PersistedSettings> = None;
    if let Some(p) = settings_partition_a {
        debug!("{TAG} loading settings partition a");
        persisted_settings_a = PersistedSettings::load(p);
    }
    if let Some(p) = settings_partition_b {
        debug!("{TAG} loading settings partition b");
        persisted_settings_b = PersistedSettings::load(p);
    }

    // choose most recent settings
    let mut id: u8 = 0;
    let mut current_settings: Option<crate::Settings> = None;
    if let Some(latest) = choose_latest_settings(&persisted_settings_a, &persisted_settings_b) {
        // use the returned tuple to update our state
        id = latest.0;
        current_settings = Some(latest.1);
    }

    // only update the persisted settings periodically
    // * allows settings changes to coalesce before update the persisted settings partitions
    const PERSISTED_SETTINGS_UPDATE_PERDIOD: embassy_time::Duration =
        embassy_time::Duration::from_secs(1);
    loop {
        // compare the "active"
        let global_state_lock = global_state.read().await;
        let active_settings = global_state_lock.settings;
        drop(global_state_lock);
        if let Some(persisted_settings) = current_settings {
            if persisted_settings != active_settings {
                // persist_setttings()
            }
        } else {
            // persist_setttings()
        }

        // wait for the next period
        embassy_time::Timer::after(PERSISTED_SETTINGS_UPDATE_PERDIOD).await;
    }
}

/// returns tuple (id, settings)
/// * id - id of chosen settings
fn choose_latest_settings(
    settings_a: &Option<PersistedSettings>,
    settings_b: &Option<PersistedSettings>,
) -> Option<(u8, crate::Settings)> {
    if let Some(a) = settings_a {
        if let Some(b) = settings_b {
            // support wrapping
            if (a.id == u8::MIN) && (b.id == u8::MAX) {
                return Some((a.id, a.settings.clone()));
            } else if (b.id == u8::MIN) && (a.id == u8::MAX) {
                return Some((b.id, b.settings.clone()));
            }
            // return the max id
            else if a.id > b.id {
                return Some((a.id, a.settings.clone()));
            } else {
                return Some((b.id, b.settings.clone()));
            }
        }
        return Some((a.id, a.settings.clone()));
    } else if let Some(b) = settings_b {
        return Some((b.id, b.settings.clone()));
    }

    // no persisted settings
    None
}

// TESTING
//--------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

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
                assert_eq!(core::cmp::max(ID_A, ID_B), choice.0);
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
                assert_eq!(ID_A, choice.0);
            }
        }
    }
}
