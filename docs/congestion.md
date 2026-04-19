LoRa Congestion
================================================================================
Background
--------------------------------------------------------------------------------
* [LoRa in high-density environments](https://meshtastic.org/blog/why-your-mesh-should-switch-from-longfast/)

What is a LoRa Channel? [^1]
* frequency - RF frequency band used (controlled by regulation)
* bandwidth - `chirp rate` (controlled by regulation)
    * higher bandwidth increases `data rate`
    * lower bandwidth decreases `data rate`
* spreading factor - ratio of `chirp rate` to `symbol rate`
    * higher spreading factors increase the likelihood of receivers to pick up
        the transmission but result in a lower `data rate`
* coding rate - manages the additional overhead of error-correction (ECC)
    * incresing the amount of error-correction increases the likelihood that
        the reciever will be able to process the packet
    * named by ratio of `data per code-word size`
        * 4_5 : found data bits and 1 additional ECC bit
        * 4_6 : found data bits and 2 additional ECC bits
        * 4_7 : found data bits and 3 additional ECC bits
        * 4_8 : found data bits and 4 additional ECC bits

Dense Environments
--------------------------------------------------------------------------------
As LoRa usage is increasing exponentially under Meshtastic and Meshcore, in high
population density areas, there can be lots of devices all sharing a LoRa
channel.

While the LoRa standard identifies how to handle contention for the channel
(i.e. allowing only one device to transmit on the channel at a time) in dense
environments, the contention can lead to long delays before a device is allowed
to transmit.

Enmesh LoRa Radio Congestion Mitigation
================================================================================
Both Meshtastic and MeshCore are working on congestion mitigation. In the
meantime, Enmesh uses the following.

Enmesh Airtime Mitigation
--------------------------------------------------------------------------------
The primary way to manage congestion is to limit device transmit time
* by limiting transmit time - more on-air time is available to other users
    desiring to transmit on the mesh
* while this introduces additional delay into the mesh, the objective to serve
    many users outweighs the cost of this delay

Local communities can enforce local `airtime` constraints by reaching out to users
who are maintaining devices that exceeding the `airtime`.

Enmesh Dynamic Transmit Power Mitigation
--------------------------------------------------------------------------------
By lowering the transmit power, the packets will hit fewer repeaters and reduce
the number of unnecessarily repeated packets on the mesh.
* reducing the airtime necessary for repeaters

Enmesh monitors the signal quality (RSSI/SNR) of the received packets. In the
transmit phase, transmit power is scaled toward the lowest signal quality
node from the received packets.
* This ensures that long-distance/poor antenna devices are still fully supported
    by the mesh



<hr/>

<!-- Footnotes -->
[^1]: https://meshtastic.org/blog/why-your-mesh-should-switch-from-longfast/