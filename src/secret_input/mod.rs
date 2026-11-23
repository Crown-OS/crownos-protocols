//! `crownos_secret_input_v1`: the compositor types a stored secret for crownos-secretsd.

/// Version 1 of the protocol.
pub mod v1 {
    crownos_protocol!("./protocols/crownos-secret-input-v1.xml");
}
