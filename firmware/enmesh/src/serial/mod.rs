// provide the common crates via re-export
use common::*;

// provide logging primitives
use log::*;
const TAG: &str = "[Serial Console]";

/// provide scheduling primitives
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::rwlock::RwLock;

pub async fn run(
    _global_state: &'static RwLock<NoopRawMutex, crate::State>,
    mut serial: impl Serial
) {
    info!("{TAG} started. You can now send configuration commands");

    // FIXME simple echo for now
    loop {
        let mut buffer = [0u8; 1];
        if let Ok(read) = serial.read_async(&mut buffer).await {
            if read > 0 {
                // echo
                let _ = serial.write_async(&buffer[0..read]).await;
                // add a newline for \r
                if buffer[read - 1] == '\r' as u8 {
                    const NEWLINE: u8 = '\n' as u8;
                    buffer[0] = NEWLINE;
                    let _ = serial.write_async(&buffer[0..read]).await;
                    warn!("{TAG} no comands implemented yet");
                }
            }
        }
    }
}

pub trait Serial {
    #![allow(async_fn_in_trait)]

    type RxError;
    type TxError;

    async fn read_async(&mut self, buffer: &mut [u8]) -> Result<usize, Self::RxError>;

    async fn write_async(&mut self, buffer: &[u8]) -> Result<usize, Self::TxError>;
}



