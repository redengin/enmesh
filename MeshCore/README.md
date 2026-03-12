MeshCore Implemented in Rust
================================================================================
MeshCore design quickly evolving. As I don't have the time to maintain this
implementation, I hope that either MeshCore devs or the community create their
own fork.

Reverse-Engineering MeshCore
================================================================================
The main development on MeshCore is on the [C/C++ implementation](https://github.com/meshcore-dev/MeshCore).

The C/C++ implementation of MeshCore lacks architecture and therefore implements
many features as side-effects.

This Rust implementation provides a software architecture that makes each feature
a real concern - rather than a side-effect.

