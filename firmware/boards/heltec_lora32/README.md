Meshcore firmware for Heltec LoRa 32
================================================================================
Powered by the ESP32S3 MCU and SX1262 LoRa Node chips. Integrated three wireless
communication ways (LoRa, BLE, and Wi-Fi)， along with an on-board OLED display,
it presents a robust, all-in-one solution.

* [Heltec LoRa 32 (v3)](https://heltec.org/project/wifi-lora-32-v3/)
* [Heltec LoRa 32 (v4)](https://heltec.org/project/wifi-lora-32-v4/)

## Usage
```sh
# flash repeater firmware
cargo run --release
```
For debugging, don't specify "--release" - so a chip reset will not be issued
upon a panic!.

Espressif Rust (esp-rs)
================================================================================
see [Espressif Rust](https://github.com/esp-rs/awesome-esp-rust) documentation.

#### Prerequisites
* [Toolchain Installation](https://docs.espressif.com/projects/rust/book/getting-started/tooling/index.html) - required to build
* [ESP-FLASH](https://docs.espressif.com/projects/rust/book/getting-started/tooling/espflash.html) - required to flash (i.e. cargo run)
