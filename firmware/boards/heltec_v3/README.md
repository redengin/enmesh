Meshcore firmware for Heltec v3
================================================================================
[Heltec LoRa 32 (v3)](https://heltec.org/project/wifi-lora-32-v3/)

## Usage
```sh
# flash repeater firmware
cargo run --release --bin repeater
```
For debugging, don't specify "--release" - so a chip reset will not be issued
upon a panic!.

Espressif Rust (esp-rs)
================================================================================
see [Espressif Rust](https://github.com/esp-rs/awesome-esp-rust) documentation.

#### Prerequisites
* [Toolchain Installation](https://docs.espressif.com/projects/rust/book/getting-started/tooling/index.html) - required to build
* [ESP-FLASH](https://docs.espressif.com/projects/rust/book/getting-started/tooling/espflash.html) - required to flash (i.e. cargo run)
