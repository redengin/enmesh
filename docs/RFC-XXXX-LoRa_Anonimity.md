LoRa Mesh Anonimity
================================================================================

#### Status: Proposed <!--Proposed / Accepted / Implemented / Obsolete -->
<details>
    <summary>Status Details</summary>

* Author(s)
    * Stephen Holstein 
* Sponsors(s)
    * None
* Obsoletes
    * None
</details>

Objective
================================================================================
Increase LoRa user anonymity by obfuscating their location.

Motivation
================================================================================
As mesh protocols evolve, anonymity of the sender should be paramount.
Geo-locating a sender should be as difficult as possible.

Current LoRa mesh protocols make it simple for a monitor at any place on the
mesh to geo-locate the sender.

User Benefit
================================================================================
Allow users to publish sensitive information
* whistle-blowers - publish illegal activities
* activists - publish information toward social change

Design Proposal
================================================================================
Provide LoRa mesh users [onion routing](https://en.wikipedia.org/wiki/Onion_routing).

LoRa Mesh Onion Routing
--------------------------------------------------------------------------------
1. sender queries for a circuit to the destination
2. sender envelopes their message by encrypting per the circuit sequence
3. sender sends the enveloped message via the circuit




