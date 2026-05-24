use core::ops::DerefMut;

// provide the shared crates via re-export
use common::*;

// provide logging primitives
use log::*;
const TAG: &str = "[PersistedSettings]";

// provide scheduling primitives
use crate::prelude::*;

// provide the serialization traits
use serde::{Deserialize, Serialize};

/// size (in bytes) of the serialized header
const PERSISTED_SETTINGS_SZ: usize = 29;
/// size (in bytes) of the buffer (padded for generic alignment)
const PERSISTED_SETTINGS_BUFFER_SZ: usize =
    crate::storage::utils::buffer_size(PERSISTED_SETTINGS_SZ, crate::storage::WordSize::max());
#[derive(Serialize, Deserialize)]
struct PersistedSettings {
    /// The id is used to find the most recent copy of the settings</summary>
    /// * This implementation uses sequential ids (i.e. increment by one with wrap-around support)
    id: u8,
    settings: crate::Settings,
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
                        error!("{TAG} support for other versions not implemented");
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
                error!("{TAG} storage read failed: {:?}", e);
                return None;
            }
        }

        // deserialize the buffer (using current version)
        return match postcard::from_bytes::<PersistedSettings>(&buffer[..PERSISTED_SETTINGS_SZ + 1])
        {
            Ok(settings) => {
                debug!("{TAG} deserialized settings");
                Some(settings)
            }
            Err(e) => {
                warn!("{TAG} deserialization failed [{e}]");
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
    ) -> Result<(), ()> {
        // write the header to storage
        let header = PersistedSettingsHeader::store(settings_partition);
        if header.is_err() {
            // header write failed
            return Err(());
        }

        // serialize the data
        let settings = PersistedSettings {
            id: next_id,
            settings: *settings,
        };
        let mut buffer = [0u8; PERSISTED_SETTINGS_BUFFER_SZ];
        match postcard::to_slice(&settings, &mut buffer) {
            Ok(bytes) => {
                debug!("{TAG} stored");
                if PERSISTED_SETTINGS_SZ != bytes.len() {
                    panic!(
                        "{TAG} incorrect PERSISTED_SETTINGS_SZ \
                           (is: {PERSISTED_SETTINGS_SZ}, should be: {:?})",
                        bytes.len()
                    );
                }
            }
            Err(e) => {
                error!("{TAG} failed to serialize: {e}");
                return Err(());
            }
        }

        // write the data to storage
        return match settings_partition.write(PERSISTED_SETTINGS_HEADER_BUFFER_SZ, &buffer) {
            Ok(()) => {
                debug!("{TAG} wrote to storage");
                Ok(())
            }
            Err(e) => {
                error!("{TAG} failed write to storage: {:?}", e);
                Err(())
            }
        };
    }
}

pub async fn run(
    global_state: &'static RwLock<NoopRawMutex, crate::State>,
    mut settings_partition_a: Option<&mut impl crate::storage::Storage>,
    mut settings_partition_b: Option<&mut impl crate::storage::Storage>,
) {
    // load the persisted settings
    let persisted_settings_a: Option<PersistedSettings> = match settings_partition_a {
        Some(ref mut p) => PersistedSettings::load(p.deref_mut()),
        None => None,
    };
    let persisted_settings_b: Option<PersistedSettings> = match settings_partition_b {
        Some(ref mut p) => PersistedSettings::load(p.deref_mut()),
        None => None,
    };

    // choose most recent settings
    let mut id: u8 = 0;
    let mut current_settings: Option<crate::Settings> = None;
    if let Some(latest) = choose_latest_settings(&persisted_settings_a, &persisted_settings_b) {
        // use the returned tuple to update our state
        id = latest.0;
        current_settings = Some(latest.1);

        // update the global state
        let mut global_state_lock = global_state.write().await;
        global_state_lock.settings = current_settings.unwrap();
        drop(global_state_lock);
    }

    // update the persisted settings periodically
    // * allows settings changes to coalesce over a short duration
    const PERSISTED_SETTINGS_UPDATE_PERDIOD: Duration = Duration::from_secs(10);
    loop {
        // compare the "active"
        let global_state_lock = global_state.read().await;
        let active_settings = global_state_lock.settings;
        drop(global_state_lock);
        if current_settings.is_none() || (current_settings.unwrap() != active_settings) {
            // update the persisted settings
            id += 1;
            current_settings = Some(active_settings);

            // store the persisted settings
            const PERSISTED_SIZE: usize =
                PERSISTED_SETTINGS_HEADER_BUFFER_SZ + PERSISTED_SETTINGS_BUFFER_SZ;
            match settings_partition_a {
                Some(ref mut p) => {
                    // erase the storage
                    let sector_count =
                        crate::storage::utils::sector_count(PERSISTED_SIZE, p.sector_size());
                    let _ = p.erase_sectors(0, sector_count);
                    // write the settings
                    let _ = PersistedSettings::store(p.deref_mut(), id, &active_settings);
                }
                None => {}
            };
            match settings_partition_b {
                Some(ref mut p) => {
                    // erase the storage
                    let sector_count =
                        crate::storage::utils::sector_count(PERSISTED_SIZE, p.sector_size());
                    let _ = p.erase_sectors(0, sector_count);
                    // write the settings
                    let _ = PersistedSettings::store(p.deref_mut(), id, &active_settings);
                }
                None => {}
            };
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
            // support wrapping max id
            if (a.id == u8::MIN) && (b.id == u8::MAX) {
                return Some((a.id, a.settings.clone()));
            } else if (b.id == u8::MIN) && (a.id == u8::MAX) {
                return Some((b.id, b.settings.clone()));
            }
            // support max id
            else if a.id > b.id {
                return Some((a.id, a.settings.clone()));
            } else {
                return Some((b.id, b.settings.clone()));
            }
        }
        // only A so return A
        return Some((a.id, a.settings.clone()));
    } else if let Some(b) = settings_b {
        // only B so return B
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
    #[allow(lint_name)]
    /// used to validate PERSISTED_SETTINGS_SZ
    fn validate_PERSISTED_SETTINGS_SZ() {
        let settings = PersistedSettings {
            id: 0,
            settings: crate::Settings::default(),
        };

        let mut buffer = [0u8; PERSISTED_SETTINGS_BUFFER_SZ];
        match postcard::to_slice(&settings, &mut buffer) {
            Ok(bytes) => {
                let actual_len = bytes.len();
                assert_eq!(
                    PERSISTED_SETTINGS_SZ, actual_len,
                    "incorrect PERSISTED_SETTINGS_SZ  (is: {PERSISTED_SETTINGS_SZ}, should be: {actual_len})"
                );
            }
            Err(e) => {
                panic!("failed to serialize");
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
