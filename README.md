Secure distributed communications enhanced by LoRa anonymity
================================================================================
Objectives
--------------------------------------------------------------------------------
* Communication Access
  - LoRa protocols that are robust to denial of service attacks
* Anonymous Communication
  - sender's can communicate freely by making the sender's location difficult to
      locate
    * whistleblowers can share information
    * users can overcome censorship to share information

Background
--------------------------------------------------------------------------------
[LoRa](https://en.wikipedia.org/wiki/LoRa) use has proliferated under
[Meshtastic](https://meshtastic.org/) and [MeshCore](https://meshcore.co.uk/)
creating LoRa hardware that is readily purchasable by users.

[Reticulum](https://reticulum.network/) is a proposed design for creating
"sovereign" communication networks that bridge LoRa traffic over additional
channels (e.g. internet).

### LoRa Protocol Evolution
While novel protocols have increased the usage of LoRa, LoRa needs to evolve to
meet the demands of users. The [IETF](https://www.ietf.org/) has managed the
evolution of the internet and should be used to evolve LoRa.
([see RFCs](https://github.com/redengin/enmesh/wiki/RFC))

Current LoRa protocols designs are insufficient to meet the objectives of EnMesh.
Rather than EnMesh become yet-another-protocol, the hope is that current
LoRa mesh protocols evolve via the RFC's.

What is EnMesh?
================================================================================
EnMesh is a [Rust](https://rust-lang.org/) implementation for distributed
communication using LoRa platforms. The EnMesh architecture allows simple
adaptation of LoRa platforms leveraging the power of Rust (i.e. to support new
LoRa platforms, one only needs to implement a simple interface layer to leverage
EnMesh).

EnMesh firmware supports multiple LoRa protocols (i.e. Meshtastic, MeshCore).
Even supporting multiple LoRa protocols simultaneously by
[time-division multiplexing](https://en.wikipedia.org/wiki/Time-division_multiplexing).

EnMesh firmware supports access via [BLE](https://en.wikipedia.org/wiki/Bluetooth_Low_Energy)
per the novel protocols.

EnMesh firmware supports [MQTT](https://en.wikipedia.org/wiki/MQTT) as a bridge
over [WiFi](https://en.wikipedia.org/wiki/Wi-Fi) - supporting services like
[letsmesh.net](LetsMesh.net).

Yet-Another Protocol?
---------------------------------------------------------------------------------
The hope is that popular Mesh LoRa designs will refactor per the accepted RFC
designs. 

To support the evolution toward RFC based design, EnMesh will provide the rust
implementation for novel protocols per the RFC. If novel protocol maintainers
fail to adhere to RFC, then EnMesh will create yet-another-protocol per the
RFC design.




Repository Overview
================================================================================
* [Firmware](firmware) - board support for common hardware
  - [Hardware](firmware/boards) - implemenations of enmesh for common hardware
    * [Heltec-T114](firmware/boards/heltec_t114/)
* [MeshTastic Library](MeshCore) - Rust implementation of MeshCore
  * IN-WORK: MeshCore rapidly evolves, this should become a separate repo
* [MeshCore Library](MeshCore) - Rust implementation of MeshCore
  * IN-WORK: Meshtastic rapidly evolves, this should become a separate repo


<!-- Enmesh Physical Architecture
--------------------------------------------------------------------------------
```mermaid
C4Component
    Container_Boundary(lora, "LoRa") {
      Component(repeater, "repeater")
      Component(wifi_repeater, "repeater <br> w/ WiFi")
      Component(companion, "companion")
    }
    BiRel(companion, repeater, "send/receive <br> LoRa packets")
    BiRel(companion, wifi_repeater, "send receive <br> LoRa packets")
    BiRel(wifi_repeater, wifi_router, "bridges local <br> LoRa to internet")
    BiRel(wifi_router, enmesh_endpoint, "bridges local <br> LoRa to internet")
    Container_Boundary(internet, "Internet") {
      Component(wifi_router, "wifi router")
      Component(enmesh_endpoint, "enmesh endpoint")
    }
    Container_Boundary(mobile_app, "Mobile App") {
      Component(mobile_app, "Mobile App")
    }
    BiRel(mobile_app, companion, "send/receive <br> LoRa packets")
    BiRel(mobile_app, enmesh_endpoint, "bridges local <br> LoRa to internet")

  UpdateLayoutConfig($c4ShapeInRow="2", $c4BoundaryInRow="2")
``` -->
<!--
* Secured by encryption
  * MeshCore - [uses asymmetric cryptography](https://en.wikipedia.org/wiki/Public-key_cryptography)
      (public_key, private_key)
    * public channels (#\<channel>) use a known private key
      * as the private key is known, anyone can create a message
    * user-to-user channels are more secure as only the public keys are necessary
      * sharing public keys
        * advert - publish your public key to the mesh
        * socially - provide your public key to the other user
  * Meshtastic - [uses symmetric cryptography](https://en.wikipedia.org/wiki/Symmetric-key_algorithm)
      (single key)
    * each channel uses a key
      * there is one common channel with a known key
    * users must have the channel key to transmit/receive messages 
      * sharing channel keys
        * socially - provide the channel key to the other user
  * Enmesh - [uses asymmetric cryptography](https://en.wikipedia.org/wiki/Public-key_cryptography)
      (public_key, private_key)
      * messages exchanged between an `enmesh endpoint` (internet service) and the `node` (local device)
          are encrypted by an exchange of public keys upon connection (like ssh)
        * as the exchange doesn't expose the private keys, exchanges between an `enmesh endpoint`
            and `node` are secured.
-->
<!-- 
Universal LoRa Communication
--------------------------------------------------------------------------------
Only a few local LoRa nodes need to support a bridge to support anonymous 
universal messaging via LoRa.


* Yet Another LoRa Protocol?  [see RFCs](https://github.com/redengin/enmesh/wiki/RFC)
  * Rather than attempt to replace current protocols (Meshtastic and MeshCore),
    enmesh will support both.
  * if the current protocols don't adapt to Enmesh objectives, Enmesh will
    provide it's own LoRa protocol

The [enmesh design](docs/design.md) describes how enmesh nodes connect local
LoRa traffic (Meshtastic/MeshCore) to the world.
-->

