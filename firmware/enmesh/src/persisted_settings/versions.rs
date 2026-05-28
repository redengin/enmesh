// provide the shared crates via re-export
use common::*;

// provide logging primitives
use log::*;
const TAG: &str = "[PersistedSettings::Version]";

// provide scheduling primitives
// use crate::prelude::*;

// provide the serialization traits
use serde::{Deserialize, Serialize};

/// old version id's will be recycled as they age out
pub enum Versions {
    V0 = 0, // started 05/2026
}
impl Versions {
    pub const fn current() -> Self {
        Self::V0
    }
}

/// incremental version for persisted settings
/// * only changes when there is a change to crate::Settings
pub const CURRENT_VERSION: u8 = Versions::current() as u8;
/// size (in bytes) of the serialized header
const PERSISTED_SETTINGS_HEADER_SZ: usize = 1;
/// size (in bytes) of the buffer (padded for generic alignment)
pub const PERSISTED_SETTINGS_HEADER_BUFFER_SZ: usize = crate::storage::utils::buffer_size(
    PERSISTED_SETTINGS_HEADER_SZ,
    crate::storage::WordSize::max(),
);
/// stored version of Settings
#[derive(Serialize, Deserialize)]
pub(crate) struct PersistedSettingsHeader {
    /// format used in this store (aka PersistedSettings_0, PersistedSettings_1, etc.)
    /// * if the store version differs from the current version, the settings will
    ///      be transmuted to the current version (using defaults for non-stored values)
    pub version: u8,
}
impl PersistedSettingsHeader {
    pub fn new() -> Self {
        Self {
            version: CURRENT_VERSION,
        }
    }

    pub fn load(settings_partition: &mut impl crate::storage::Storage) -> Option<Self> {
        // read the data from storage
        let mut buffer = [0u8; PERSISTED_SETTINGS_HEADER_BUFFER_SZ];
        match settings_partition.read(0, &mut buffer) {
            Ok(()) => {
                debug!("{TAG} storage read successful");
            }
            Err(e) => {
                error!("{TAG} storage read failed: {:?}", e);
                return None;
            }
        }

        // deserialize the buffer
        return match postcard::from_bytes::<PersistedSettingsHeader>(
            &buffer[..PERSISTED_SETTINGS_HEADER_SZ + 1],
        ) {
            Ok(header) => {
                debug!("{TAG} deserialized");
                Some(header)
            }
            Err(e) => {
                warn!("{TAG} deserialization failed [{e}]");
                None
            }
        };
    }

    /// Write the persisted settings header to storage
    /// * assumes destination is erased
    pub fn store(settings_partition: &mut impl crate::storage::Storage) -> Result<(), ()> {
        // serialize the data
        let mut buffer = [0u8; PERSISTED_SETTINGS_HEADER_BUFFER_SZ];
        let header = PersistedSettingsHeader::new();
        match postcard::to_slice(&header, &mut buffer) {
            Ok(bytes) => {
                debug!("{TAG} stored");
                if PERSISTED_SETTINGS_HEADER_SZ != bytes.len() {
                    panic!(
                        "{TAG} incorrect PERSISTED_SETTINGS_HEADER_SZ \
                           (is: {PERSISTED_SETTINGS_HEADER_SZ}, should be: {:?})",
                        bytes.len()
                    );
                }
            }
            Err(e) => {
                error!("{TAG} failed to serialize: {e}");
                return Err(());
            }
        }

        // write the header to storage
        return match settings_partition.write(0, &buffer) {
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

// TESTING
//--------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(non_snake_case)]
    /// used to validate PERSISTED_SETTINGS_HEADER_SZ
    fn validate_PERSISTED_SETTINGS_HEADER_SZ() {
        let header = PersistedSettingsHeader::new();

        let mut buffer = [0u8; PERSISTED_SETTINGS_HEADER_BUFFER_SZ];
        match postcard::to_slice(&header, &mut buffer) {
            Ok(bytes) => {
                let actual_len = bytes.len();
                assert_eq!(
                    PERSISTED_SETTINGS_HEADER_SZ, actual_len,
                    "incorrect PERSISTED_SETTINGS_HEADER_SZ  (is: {PERSISTED_SETTINGS_HEADER_SZ}, should be: {actual_len})"
                );
            }
            Err(e) => {
                panic!("failed to serialize: {e}");
            }
        }
    }
}
