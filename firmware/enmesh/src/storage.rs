pub trait EnmeshStorage {
    fn settings_a_partition(&self) -> Option<&'static impl AsyncStorage>;
    fn settings_b_partition(&self) -> Option<&'static impl AsyncStorage>;
    fn data_partition(&self) -> Option<&'static impl AsyncStorage>;
}

pub trait AsyncStorage {
    type Error;

    fn read_async(&mut self, offset: usize, buffer: &mut[u8])
        -> impl core::future::Future<Output=Result<(), Self::Error>> + Send;

    fn write_async(&mut self, offset: usize, buffer: &[u8])
        -> impl core::future::Future<Output=Result<(), Self::Error>> + Send;

    fn erase_async(&mut self, sector_start: usize, sector_count: usize)
        -> impl core::future::Future<Output=Result<(), Self::Error>> + Send;

    fn size(&self) -> usize;

    fn sector_size(&self) -> usize;
}



/// storage access errors
pub enum StorageError {
    OperationFailed,
}
use core::fmt;
impl fmt::Display for StorageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StorageError::OperationFailed => write!(f, "operation failed"),
        }
    }
}

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
