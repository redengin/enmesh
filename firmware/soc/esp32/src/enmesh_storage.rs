// provide the shared crates via re-export
use common::*;

// provide logging primitives
use log::*;
const TAG: &str = "[enmesh_storage]";

pub struct EnmeshStorage {
    settings_partition_a: Option<Partition>,
    settings_partition_b: Option<Partition>,
    data_partition: Option<Partition>,
}
impl enmesh_firmware::storage::EnmeshStorage for EnmeshStorage {
    fn settings_partition_a(&self) -> Option<&impl enmesh_firmware::storage::Storage> {
        self.settings_partition_a.as_ref()
    }

    fn settings_partition_b(&self) -> Option<&impl enmesh_firmware::storage::Storage> {
        self.settings_partition_b.as_ref()
    }

    fn data_partition(&self) -> Option<&impl enmesh_firmware::storage::Storage> {
        self.data_partition.as_ref()
    }
}
impl EnmeshStorage {
    pub fn open(flash: esp_hal::peripherals::FLASH<'static>) -> Self {
        // get the partition table
        let mut flash_storage = esp_storage::FlashStorage::new(flash);
        let mut buffer = [0u8; esp_bootloader_esp_idf::partitions::PARTITION_TABLE_MAX_LEN];
        let partition_table = esp_bootloader_esp_idf::partitions::read_partition_table(
            &mut flash_storage,
            &mut buffer,
        )
        .unwrap();

        // find the enmesh partitions
        let mut settings_partition_a: Option<Partition> = None;
        let mut settings_partition_b: Option<Partition> = None;
        let mut data_partition: Option<Partition> = None;
        for partition in partition_table.iter() {
            match partition.label_as_str() {
                "app_settings.A" => {
                    debug!(
                        "{TAG} found {} partition [size: {}]",
                        partition.label_as_str(),
                        partition.len()
                    );
                    settings_partition_a = Some(Partition {
                        address: partition.offset() as usize,
                        capacity: partition.len() as usize,
                    });
                }
                "app_settings.B" => {
                    debug!(
                        "{TAG} found {} partition [size: {}]",
                        partition.label_as_str(),
                        partition.len()
                    );
                    settings_partition_b = Some(Partition {
                        address: partition.offset() as usize,
                        capacity: partition.len() as usize,
                    });
                }
                "app_data" => {
                    debug!(
                        "{TAG} found {} partition [size: {}]",
                        partition.label_as_str(),
                        partition.len()
                    );
                    data_partition = Some(Partition {
                        address: partition.offset() as usize,
                        capacity: partition.len() as usize,
                    });
                }
                label => trace!("{TAG} ignoring '{label}' partition"),
            }
        }

        Self {
            settings_partition_a,
            settings_partition_b,
            data_partition,
        }
    }
}

// #[derive(Copy, Clone)]
struct Partition {
    pub address: usize,
    pub capacity: usize,
}

// create a mutex for accessing flash storage
use embassy_sync::blocking_mutex::raw::{CriticalSectionRawMutex, RawMutex};
static FLASH_MUTEX: CriticalSectionRawMutex = CriticalSectionRawMutex::new();

impl enmesh_firmware::storage::Storage for Partition {
    fn capacity(&self) -> usize {
        self.capacity
    }

    fn sector_size(&self) -> usize {
        esp_storage::FlashStorage::SECTOR_SIZE as usize
    }

    fn word_size(&self) -> enmesh_firmware::storage::WordSize {
        enmesh_firmware::storage::WordSize::_32Bit
    }

    type StorageError = esp_storage::FlashStorageError;

    fn read(&mut self, offset: usize, buffer: &mut [u8]) -> Result<(), Self::StorageError> {
        if offset + buffer.len() > self.capacity {
            return Err(esp_storage::FlashStorageError::OutOfBounds);
        }
        if (offset % self.word_size() as usize) != 0 {
            return Err(esp_storage::FlashStorageError::NotAligned);
        }
        if (buffer.len() % self.word_size() as usize) != 0 {
            return Err(esp_storage::FlashStorageError::NotAligned);
        }
        unsafe {
            debug!(
                "{TAG} reading flash at offset {} into buffer of size {}",
                (self.address + offset),
                buffer.len()
            );
            FLASH_MUTEX.lock(|| {
                let _ = esp_storage::ll::spiflash_read(
                    (self.address + offset) as u32,
                    buffer.as_mut_ptr() as *mut u32,
                    buffer.len() as u32,
                );
            });
        }
        Ok(())
    }

    fn write(&mut self, offset: usize, buffer: &[u8]) -> Result<(), Self::StorageError> {
        if offset + buffer.len() > self.capacity {
            return Err(esp_storage::FlashStorageError::OutOfBounds);
        }
        if (offset % self.word_size() as usize) != 0 {
            return Err(esp_storage::FlashStorageError::NotAligned);
        }
        if (buffer.len() % self.word_size() as usize) != 0 {
            return Err(esp_storage::FlashStorageError::NotAligned);
        }
        unsafe {
            debug!(
                "{TAG} writing flash at offset {} into buffer of size {}",
                (self.address + offset),
                buffer.len()
            );
            FLASH_MUTEX.lock(|| {
                let _ = esp_storage::ll::spiflash_write(
                    (self.address + offset) as u32,
                    buffer.as_ptr() as *mut u32,
                    buffer.len() as u32,
                );
            });
        }
        Ok(())
    }

    fn erase_sectors(
        &mut self,
        start_sector: usize,
        sector_count: usize,
    ) -> Result<(), Self::StorageError> {
        if (start_sector * self.sector_size()) > self.capacity {
            return Err(esp_storage::FlashStorageError::OutOfBounds);
        }
        if (start_sector + sector_count) * self.sector_size() > self.capacity {
            return Err(esp_storage::FlashStorageError::OutOfBounds);
        }
        let first_sector = (self.address / self.sector_size()) + start_sector;
        let last_sector = first_sector + sector_count;
        unsafe {
            FLASH_MUTEX.lock(|| {
                for sector in first_sector..last_sector {
                    let _ = esp_storage::ll::spiflash_erase_sector(sector as u32);
                }
            });
        }
        Ok(())
    }
}
