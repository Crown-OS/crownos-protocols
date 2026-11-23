//! `crownos_context_v1`: what the user is doing, without content, for the visual context engine.

/// Version 1 of the protocol. Its interfaces are at version 2, which adds
/// many bound clients, the `scope` and `revoked` events and per-client pause.
pub mod v1 {
    crownos_protocol!("./protocols/crownos-context-v1.xml");
}
