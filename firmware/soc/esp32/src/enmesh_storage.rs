// provide the shared crates via re-export
use common::{embassy_sync::blocking_mutex::raw::RawMutex, *};

// provide logging primitives
use log::*;

// provide mutex primitives
use embassy_sync::blocking_mutex::NoopMutex;

use embedded_storage::nor_flash::{NorFlash, ReadNorFlash};
use enmesh_firmware::storage::Partition;

pub struct Storage<'a> {
    /// mutex for single executor
    flash_storage: NoopMutex<esp_storage::FlashStorage<'a>>,
    pub settings_partition_a: Option<Partition>,
    pub settings_partition_b: Option<Partition>,
    pub data_partition: Option<Partition>,
}
impl<'a> Storage<'a> {
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
                        "found {} partition [size: {}]",
                        partition.label_as_str(),
                        partition.len()
                    );
                    settings_partition_a = Some(Partition {
                        address: partition.offset() as usize,
                        size: partition.len() as usize,
                        sector_size: esp_storage::FlashStorage::SECTOR_SIZE as usize,
                    });
                }
                "app_settings.B" => {
                    debug!(
                        "found {} partition [size: {}]",
                        partition.label_as_str(),
                        partition.len()
                    );
                    settings_partition_b = Some(Partition {
                        address: partition.offset() as usize,
                        size: partition.len() as usize,
                        sector_size: esp_storage::FlashStorage::SECTOR_SIZE as usize,
                    });
                }
                "app_data" => {
                    debug!(
                        "found {} partition [size: {}]",
                        partition.label_as_str(),
                        partition.len()
                    );
                    data_partition = Some(Partition {
                        address: partition.offset() as usize,
                        size: partition.len() as usize,
                        sector_size: esp_storage::FlashStorage::SECTOR_SIZE as usize,
                    });
                }
                label => trace!("ignoring '{label}' partition"),
            }
        }

        Self {
            flash_storage: NoopMutex::new(flash_storage),
            settings_partition_a,
            settings_partition_b,
            data_partition,
        }
    }

    pub fn read(
        &mut self,
        address: usize,
        buffer: &mut [u8],
    ) -> Result<(), esp_storage::FlashStorageError> {
        // SAFETY: this is safe, see embassy_sync::blocking_mutex::lock_mut() for details
        unsafe {
            self.flash_storage
                .lock_mut(|flash_storage| flash_storage.read(address as u32, buffer))
        }
    }

    /// will merge data, erase the sector, and then write the merged data
    pub fn write(
        &mut self,
        address: usize,
        buffer: &[u8],
    ) -> Result<(), esp_storage::FlashStorageError> {
        // SAFETY: this is safe, see embassy_sync::blocking_mutex::lock_mut() for details
        unsafe {
            self.flash_storage.lock_mut(|flash_storage| {
                flash_storage.write(address as u32, buffer)
            })
        }
    }

    pub fn erase(
        &mut self,
        address: usize,
        size: usize,
    ) -> Result<(), esp_storage::FlashStorageError> {
        // SAFETY: this is safe, see embassy_sync::blocking_mutex::lock_mut() for details
        unsafe {
            self.flash_storage.lock_mut(|flash_storage| {
                flash_storage.erase(address as u32, (address + size) as u32)
            })
        }
    }
}

// impl<'a> enmesh_firmware::storage::EnmeshStorage for Storage<'a> {
//     fn settings_partition(&self) -> Option<Partition> {
//         self.settings_partition
//     }
//     fn data_partition(&self) -> Option<Partition> {
//         self.data_partition
//     }
// }

// impl<'a> EnmeshStorage for Storage<'a> {

// }

//     /// returns:
//     ///     OK(count of bytes written to buffer)
//     ///     Err(count of bytes written to buffer)
//     pub async fn load_settings_raw(&mut self, buffer: &mut [u8]) -> Result<usize, usize> {
//         if let Some(partition) = &self.settings_partition {
//             match self.flash_storage.read(partition.flash_offset, buffer) {
//                 Ok(_) => Ok(buffer.len()),
//                 Err(_) => Err(0),
//             }
//         } else {
//             // no partition
//             return Err(0);
//         }
//     }

//     /// NOTE: this will erase flash sectors before writing
//     /// returns:
//     ///     OK(count of bytes written to flash)
//     ///     Err(count of bytes written to flash)
//     pub async fn save_settings_raw(&mut self, buffer: &[u8]) -> Result<usize, usize> {
//         if let Some(partition) = &self.settings_partition {
//             match self.flash_storage.write(partition.flash_offset, buffer) {
//                 Ok(_) => Ok(buffer.len()),
//                 Err(_) => Err(0),
//             }
//         } else {
//             // no partition
//             return Err(0);
//         }
//     }
// }
