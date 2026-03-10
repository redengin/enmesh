// provide the shared crates via re-export
use common::*;

// provide logging primitives
use log::*;

// use soc crate (it provides the panic handler)
use soc_esp32::*;

// use embedded-storage
use embedded_storage::{ReadStorage, Storage};

struct Partition {
    flash_offset: u32,
    size: u32,
}
pub(crate) struct AppPartitions<'a> {
    flash_storage: esp_storage::FlashStorage<'a>,
    settings_partition: Option<Partition>,
    data_partition: Option<Partition>,
}

impl AppPartitions<'_> {
    pub(crate) fn new(flash: esp_hal::peripherals::FLASH<'static>) -> Self {
        // get the partition table
        let mut flash_storage = esp_storage::FlashStorage::new(flash);
        let mut buffer = [0u8; esp_bootloader_esp_idf::partitions::PARTITION_TABLE_MAX_LEN];
        let partition_table = esp_bootloader_esp_idf::partitions::read_partition_table(
            &mut flash_storage,
            &mut buffer,
        )
        .unwrap();

        // find the settings partition
        let mut settings_partition: Option<Partition> = None;
        let mut data_partition: Option<Partition> = None;
        for partition in partition_table.iter() {
            match partition.label_as_str() {
                "app.settings" => {
                    debug!("found settings partition [size: {}]", partition.len());
                    settings_partition = Some(Partition {
                        flash_offset: partition.offset(),
                        size: partition.len(),
                    });
                }
                "app.data" => {
                    debug!("found settings partition [size: {}]", partition.len());
                    data_partition = Some(Partition {
                        flash_offset: partition.offset(),
                        size: partition.len(),
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

    /// returns:
    ///     OK(count of bytes written to buffer)
    ///     Err(count of bytes written to buffer)
    pub async fn load_settings_raw(&mut self, buffer: &mut [u8]) -> Result<usize, usize> {
        if let Some(partition) = &self.settings_partition {
            match self.flash_storage.read(partition.flash_offset, buffer) {
                Ok(_) => Ok(buffer.len()),
                Err(_) => Err(0),
            }
        } else {
            // no partition
            return Err(0);
        }
    }

    /// NOTE: this will erase flash sectors before writing
    /// returns:
    ///     OK(count of bytes written to flash)
    ///     Err(count of bytes written to flash)
    pub async fn save_settings_raw(&mut self, buffer: &[u8]) -> Result<usize, usize> {
        if let Some(partition) = &self.settings_partition {
            match self.flash_storage.write(partition.flash_offset, buffer) {
                Ok(_) => Ok(buffer.len()),
                Err(_) => Err(0),
            }
        } else {
            // no partition
            return Err(0);
        }
    }
}

// TODO implement enmesh traits for settings load/save
// TODO implement enmesh traits for data partition
