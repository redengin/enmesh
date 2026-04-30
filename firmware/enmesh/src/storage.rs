/// generic trait for persistables
/// 
/// Persitables have no need to support functionality like roll-back.
/// * implementations **shall**
///     * be able to verify the integrity of the persitable
///         - e.g. CRC, ECC, etc.
///     * be able to identify the version of the persistable
///     * convert the persistable to the current version
///         - invoking a store() upon conversion
/// * implementations *should*
///     * manage multiple persisted copies for robustness
pub trait Persistable {
    type Item;

    fn load() -> Option<Self::Item>;

    /// update all persistable's copies
    fn store(settings: &Self::Item) -> Result<(), crate::storage::StorageError>;
}

/// storage access errors
pub enum StorageError {
    NoPartition,
    OperationFailed,
}
use core::fmt;
impl fmt::Display for StorageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
       match self {
        StorageError::NoPartition => write!(f, "partition doesn't exist"),
        StorageError::OperationFailed => write!(f, "operation failed"),
       } 
    }
}
