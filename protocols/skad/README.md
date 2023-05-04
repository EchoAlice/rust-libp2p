# Secure Kademlia Library
### Summary
DAS Networking in Ethereum requires lots of small blob samples to be placed in a distributed network where arbitrary nodes can query "the network" to make sure these samples are available.

What this "network" actually looks like is still an open question, but there seems to be high level consensus on its process and design:

1. **Initial Dissemination** - Builder sends rows and columns to validators via gossipsub-like protocol (unstructured network)
2. **Continued Dissemination** - Validators distribute samples to rest of network via Kademlia-style DHT(structured network)
3. **Querying DHT for samples** - Sampling nodes ask DHT for individual samples
4. **Reconstruction** - If there's a liveness failure within the network and there are enough individual samples held amongst participants, participants send samples to at least **one honest validator** (or full node?) to reconstruct (and begin redistribution?) of the blob.

### Motivation
This structured network, where anyone can participate, has a lot of nice properties for distributing small chunks of data across an ever-changing set of data and peers, but this thing comes with major drawbacks.

Most notably, anyone can spin up a bunch of node ids and have a disproportionately large amount of control over data and data flow within the network.  These are *sybil attacks*. 

A secure kademlia overlay can help mitigate adversaries wishing to cause harm and seems to important piece of the puzzle to making DAS Networking happen.

Within this crate, I modify libp2p's rust implementation of the kademlia protocol to create a "hardened version" of our Kademlia DHT.

### Key Modifications
- Parallel, disjoint-path key-value lookups
- Validator-only peer logic for routing table
  **I might stick with an easier proof of work requirement in initial creation of the library (if it's easier) to get the interface down.
- Sibling broadcast list

 Inspired by Dankrad's [S/Kademlia document](https://notes.ethereum.org/@dankrad/S-Kademlia-DAS).

&nbsp;
&nbsp;
## High Level Roadmap
### 1. Fork Libp2p's implementation of a Kademlia library.
Questions:
- Does it matter that Libp2p's DHT relies on libp2p streams? Can that work over UDP?

Resources:
- Libp2p's [high level docs](https://curriculum.pl-launchpad.io/curriculum/libp2p/dht/)
- Libp2p's K-DHT [specs](https://github.com/libp2p/specs/blob/master/kad-dht/README.md)
- Libp2p's Rust K-DHT [implementation](https://github.com/libp2p/rust-libp2p/tree/master/protocols/kad) 

### 2. Implement [S/Kad Modifications](https://hackmd.io/fG81lFVHSXiw3UjjyjKC6A#Key-Modifications) within fork

Expose same functionality as Kademlia:
- Ping
- Store
- FindNode
- FindValue
    
### 3. Write tests
There are already lots of tests within libp2p's repo.  Great!

### 4. Integrate with discv5 (via overlay protocol)
Figure out how this library could plug into [overlay protocol](https://github.com/ethereum/portal-network-specs/blob/master/portal-wire-protocol.md) routing tables.

&nbsp;
&nbsp;
## Secure Kademlia DHT Overlay Network
Our secure routing table needs to follow specialized rules. **Where and how** should these rules be implemented?

I believe these questions fall under the umbrella of the Portal Network's [Overlay Routing Table](https://github.com/ethereum/portal-network-specs/blob/796d3c5772e845b98a6191465a695be7f5324b65/implementation-details-overlay.md#d---overlay-routing-table) functionality.

#### Questions:
- How do sampling nodes within a validator-only routing table ask for queries?


### Resources:
Discv5 Overlay:
- [Portal Network Specs](https://github.com/ethereum/portal-network-specs)
- Portal Network's [Overlay Network Functionality](https://github.com/ethereum/portal-network-specs/blob/796d3c5772e845b98a6191465a695be7f5324b65/implementation-details-overlay.md
)
- [TalkReq/TalkResp](https://github.com/ethereum/devp2p/issues/156) convo between Piper and Felix describing discv5 protocol being leveraged for overlay (or sub) protocols