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

    let mut mode = SessionMode::Enmesh;
    let mut command: heapless::String<80> = heapless::String::new();

    const NEWLINE: [u8; 1] = [b'\n'];
    loop {
        let mut buffer = [0u8; 1];
        if let Ok(read) = serial.read_async(&mut buffer).await {
            if read > 0 {
                match buffer[0] {
                    b'\r' => {
                        // upon 'carriage return' submit the command
                        debug!("{TAG} command: '{command}'");
                        // TODO

                        // clear the command buffer
                        command.clear();

                        // send new prompt to the console
                        let _ = serial.write_async(mode.prompt()).await;
                    }
                    b'\x08' => {
                        // upon backspace drop the last char
                        if command.pop().is_some() {
                            // clear the character from the serial console
                            const BACKSPACE_SEQUENCE: [u8; 3] = [b'\x08', b' ', b'\x08'];
                            let _ = serial.write_async(&BACKSPACE_SEQUENCE).await;
                        }
                    }
                    b'\x1B' => {
                        // upon 'escape' eat the escaped chars
                        warn!("{TAG} escape codes not supported");
                        let mut buffer: [u8; 5] = [0u8; _];
                        let _ = serial.read_async(&mut buffer).await;

                        // redisplay the command
                        let _ = serial.write_async(mode.prompt()).await;
                        let _ = serial.write_async(command.as_bytes()).await;
                    }
                    _ => {
                        if command.push(buffer[0] as char).is_err() {
                            debug!("{TAG} command buffer overflow");
                        }
                        else {
                            // echo the character back to the serial console
                            let _ = serial.write_async(&buffer).await;
                        }
                    }
                }
            }
        }
    }
}

enum SessionMode {
    Enmesh,
    MeshCore,
    Meshtastic,
}
impl SessionMode {
    pub fn prompt(&self) -> &'static [u8]
    {
        return match self
        {
            Self::Enmesh => "\nenmesh> ".as_bytes(),
            Self::MeshCore => "\nmeshcore> ".as_bytes(),
            Self::Meshtastic=> "\nmeshtastic> ".as_bytes(),
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

