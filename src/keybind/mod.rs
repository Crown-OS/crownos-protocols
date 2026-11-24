//! `crownos_keybind_v1`: global shortcuts that applications register.

/// Version 1 of the protocol.
pub mod v1 {
    crownos_protocol!("./protocols/crownos-keybind-v1.xml");
}
