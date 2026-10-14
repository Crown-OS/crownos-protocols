//! `crownos_session_v1`: the per-user session agent's link to the compositor
//! — configuration in, programs to start out.

/// Version 1 of the protocol.
pub mod v1 {
    crownos_protocol!("./protocols/crownos-session-v1.xml");
}
