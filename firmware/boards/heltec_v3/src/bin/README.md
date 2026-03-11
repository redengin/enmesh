Firmware Images
================================================================================
[repeater](repeater.rs) - LoRa packet repeater
* support MeshCore, Meshtastic
    * (option) support both simultaneously
        * equal time for receive - lossy due to LoRa RF parameter difference
            between the two
* support enmesh bridge
    * use the SoC WiFi hardware to connect to a local WiFi router
        * coordinate with enmesh endpoints to bridge LoRa traffic

[companion](companion.rs) - stores and sends LoRa packets
* support MeshCore, Meshtastic
    * (option) support both simultaneously
        * equal time for receive - lossy due to LoRa RF parameter difference
            between the two
* support enmesh bridge
    * use the SoC WiFi hardware to connect to a local WiFi router
        * coordinate with enmesh endpoints to bridge LoRa traffic


Novel Firmware
--------------------------------------------------------------------------------
The hardware is much more capable than the MeshCore role design.

It's possible for a node to provide
* multiple MeshCore rooms
* channel bot
* and more...