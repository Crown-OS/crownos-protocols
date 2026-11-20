//! `crownos_output_info_v1`: how the compositor treats each output: its
//! settings name, the shell surfaces it shows, whether it is primary or
//! focused, what its display can do and its scanout depth.

/// Version 1 of the protocol.
pub mod v1 {
    crownos_protocol!("./protocols/crownos-output-info-v1.xml");
}
