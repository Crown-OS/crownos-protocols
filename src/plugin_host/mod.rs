//! `crownos_plugin_host_v1`: crownplugind's privileged link to the
//! compositor — actions, window and workspace events, plugin keybindings and
//! the lock-screen scene.

/// Version 1 of the protocol.
pub mod v1 {
    crownos_protocol!("./protocols/crownos-plugin-host-v1.xml");
}
