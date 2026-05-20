// provide the shared crates via re-export
use common::*;

// provide logging primitives
use log::*;
const TAG: &str = "[PersistedSettings]";

// provide scheduling primitives
use crate::prelude::*;

// provide the serialization traits
use serde::{Deserialize, Serialize};

/// stored version of Settings
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
    /// format used in this storage (aka PersistedSettings_0, PersistedSettings_1, etc.)
    version: u8,
    settings: crate::Settings,
}
impl PersistedSettings {
    pub fn load(_settings_partition: &mut impl crate::storage::Storage) -> Option<Self> {
        None
    }

    pub fn store(&self, _settings_partition: &mut impl crate::storage::Storage) -> Result<(), ()> {
        Err(())
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
        let mut global_state_lock = global_state.read().await;
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
            if a.id.abs_diff(b.id) == 1 {
                if a.id == 0 {
                    return Some((a.id, a.settings.clone()));
                }
                if b.id == 0 {
                    return Some((a.id, a.settings.clone()));
                }
                // else fall through and just use the largest id
            }
            // use the largest id
            if a.id > b.id {
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
        // if A is Some and B is None, should return A
        {
            let settings_a = Some(PersistedSettings{
                id: 0,
                version: 0,
                settings: Default::default(),
            });
            let chosen = choose_latest_settings(&settings_a, &None);
            assert!(chosen.is_some(), "no choice was made");
            if let Some(choice) = chosen {
                if let Some(persisted_settings) = settings_a {
                    assert!(persisted_settings.id == choice.0, "with only A, failed to select A");
                }
            }
        }
    }
}