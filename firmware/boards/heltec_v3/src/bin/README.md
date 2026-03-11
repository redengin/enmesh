Firmware Images
================================================================================
[repeater](repeater.js)
* support MeshCore, Meshtastic
    * (option) support both simultaneously
        * equal time for receive - lossy due to LoRa RF parameter difference
            between the two
* support enmesh bridge
    * use the SoC WiFi hardware to connect to a local WiFi router
        * coordinate with enmesh endpoints to bridge LoRa traffic