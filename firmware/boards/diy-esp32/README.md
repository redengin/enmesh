Meshcore firmware for DIY ESP32 based device
================================================================================
This is a a template for creating ESP32 based devices. Users will need to
* map pins to peripherals
* configure services per the hardware implementation
    * for BLE, if there is no screen a statically configured PIN Code will be used.
    * intitialize a LORA device driver for the chosen device

## Usage
```sh
# build and flash
# 'cargo <esp32 type> --bin <role>'
# * esp32 type: [esp32, esp32s3]
# * role: [companion, repeater, room]
cargo esp32 --bin companion
```

Espressif Rust (esp-rs)
================================================================================
see [Espressif Rust](https://github.com/esp-rs/awesome-esp-rust) documentation.

#### Prerequisites
* [Toolchain Installation](https://docs.espressif.com/projects/rust/book/getting-started/tooling/index.html) - required to build
* [ESP-FLASH](https://docs.espressif.com/projects/rust/book/getting-started/tooling/espflash.html) - required to flash (i.e. cargo run)
