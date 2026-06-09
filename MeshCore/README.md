MeshCore support (using Rust)
================================================================================
Where MeshCore design is insecure, work on those features will not be maintained.

Insecure MeshCore Designs
--------------------------------------------------------------------------------
* BLE Companion - will not be supported as it is insecure and inefficient
    * places private secrets onto remote devices, allowing those secrets to be
      exposed by physical access and possibly software exploits.
        * private keys should only be stored on the companion device
    * requires the remote device to encrypt/decrypt
        * this is inefficient as the companion device has greater compute
            resources available to perform the encryption/decryption
