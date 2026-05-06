// pub trait EnmeshStorage {
//     fn settings_partition(&self) -> Option<Partition>;
//     fn data_partition(&self) -> Option<Partition>;
// }

#[derive(Copy, Clone)]
pub struct Partition {
    /// start address of the partition
    pub address: usize,
    pub size: usize,
    pub sector_size: usize,
}
pub trait AsyncStorage {
    fn read_async(
        &mut self,
        partition: Partition,
        offset: usize,
        buffer: &mut [u8],
    ) -> impl core::future::Future<Output=Result<(), StorageError>> + Send;

    /// will erase sectors as necessary to support the write region
    /// * if sectors need to be erased, the implementation should
    ///     rewrite any persisted data outside of the buffer range
    fn write_async(
        &mut self,
        partition: Partition,
        offset: usize,
        buffer: &[u8],
    ) -> impl core::future::Future<Output=Result<(), StorageError>> + Send;
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
