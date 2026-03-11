Secure distributed communications enhanced by anonymity of LoRa
================================================================================
```mermaid
C4Component

    Container_Boundary(lora, "LoRa") {
      Component(repeater, "repeater")
      Component(wifi_repeater, "wifi repeater")
      Component(companion, "companion")
    }

    BiRel(companion, repeater, "send/receive LoRa packets")
    BiRel(companion, wifi_repeater, "send receive LoRa packets")
    BiRel(wifi_repeater, wifi_router, "bridges local LoRa to internet")
    BiRel(wifi_router, enmesh_endpoint, "bridges local LoRa to internet")

    Container_Boundary(internet, "Internet") {
      Component(wifi_router, "wifi router")
      Component(enmesh_endpoint, "endmesh endpoint")
    }

    Container_Boundary(mobile_app, "Mobile App") {
      Component(mobile_app, "Mobile App")
    }
    BiRel(mobile_app, companion, "send/receive LoRa packets")
    BiRel(mobile_app, enmesh_endpoint, "bridges local LoRa to internet")

  UpdateLayoutConfig($c4ShapeInRow="2", $c4BoundaryInRow="2")

```

Background
--------------------------------------------------------------------------------
[LoRa](https://en.wikipedia.org/wiki/LoRa) use has proliferated under
[Meshtastic](https://meshtastic.org/) and [MeshCore](https://meshcore.co.uk/)
creating LoRa hardware that is readily purchasable by users.

[Reticulum](https://reticulum.network/) proposes that the privacy/anonymity of
LoRa can be extended beyond the reach of LoRa.

Connecting LoRa to everything
--------------------------------------------------------------------------------
Connecting LoRa node a WiFi router, extends the reach of a LoRa node to the
world. Some readily purchasable mesh hardware supports WiFi.

Only a few LoRa nodes need to support an IP bridge for this to work.

The LoRa traffic will be encapsulated in an IP packet (and vice-versa).

<!-- The IP endpoint(s) are determined by a query for enmesh bridge nodes. -->


What you'll find here
================================================================================
* Bridge Node Implementation - internet service to bridge LoRa traffic
* LoRa Node Implementation - supports local LoRa traffic
    * Meshes
        * Meshtastic
        * MeshCore
        * enmesh
    * WiFi bridge (per hardware support)
* Mobile Application - provides enhanced support beyond Meshtastic/MeshCore




