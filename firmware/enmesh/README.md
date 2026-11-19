Enmesh Board Agnostic Implementation
================================================================================
The Enmesh architecture requires a minimal BSP support layer to run.

### See what Enmesh firmware looks like (without needing hardware)
You can run a demonstration of the Enmesh UX from the 'enmesh/firmware/enmesh'
folder.

#### Prerequisites
The demonstration draws the display in a desktop window. It needs the SDL2
development library.

Debian and Ubuntu:
```sh
sudo apt install libsdl2-dev
```

Without this library, the build fails with `unable to find library -lSDL2`.
See the [simulator setup](https://github.com/embedded-graphics/simulator#setup)
for other platforms.

#### Small Mono Display (128x64)
```sh
cargo run --example binarycolor_ux
```

#### Large Mono Display (250x122)
```sh
cargo run --example binarycolor_ux --features=example_ux-large
```
