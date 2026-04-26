


UX Examples
================================================================================
uses embedded-graphics-simulator which requires SDL2
```sh
sudo apt install libsdl2-dev
```

Running UX Examples
--------------------------------------------------------------------------------
```sh
# generic display
cargo run --release --example ux
# heltec t114 emulation
cargo run --release --example ux --features ux-heltec
```