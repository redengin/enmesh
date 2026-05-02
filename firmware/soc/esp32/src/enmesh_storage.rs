// provide the shared crates via re-export
use common::*;

// provide logging primitives
use log::*;

use enmesh_firmware::storage::{EnmeshStorage, Partition};
pub struct Storage<'a> {
    flash_storage: esp_storage::FlashStorage<'a>,
    pub settings_partition: Option<Partition>,
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
        let mut settings_partition: Option<Partition> = None;
        let mut data_partition: Option<Partition> = None;
        for partition in partition_table.iter() {
            match partition.label_as_str() {
                "app.settings" => {
                    debug!("found settings partition [size: {}]", partition.len());
                    settings_partition = Some(Partition {
                        address: partition.offset() as usize,
                        size: partition.len() as usize,
                        sector_size: esp_storage::FlashStorage::SECTOR_SIZE as usize,
                    });
                }
                "app.data" => {
                    debug!("found settings partition [size: {}]", partition.len());
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
            flash_storage,
            settings_partition,
            data_partition,
        }
    }
}
impl<'a> enmesh_firmware::storage::EnmeshStorage for Storage<'a> {
    fn settings_partition(&self) -> Option<Partition> {
        self.settings_partition
    }
    fn data_partition(&self) -> Option<Partition> {
        self.data_partition
    }
}

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
