//! `crownos_app_icons`: where a launcher shows an application's icon, so the
//! compositor can grow the application's windows out of it and shrink them
//! back into it.

/// Version 1 of the protocol.
pub mod v1 {
    crownos_protocol!("./protocols/crownos-app-icons-v1.xml");
}
