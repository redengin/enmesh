// provide the shared crates via re-export
use common::*;

// provide scheduling primitives
use crate::prelude::*;

// provide the serialization traits
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
/// stored version of Settings
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

pub async fn run(
    _global_state: &'static RwLock<NoopRawMutex, crate::State>,
    settings_partition_a: Option<&mut impl crate::storage::Storage>,
    _settings_partition_b: Option<&mut impl crate::storage::Storage>,
) {
    if let Some(p) = settings_partition_a {
        let mut buffer: [u8; 100] = [0; 100];
        p.read(0, &mut buffer);
    }
}
