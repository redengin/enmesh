# Mesh LoRa simulation

A dependency-free, browser-based visualization of LoRa channel congestion and
local shard formation. It is an illustrative model based on the Mesh LoRa RFCs,
not a prediction of real radio performance.

The topology view projects a synthetic map into 3D. Every channel occupies a
separate, color-coded plane, with the primary channel above the shard layers.
Nodes on shard channels rise from their ground position to their channel plane,
and shard links and broadcast ripples use their channel color. Drag an empty
part of the map horizontally to orbit and vertically to tilt; use the mouse
wheel to zoom around the cursor. Reset View restores the default camera.
Select a node to inspect its range.
During each round, animated channel-colored paths show a broadcast reaching
in-range peers on the same channel; failed attempts are dashed red. The model
does not simulate relays between different channel layers.

## Run

From this directory, start a local static server:

```sh
python3 -m http.server 8000
```

Open [http://localhost:8000](http://localhost:8000). Use the controls to change
the node count and traffic intensity, start or pause the simulation, advance a
single round, or reset the scenario. Select a node in the topology to inspect
its transmit range and reachable peers.

The app uses browser-native JavaScript modules and has no package installation
or build step.

## Model

- Nodes are placed in local clusters of up to eight. Clusters are spatially
  separated, but all nodes within a cluster begin on the primary channel.
- Map coordinates, contour lines, and channel elevations are visual aids, not
  geographic data or physical altitude.
- Every node has at least one peer within its transmit range. The minimum
  transmit range for each node is set from its nearest peer and is maintained
  while the node adapts its power.
- A channel has an illustrative capacity of four packet attempts per round.
  Packet success is sampled from the ratio of capacity to attempts.
- After three consecutive heavily congested rounds, a node advertises a local
  shard. Nodes move to an underused channel one at a time as capacity allows.
- Sharded nodes reduce their modeled range by five units every three rounds,
  stopping at the nearest-peer limit.
- A fixed seed makes each reset reproducible. These rules are intentionally
  simplified; the UI's range units and airtime capacity are not physical LoRa
  measurements.

The model does not simulate radio propagation, regulatory bands, actual LoRa
time-on-air, interference across real terrain, cryptography, or the full
protocol state machine. Channel links and node positions are visualization
concepts only.

## Tests

With the static server running, open
[http://localhost:8000/tests.html](http://localhost:8000/tests.html). The
browser test page checks:

- Reachability for layouts from 2 through 48 nodes.
- Reproducible state for identical seeds.
- Shard advertisement and node migration under sustained load.
- Peer reachability throughout shard and transmit-range adaptation.

If Node.js is available, the same checks can run from this directory without
installing any packages:

```sh
node --experimental-default-type=module --test tests/model.test.mjs
```

## RFC sources

- [Mesh LoRa overview](https://github.com/redengin/enmesh/wiki/RFC-%E2%80%90-Mesh-LoRa)
- [Mesh LoRa Network Protocol](https://github.com/redengin/enmesh/wiki/RFC-%E2%80%90-Mesh-LoRa-Network-Protocol)
- [Mesh LoRa RF Protocol](https://github.com/redengin/enmesh/wiki/RFC-%E2%80%90-Mesh-LoRa-RF-Protocol)
- [Mesh LoRa Anonymity](https://github.com/redengin/enmesh/wiki/RFC-%E2%80%90-Mesh-LoRa-Anonymity)

The initial scenario focuses on congestion, shard advertisements, and range
limiting from the RF protocol. It does not expose path or signal-strength
metadata as packet fields.
