/// support loading and persisting settings
pub trait SettingsStorage {
    fn load_settings_raw(&mut self, buffer: &mut [u8]) -> Result<usize, StorageError>;

    fn save_settings_raw(&mut self, buffer: &[u8]) -> Result<(), StorageError>;
}

pub trait AppStorage {

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

