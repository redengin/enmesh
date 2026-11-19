What is Enmesh?
================================================================================
Enmesh is an all Rust implementation for Mesh LoRa.

### What is Mesh LoRa?
Current LoRa mesh implementations ([Meshtastic][1], [MeshCore][2]) using a
single LoRa Channel are susceptible to denial-of-service by congestion of the
LoRa Channel. Congestion can occur by **density** (many nearby particpants) or
**malicisouly** (a node overly transmits to deny service locally).

Mesh LoRa mitigates the congestion of LoRa channels by constantly evolving LoRa
channels to support access by the mesh participants.

Mesh LoRa's only concern is propagation of data, supporting all mesh content
semantics ([Meshtastic][1], [MeshCore][2], and others).


#### Mesh LoRa is being designed per our [RFC Process](https://github.com/redengin/enmesh/wiki/RFC-Process).
* [RFC Mesh LoRa Anonymity](https://github.com/redengin/enmesh/wiki/RFC-%E2%80%90-Mesh-LoRa-Anonymity)
* [RFC Mesh LoRa Network Protocol](https://github.com/redengin/enmesh/wiki/RFC-%E2%80%90-Mesh-LoRa-Network-Protocol)
* [RFC Mesh LoRa RF Protocol](https://github.com/redengin/enmesh/wiki/RFC-%E2%80%90-Mesh-LoRa-RF-Protocol)

#### Visualiztion of Mesh LoRa - https://redengin.github.io/enmesh/
The visualization was produced per the RFCs to demonstrate how the
Mesh LoRa design evolves `shards` toward universal mesh communication.

### What does Enmesh firmware look like?
You can run a demonstration - [see enmesh demo](./firmware/enmesh/README.md).

### Suported Boards ([see flashing instructions](./firmware/boards/README.md))
<table>
<thead>
<tr> <th>Board</th> <th>LoRa</th> <th>BLE</th> <th>WiFi</th> <th>GPS</th> <th>Sensors</th> </tr>
</thead>
<tbody>
<tr>
  <td><a href="https://heltec.org/project/wifi-lora-32-v3/">Heltec WiFi LoRa 32 v3</a> </td>
  <td> <span style="color:green">In-Work</span> </td>
  <td> <span style="color:green">Tested</span> </td>
  <td> <span style="color:green">In-Work</span> </td>
  <td> <span style="color:yellow">TODO</span> </td>
  <td> <span style="color:yellow">TODO</span> </td>
</tr>
<tr>
  <td><a href="https://heltec.org/project/wifi-lora-32-v4/">Heltec WiFi LoRa 32 v4</a> </td>
  <td> <span style="color:green">In-Work</span> </td>
  <td> <span style="color:green">Tested</span> </td>
  <td> <span style="color:green">In-Work</span> </td>
  <td> <span style="color:yellow">TODO</span> </td>
  <td> <span style="color:yellow">TODO</span> </td>
</tr>
<tr>
  <td><a href="https://heltec.org/project/wireless-stick-lite-v2/">Heltec Wireless Stick v2</a> </td>
  <td> <span style="color:green">In-Work</span> </td>
  <td> <span style="color:blue">Not Tested</span> </td>
  <td> <span style="color:blue">Not Tested</span> </td>
  <td> <span style="color:yellow">TODO</span> </td>
  <td> <span style="color:yellow">TODO</span> </td>
</tr>
<tr>
  <td><a href="https://heltec.org/project/wireless-stick-lite-v3/">Heltec Wireless Stick v3</a> </td>
  <td> <span style="color:red">Incomplete</span> </td>
  <td> <span style="color:blue">Not Tested</span> </td>
  <td> <span style="color:green">In-Work</span> </td>
  <td> <span style="color:yellow">TODO</span> </td>
  <td> <span style="color:yellow">TODO</span> </td>
</tr>
<tr>
  <td><a href="https://heltec.org/project/wireless-stick-lite-v3/">Heltec Wireless Stick v3</a> </td>
  <td> <span style="color:green">In-Work</span> </td>
  <td> <span style="color:blue">Not Tested</span> </td>
  <td> <span style="color:green">In-Work</span> </td>
  <td> <span style="color:yellow">TODO</span> </td>
  <td> <span style="color:yellow">TODO</span> </td>
</tr>
<tr>
  <td><a href="https://heltec.org/project/wireless-tracker/">Heltec Wireless Tracker</a> </td>
  <td> <span style="color:green">In-Work</span> </td>
  <td> <span style="color:blue">Not Tested</span> </td>
  <td> <span style="color:green">In-Work</span> </td>
  <td> <span style="color:yellow">TODO</span> </td>
  <td> <span style="color:yellow">TODO</span> </td>
</tr>
<tr>
  <td><a href="https://heltec.org/project/wireless-paper/">Heltec Wireless Paper</a> </td>
  <td> <span style="color:green">In-Work</span> </td>
  <td> <span style="color:green">Tested</span> </td>
  <td> <span style="color:green">In-Work</span> </td>
  <td> <span style="color:yellow">TODO</span> </td>
  <td> <span style="color:yellow">TODO</span> </td>
</tr>
</tbody>
</table>

The architecture requires a minimal BSP layer to be implemeneted to support
other platforms (nrf52 and others).


### Support for WiFi Bridges
EnMesh firmware supports [MQTT](https://en.wikipedia.org/wiki/MQTT) as a bridge
over [WiFi](https://en.wikipedia.org/wiki/Wi-Fi) - supporting services like
[letsmesh.net](LetsMesh.net).

Repository Overview
================================================================================
* firmware - board support for common hardware
  - boards - flashable implementations of Enmesh
  - soc/* - Rust HALs for LoRa platforms
  - common - shared Rust cargo used by Enmesh
  - enmesh - board agnostic implementation of Enmesh
* [MeshTastic Library](MeshCore) - Rust implementation of MeshCore
  * IN-WORK: MeshCore rapidly evolves, this should become a separate repo
* [MeshCore Library](MeshCore) - Rust implementation of MeshCore
  * IN-WORK: Meshtastic rapidly evolves, this should become a separate repo

Enmesh Physical Architecture
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
```

Enmesh BLE is implemented to support mobile apps that use Meshtastic or
MeshCore defined BLE services. To further the usage of Enmesh, an
Enmesh Mobile App will be created to support further control per the
Enmesh design using an Enmesh BLE service.


<!-- REFERENCES -->
[1]: https://meshtastic.org/
[2]: https://meshcore.io