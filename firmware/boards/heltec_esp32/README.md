Enmesh firmware for Heltec ESP32 platforms
================================================================================

### Prerequisites
* Espressif Rust (esp-rs) - see [Espressif Rust](https://github.com/esp-rs/awesome-esp-rust) documentation.
    * [Toolchain Installation](https://docs.espressif.com/projects/rust/book/getting-started/tooling/index.html) - required to build
    * [ESP-FLASH](https://docs.espressif.com/projects/rust/book/getting-started/tooling/espflash.html) - required to flash (i.e. cargo run)

## Usage
see [Cargo.toml](Cargo.toml) "features" for platform names

### Flash the release version
The release version reboots the board upon panic!
```sh
cargo run --release --features="<platform name>"
```

### Flash the debug version
```sh
ESP_LOG=debug cargo run --features="<platform name>"
```
ESP_LOG should follow [RUST_LOG conventions](https://docs.rs/env_logger/latest/env_logger/#enabling-logging).