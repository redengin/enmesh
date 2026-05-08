pub trait EnmeshStorage {
    fn settings_a_partition(&self) -> Option<&impl Storage>;
    fn settings_b_partition(&self) -> Option<&impl Storage>;
    fn data_partition(&self) -> Option<&impl Storage>;
}

pub trait Storage {
    /// size in bytes of the storage region
    fn capacity(&self) -> usize;

    /// size of sectors
    fn sector_size(&self) -> usize;

    /// size of words in the storage
    /// * read/write offset must be aligned to this size
    /// * read/write buffers must sized to this granularity
    fn word_size(&self) -> WordSize;

    type StorageError;

    fn read(&mut self, offset: usize, buffer: &mut[u8]) -> Result<(), Self::StorageError>;

    fn write(&mut self, offset: usize, buffer: &[u8]) -> Result<(), Self::StorageError>;

    fn erase_sectors(&mut self, start_sector: usize, sector_count: usize) -> Result<(), Self::StorageError>;
}
pub enum WordSize {
    _8Bit   = 1,
    _16Bit  = 2,
    _32Bit  = 4,
}

// pub trait AsyncStorage {
//     type Error;

//     fn read_async(&mut self, offset: usize, buffer: &mut[u8])
//         -> impl core::future::Future<Output=Result<(), Self::Error>> + Send;

//     fn write_async(&mut self, offset: usize, buffer: &[u8])
//         -> impl core::future::Future<Output=Result<(), Self::Error>> + Send;

//     fn erase_async(&mut self, sector_start: usize, sector_count: usize)
//         -> impl core::future::Future<Output=Result<(), Self::Error>> + Send;

//     fn capacity(&self) -> usize;

//     fn sector_size(&self) -> usize;
// }

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
