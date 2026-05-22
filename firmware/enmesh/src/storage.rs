pub trait Storage {
    /// size in bytes of the storage region
    fn capacity(&self) -> usize;

    /// size of sectors (in bytes)
    fn sector_size(&self) -> usize;

    /// size of words in the storage
    /// * read/write offset must be aligned to this size
    /// * read/write buffers must sized to this granularity
    fn word_size(&self) -> WordSize;

    type StorageError: core::fmt::Debug;

    /// * offset - must be aligned to word_size()
    /// * buffer - must be sized to N*word_size()
    fn read(&mut self, offset: usize, buffer: &mut [u8]) -> Result<(), Self::StorageError>;

    /// * offset - must be aligned to word_size()
    /// * buffer - must be sized to N*word_size()
    fn write(&mut self, offset: usize, buffer: &[u8]) -> Result<(), Self::StorageError>;

    fn erase_sectors(
        &mut self,
        start_sector: usize,
        sector_count: usize,
    ) -> Result<(), Self::StorageError>;

}

#[derive(Clone, Copy)]
pub enum WordSize {
    _8Bit = 1,
    _16Bit = 2,
    _32Bit = 4,
}

pub mod utils {
    use crate::storage::WordSize;

    /// determine the size of the buffer based upon word_size
    pub fn buffer_size(atleast_size: usize, word_size: WordSize) -> usize {
        let word_count = atleast_size.div_ceil(word_size as usize);
        return word_count * (word_size as usize);
    }

    /// determine the number of sectors bassed up size
    pub fn sector_count(atleast_size: usize, sector_size: usize) -> usize {
        return atleast_size.div_ceil(sector_size);
    }
}

// TESTING
//--------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_utils_buffer_size() {
        const ATLEAST_SIZE: usize = 1;
        assert_eq!(WordSize::_8Bit as usize, utils::buffer_size(ATLEAST_SIZE, WordSize::_8Bit), "8 bit failed");
        assert_eq!(WordSize::_16Bit as usize, utils::buffer_size(ATLEAST_SIZE, WordSize::_16Bit), "16 bit failed");
        assert_eq!(WordSize::_32Bit as usize, utils::buffer_size(ATLEAST_SIZE, WordSize::_32Bit), "32 bit failed");
    }

    fn test_utils_sector_count() {
        {
            const ATLEAST_SIZE: usize = 1;
            const SECTOR_SIZE: usize = 100;
            assert_eq!(1, utils::sector_count(ATLEAST_SIZE, SECTOR_SIZE));
        }
        {
            const ATLEAST_SIZE: usize = 1000;
            const SECTOR_SIZE: usize = 10;
            assert_eq!(100, utils::sector_count(ATLEAST_SIZE, SECTOR_SIZE));
        }
    }
}

// pub trait AsyncStorage {
//     type StorageError: core::fmt::Debug;

//     fn read_async(&mut self, offset: usize, buffer: &mut[u8])
//         -> impl core::future::Future<Output=Result<(), Self::Error>> + Send;

//     fn write_async(&mut self, offset: usize, buffer: &[u8])
//         -> impl core::future::Future<Output=Result<(), Self::Error>> + Send;

//     fn erase_async(&mut self, sector_start: usize, sector_count: usize)
//         -> impl core::future::Future<Output=Result<(), Self::Error>> + Send;

//     fn capacity(&self) -> usize;

//     fn sector_size(&self) -> usize;
// }

// /// generic trait for persistables
// ///
// /// Persitables have no need to support functionality like roll-back.
// /// * implementations **shall**
// ///     * be able to verify the integrity of the persitable
// ///         - e.g. CRC, ECC, etc.
// ///     * be able to identify the version of the persistable
// ///     * convert the persistable to the current version
// ///         - invoking a store() upon conversion
// /// * implementations *should*
// ///     * manage multiple persisted copies for robustness
// pub trait Persistable {
//     type Item;

//     fn load() -> Option<Self::Item>;

//     /// update all persistable's copies
//     fn store(settings: &Self::Item) -> Result<(), crate::storage::StorageError>;
// }
