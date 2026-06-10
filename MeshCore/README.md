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

Possibly Insecure MeshCore Designs
--------------------------------------------------------------------------------
MeshCore support of these until a design flaw is identified.
* MeshCore LoRa Protocol - the [description of the protocol](https://docs.meshcore.io/packet_format/)
  doesn't provide guidance as to how these protocols are to be used.
    * documentation doesn't specifiy what is **encrypted** vs **clear-text**
        * only the PAYLOAD_TYPE_CONTROL is identified as **clear text**
            * what this packet is used for is not described
        * for **encrypted** packets, the location of private keys used is not specified
            * if private keys must be distributed on the Mesh, MeshCore is by design
                insecure
        * there is no guidadance on which encryption method must be used
            * nor, does the packet protocol allow multiple encryption methods to
                be used - this makes evolving toward a more secure encryption method
                difficult as it requires all participants to update to the one-and-only
                supported encryption method
    * documentation doesn't specify if *tracing* is required to support the Mesh
        * for senders desiring anonymity, they shouldn't have to expose their location
            by the initial hop onto the Mesh
