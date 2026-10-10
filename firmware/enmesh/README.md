Enmesh Board Agnostic Implementation
================================================================================
The Enmesh architecture requires a minimal BSP support layer to run.

### See what Enmesh firmware looks like (without needing hardware)
You can run a demonstration of the Enmesh UX from the 'enmesh/firmware/enmesh'
folder.

#### Small Mono Display (128x64)
```sh
cargo run --example binarycolor_ux
```

#### Large Mono Display (250x122)
```sh
cargo run --example binarycolor_ux --features=example_ux-large
```
