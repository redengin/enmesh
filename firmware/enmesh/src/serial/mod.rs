// provide the common crates via re-export
use common::*;

// provide logging primitives
use log::*;
const TAG: &str = "[Serial Console]";

/// provide scheduling primitives
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::rwlock::RwLock;

pub async fn run(
    global_state: &'static RwLock<NoopRawMutex, crate::State>,
    mut serial: impl Serial,
) {
    info!("{TAG} started. You can now send configuration commands");

    let mut mode = SessionMode::Enmesh;
    let mut command: heapless::String<80> = heapless::String::new();

    loop {
        let mut buffer = [0u8; 1];
        if let Ok(read) = serial.read_async(&mut buffer).await {
            if read > 0 {
                match buffer[0] {
                    b'\r' => {
                        // upon 'carriage return' submit the command
                        const NEWLINE: [u8; 1] = [b'\n'];
                        let _ = serial.write_async(&NEWLINE).await;
                        match handle_command(&mut mode, global_state, command.as_str()).await {
                            Ok(s) => {
                                if let Some(message) = s {
                                    let _ = serial.write_async(message.as_bytes()).await;
                                }
                            }
                            Err(e) => {
                                let _ = serial.write_async(e.as_bytes()).await;
                            }
                        }

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
                        // upon 'escape' eat the escaped codes
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
                        } else {
                            // echo the character back to the serial console
                            let _ = serial.write_async(&buffer).await;
                        }
                    }
                }
            }
        }
    }
}


mod meshcore_bindings;

async fn handle_command<'a>(
    mode: &mut SessionMode,
    global_state: &'static RwLock<NoopRawMutex, crate::State>,
    command: &'a str,
) -> Result<Option<heapless::String<80>>, &'a str> {
    debug!("{TAG} handling command: '{command}'");

    if command == "exit" {
        // return to enmesh mode
        *mode = SessionMode::Enmesh;
        return Ok(None);
    }

    match mode {
        SessionMode::Enmesh => {
            if command == "meshcore" || command == "mc" {
                // enter meshcore mode
                *mode = SessionMode::MeshCore;
                return Ok(None);
            }

            if command == "meshtastic" || command == "mt" {
                // enter meshtastic mode
                *mode = SessionMode::Meshtastic;
                return Ok(None);
            }

            // TODO handle enmesh commands
        }

        SessionMode::MeshCore => {
            match meshcore::cli::CliCommands::from_string(command) {
                Ok(cli_command) => {
                    match meshcore_bindings::handle(global_state, cli_command).await {
                        Ok(m) => {
                            if let Some(message) = m {
                                return Ok(Some(message))
                            }
                            else {
                                return Ok(None)
                            }
                        }
                        Err(message) => return Err(message)
                    }
                }
                Err(message) => return Err(message)
            }
        }

        SessionMode::Meshtastic => {
            // TODO handle meshstastic commands
        }
    }
    Err("not implemented")
}

enum SessionMode {
    Enmesh,
    MeshCore,
    Meshtastic,
}
impl SessionMode {
    pub fn prompt(&self) -> &'static [u8] {
        return match self {
            Self::Enmesh => "\n\nenmesh> ".as_bytes(),
            Self::MeshCore => "\n\nmeshcore> ".as_bytes(),
            Self::Meshtastic => "\n\nmeshtastic> ".as_bytes(),
        };
    }
}

pub trait Serial {
    #![allow(async_fn_in_trait)]

    type RxError;
    type TxError;

    async fn read_async(&mut self, buffer: &mut [u8]) -> Result<usize, Self::RxError>;

    async fn write_async(&mut self, buffer: &[u8]) -> Result<usize, Self::TxError>;
}
