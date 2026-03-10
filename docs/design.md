Enmesh Design
================================================================================
Background
--------------------------------------------------------------------------------
Both Meshtastic and MeshCore are evolving to manage `LoRa congestion`.

`LoRa congestion` occurs when too many devices transmit simultaneously,
leading to packet collisions, high packet error rates, and increased latency.

Usage of LoRa is exponentially increasing - expect congestion.

It is uncertain whether Meshtastic/Meshcore can mitigate congestion.

Enmesh LoRa 
================================================================================
Should Meshtastic/Meschore designs fail to mitigate congestion,
enmesh provides an alternative that does mitigate congestion.

Congestion Mitigations
* leverage [NFC standards](https://www.iso.org/standard/82095.html) - to choose
    which sender to listen to


Bridging LoRa traffic - Universal LoRa Communication
================================================================================
The enmesh bridge connects LoRa traffic via the internet.

Enmesh provides universal LoRa communication (both Meshtastic and MeshCore).

### LoRa Congestion
Universal LoRa communications increases `LoRa congestion`.

To mitigate `LoRa congestion` enmesh nodes:
* filter by sender's location
    * configurable set of supported locations
        * only white-listed locations will be forwarded locally
* filter by channel type
    * public channel
        * never forward, as public channels are designed to be used by locals




