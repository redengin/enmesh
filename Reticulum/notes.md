## Announce Mechanism
Transport Node will forward per
- if `announce` has already been received before, ignore it
    - else record the `announce` and number of retransmissions before reception
- if `announce` has been retransmitted 'm+1' times, do not forwared (default m = 128)
- after a randomized delay, `announce` will be retransmitted on all interfaces w/
    bandwidth available for announcements (default bandwidth allocation 2%)
- for interfaces without available bandwidth, the `announce` will be placed in
    a priority queue (priority inversely proportional to hop count)
    - when bandwidth becomes available, the highest priority `announce`s will
        be forwarded
- after the `announce` has been re-transmitted and no other nodes are heard
    retransmitting the same `announce` with a greater hop count than the hop
    count recorded by this node, the forwarding will be retried r times
    (default r = 1)
- if a newer `announce` from the same destination arrives and matches the current
    it will be discarded
    - if the newer `announce` contains new data, it will replace the current
        `announce`

## Routing (Reaching the Destination)
packet level transport
* single destination
    - packet is encrypted with an ephemeral encryption key using the public key of the destination
        - the public part of the ephemeral key is included with the encrypted packet
    - destinations can prove receipt by calculating the SHA-256 hash of the packet data
        and signing this hash with it's Ed25519 signing key - allowing the network to
        authenticate the reception using the destinations public signing key.

* group destination
    - packet is encrypted with the pre-shared AES-256 key of the destination.

* link transport
    1. client node will send a `link request` toward the destination
        - along the way, Transport Nodes will memo the `link request` as **pending**
    2. if the destination accepts the `link request` it will send back an
        authenticated ACK.
        - all Transport Nodes on the path seeing an authentic ACK will memo the
            `link request` as **established**

* Resources (large data)
leverage `link transport` with support for compression, fragmentation, and sequencing.

## Zones
Transport Nodes broadcasting an `interface discovery` can optionally sign the `announce`
with a `Network Identity`
- allows authentication of the *network* nodes - proof that the node has the private key
    for that `Network Identity`
This allows users to configure `Trust Boundaries` to *whitelist* zones of trust.

[Announcment Propogation Rules](https://reticulum.network/manual/Reticulum%20Manual.pdf#page=90)

## Cryptographic Primitives
* Ed25519 for signatures
* X25519 for ECDH key exchanges
* HKDF for key derivation
* Encrypted tokens are based on the Fernet spec
    – Ephemeral keys derived from an ECDH key exchange on Curve25519
    – AES-256 in CBC mode with PKCS7 padding
    – HMAC using SHA256 for message authentication
    – IVs must be generated through os.urandom() or better
    – No Fernet version and timestamp metadata fields
* SHA-256
* SHA-512

## Rate Control
* The `announce_rate_target` option sets the minimum amount of time, in seconds, that should pass between
    received announces, for any one destination. As an example, setting this value to 3600 means that announces
    received on this interface will only be re-transmitted and propagated to other interfaces once every hour, no
    matter how often they are received.
* The optional `announce_rate_grace` defines the number of times a destination can violate the announce rate
    before the target rate is enforced.
* The optional `announce_rate_penalty` configures an extra amount of time that is added to the normal rate
    target. As an example, if a penalty of 7200 seconds is defined, once the rate target is enforced, the destination
    in question will only have its announces propagated every 3 hours, until it lowers its actual announce rate to
    within the target


Chat with Gemini
* prompt - I need to design mesh lora to be robust to dense nodes and malicious nodes.

- Reputation-Based Trust Routing (Beta-Reputation System)
    * Every node monitors its neighbors continuously. When Node A forwards a packet to Node B, Node A listens to the channel
        to verify that Node B actually rebroadcasts it to the next hop (passive acknowledgement).
    * Maintain a local tracking table using a Beta distribution model
    * If a node's trust metric falls below a set threshold (e.g., $T < 0.7$), it is dynamically blacklisted, and routing tables are recalculated to bypass it entirely.

```math
\alpha = \text{Successful Forwards} + 1
```
```math
\beta = \text{Dropped / Tampered Packets} + 1
```
```math
    Trust Metric (T):
        T = \frac{\alpha}{\alpha + \beta}
```
- Multipath Routing with Erasure Coding
    To neutralize selective forwarding or blackhole nodes without the
    delay of waiting for reputation updates, implement multi-path routing:

    Split data packets using a lightweight erasure coding scheme
    (like Reed-Solomon or Fountain codes). For instance, break a message
    into 4 shards where any 2 shards can rebuild the original.

    Route the shards along completely independent geographic trajectories
    toward the sink/gateway. A malicious node dropping a single shard will
    fail to disrupt the data stream.

