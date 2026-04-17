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

[Congestion Mitigations](congestion.md)
* constrain airtime duration - by limiting LoRa transmission duration, more
        LoRa transmitters (users) are allowed to participate
    * LoRa should be expected to be a lossy channel (i.e. packet transmission aren't guaranteed)
        * when the sender/receiver requires a lossless channel - they need to leverage a sync
            protocol (if the transmission wasn't acknowleged - resend the transmission)


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




